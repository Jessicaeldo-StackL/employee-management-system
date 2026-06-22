use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use base64::Engine;
use sqlx::{PgPool, Row};
use std::path::PathBuf;
use uuid::Uuid;

use crate::models::employee::{CreateEmployeeRequest, Employee, UpdateEmployeeRequest};
use crate::utils::password::hash_password;

const ALLOWED_PDF_MIME: &[&str] = &["application/pdf"];
const ALLOWED_IMAGE_MIME: &[&str] = &["image/jpeg", "image/png", "image/webp", "image/gif"];
const MAX_FILE_SIZE_BYTES: usize = 5 * 1024 * 1024;

fn uploads_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("uploads")
}


fn extension_from_mime(mime: &str) -> &'static str {
    match mime {
        "application/pdf" => "pdf",
        "image/jpeg" => "jpg",
        "image/png" => "png",
        "image/webp" => "webp",
        "image/gif" => "gif",
        _ => "bin",
    }
}

fn save_data_url(data: &str, prefix: &str, allowed_mimes: &[&str]) -> Result<String, String> {
    if data.starts_with("/uploads/") {
        return Ok(data.to_string());
    }
    if !data.starts_with("data:") {
        return Ok(data.to_string());
    }
    let comma_index = data.find(',').ok_or("Invalid data URL format")?;
    let header = &data[..comma_index];
    let payload = &data[comma_index + 1..];
    if !header.contains(";base64") {
        return Err("Only base64 encoded files are supported".to_string());
    }
    let mime = header.strip_prefix("data:").and_then(|h| h.split(';').next()).unwrap_or("application/octet-stream");
    if !allowed_mimes.contains(&mime) {
        return Err(format!("File type '{}' not allowed. Allowed: {}", mime, allowed_mimes.join(", ")));
    }
    let bytes = base64::engine::general_purpose::STANDARD.decode(payload).map_err(|e| format!("Base64 decode error: {e}"))?;
    if bytes.len() > MAX_FILE_SIZE_BYTES {
        return Err(format!("File exceeds maximum size of {}MB", MAX_FILE_SIZE_BYTES / 1024 / 1024));
    }
    let ext = extension_from_mime(mime);
    let file_name = format!("{}_{}.{}", prefix, Uuid::new_v4(), ext);
    let absolute_path = uploads_dir().join(&file_name);
    std::fs::write(&absolute_path, bytes).map_err(|e| format!("Failed to save file: {e}"))?;
    Ok(format!("/uploads/{file_name}"))
}

fn row_to_employee(row: &sqlx::postgres::PgRow) -> Employee {
    Employee {
        id: row.get("id"),
        first_name: row.get("first_name"),
        last_name: row.get("last_name"),
        email: row.get("email"),
        department: row.get("department"),
        salary: row.get("salary"),
        mobile_number: row.get("mobile_number"),
        address: row.get("address"),
        username: row.get("username"),
        password: String::new(),
        id_proof_1: row.get("id_proof_1"),
        id_proof_2: row.get("id_proof_2"),
        profile_image: row.get("profile_image"),
    }
}

#[get("/employees")]
pub async fn get_employees(
    pool: web::Data<PgPool>
) -> impl Responder {
    let rows = sqlx::query(
        "SELECT id,first_name,last_name,email,department,salary::TEXT AS salary,mobile_number,address,username,id_proof_1,id_proof_2,profile_image FROM employees ORDER BY id"
    ).fetch_all(pool.get_ref()).await;
    match rows {
        Ok(records) => HttpResponse::Ok().json(records.iter().map(row_to_employee).collect::<Vec<_>>()),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": format!("Database error: {e}")})),
    }
}

#[get("/employees/{id}")]
pub async fn get_employee(pool: web::Data<PgPool>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let result = sqlx::query(
        "SELECT id,first_name,last_name,email,department,salary::TEXT AS salary,mobile_number,address,username,id_proof_1,id_proof_2,profile_image FROM employees WHERE id=$1"
    ).bind(id).fetch_optional(pool.get_ref()).await;
    match result {
        Ok(Some(row)) => HttpResponse::Ok().json(row_to_employee(&row)),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({"error": "Employee not found"})),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": format!("Database error: {e}")})),
    }
}

#[post("/employees")]
pub async fn add_employee(pool: web::Data<PgPool>, employee: web::Json<CreateEmployeeRequest>) -> impl Responder {
    if employee.first_name.trim().is_empty() || employee.last_name.trim().is_empty() || employee.email.trim().is_empty() || employee.username.trim().is_empty() || employee.password.trim().is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({"error": "All required fields must be filled"}));
    }
    let id_proof_1 = match save_data_url(&employee.id_proof_1, "idproof1", ALLOWED_PDF_MIME) {
        Ok(v) => v, Err(e) => return HttpResponse::BadRequest().json(serde_json::json!({"error": e})),
    };
    let id_proof_2 = match save_data_url(&employee.id_proof_2, "idproof2", ALLOWED_PDF_MIME) {
        Ok(v) => v, Err(e) => return HttpResponse::BadRequest().json(serde_json::json!({"error": e})),
    };
    let profile_image = match save_data_url(&employee.profile_image, "profile", ALLOWED_IMAGE_MIME) {
        Ok(v) => v, Err(e) => return HttpResponse::BadRequest().json(serde_json::json!({"error": e})),
    };
    let hashed_password = match hash_password(&employee.password) {
        Ok(h) => h, Err(e) => return HttpResponse::InternalServerError().json(serde_json::json!({"error": e})),
    };
    let result = sqlx::query(
        "INSERT INTO employees(first_name,last_name,email,department,salary,mobile_number,address,username,password,id_proof_1,id_proof_2,profile_image) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12)"
    ).bind(&employee.first_name).bind(&employee.last_name).bind(&employee.email).bind(&employee.department).bind(employee.salary)
     .bind(&employee.mobile_number).bind(&employee.address).bind(&employee.username).bind(&hashed_password).bind(id_proof_1).bind(id_proof_2).bind(profile_image)
     .execute(pool.get_ref()).await;
    match result {
        Ok(_) => {
            let _ = sqlx::query("INSERT INTO users(username,password,role) VALUES($1,$2,$3) ON CONFLICT(username) DO NOTHING")
                .bind(&employee.username).bind(&hashed_password).bind("employee").execute(pool.get_ref()).await;
            HttpResponse::Ok().json(serde_json::json!({"message": "Employee added successfully"}))
        },
        Err(e) if e.to_string().contains("unique") || e.to_string().contains("duplicate") =>
            HttpResponse::Conflict().json(serde_json::json!({"error": "Username or email already exists"})),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": format!("Database error: {e}")})),
    }
}

#[put("/employees/{id}")]
pub async fn update_employee(pool: web::Data<PgPool>, path: web::Path<i32>, employee: web::Json<UpdateEmployeeRequest>) -> impl Responder{
    let id = path.into_inner();
    let id_proof_1 = match &employee.id_proof_1 {
        Some(v) => match save_data_url(v, "idproof1", ALLOWED_PDF_MIME) { Ok(s) => Some(s), Err(e) => return HttpResponse::BadRequest().json(serde_json::json!({"error": e})) },
        None => None,
    };
    let id_proof_2 = match &employee.id_proof_2 {
        Some(v) => match save_data_url(v, "idproof2", ALLOWED_PDF_MIME) { Ok(s) => Some(s), Err(e) => return HttpResponse::BadRequest().json(serde_json::json!({"error": e})) },
        None => None,
    };
    let profile_image = match &employee.profile_image {
        Some(v) => match save_data_url(v, "profile", ALLOWED_IMAGE_MIME) { Ok(s) => Some(s), Err(e) => return HttpResponse::BadRequest().json(serde_json::json!({"error": e})) },
        None => None,
    };
    let hashed_password = match &employee.password {
        Some(p) if !p.is_empty() => match hash_password(p) { Ok(h) => Some(h), Err(e) => return HttpResponse::InternalServerError().json(serde_json::json!({"error": e})) },
        _ => None,
    };
    let result = sqlx::query(
        "UPDATE employees SET first_name=$1,last_name=$2,email=$3,department=$4,salary=$5,mobile_number=$6,address=$7,password=COALESCE($8,password),id_proof_1=COALESCE(NULLIF($9,''),id_proof_1),id_proof_2=COALESCE(NULLIF($10,''),id_proof_2),profile_image=COALESCE(NULLIF($11,''),profile_image) WHERE id=$12"
    ).bind(&employee.first_name).bind(&employee.last_name).bind(&employee.email).bind(&employee.department).bind(employee.salary)
     .bind(&employee.mobile_number).bind(&employee.address).bind(hashed_password.as_deref()).bind(id_proof_1.as_deref()).bind(id_proof_2.as_deref()).bind(profile_image.as_deref()).bind(id)
     .execute(pool.get_ref()).await;
    match result {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({"message": "Employee updated successfully"})),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": format!("Database error: {e}")})),
    }
}

#[delete("/employees/{id}")]
pub async fn delete_employee(pool: web::Data<PgPool>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    if let Ok(Some(row)) = sqlx::query("SELECT username FROM employees WHERE id=$1").bind(id).fetch_optional(pool.get_ref()).await {
        let username: String = row.get("username");
        let _ = sqlx::query("DELETE FROM users WHERE username=$1").bind(&username).execute(pool.get_ref()).await;
    }
    let result = sqlx::query("DELETE FROM employees WHERE id=$1").bind(id).execute(pool.get_ref()).await;
    match result {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({"message": "Employee deleted successfully"})),
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({"error": format!("Database error: {e}")})),
    }
}

#[get("/employee-profile/{username}")]
pub async fn employee_profile(
    pool: web::Data<PgPool>,
    path: web::Path<String>,
) -> impl Responder {

    let requested_username = path.into_inner();

    let result = sqlx::query(
        "SELECT id,first_name,last_name,email,department,salary::TEXT AS salary,mobile_number,address,username,id_proof_1,id_proof_2,profile_image FROM employees WHERE username=$1"
    )
    .bind(&requested_username)
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(row) => HttpResponse::Ok().json(row_to_employee(&row)),
        Err(e) => HttpResponse::InternalServerError().json(
            serde_json::json!({
                "error": format!("Database error: {e}")
            })
        ),
    }
}
