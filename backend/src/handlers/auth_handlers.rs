use actix_web::{post, web, HttpResponse, Responder};
use sqlx::{PgPool, Row};

use crate::models::auth::{LoginRequest, LoginResponse};
use crate::utils::jwt::generate_token;
use crate::utils::password::verify_password;

#[post("/login")]
pub async fn login(
    pool: web::Data<PgPool>,
    login_data: web::Json<LoginRequest>,
) -> impl Responder {
    if login_data.username.trim().is_empty()
        || login_data.password.trim().is_empty()
    {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "success": false,
            "message": "Username and password are required",
            "role": "",
            "username": "",
            "token": ""
        }));
    }

    // Query users table to get user with password
    let result = sqlx::query(
        "SELECT id, username, password, role FROM users WHERE username = $1"
    )
    .bind(&login_data.username)
    .fetch_optional(pool.get_ref())
    .await;

    match result {
        Ok(Some(user)) => {
            let user_id: i32 = user.get("id");
            let username: String = user.get("username");
            let stored_password: String = user.get("password");
            let role: String = user.get("role");

            // Verify password
            if !verify_password(&login_data.password, &stored_password) {
                println!("LOGIN FAILED: Password mismatch for user {}", username);
                return HttpResponse::Unauthorized().json(
                    serde_json::json!({
                        "success": false,
                        "message": "Invalid username or password",
                        "role": "",
                        "username": "",
                        "token": ""
                    }),
                );
            }

            // Generate JWT token
            match generate_token(&username, &role) {
                Ok((token, expiry_timestamp)) => {
                    // Convert timestamp back to DateTime for database storage
                    let expiry_datetime = chrono::DateTime::<chrono::Utc>::from_timestamp(expiry_timestamp, 0)
                        .expect("Failed to convert timestamp");
                    
                    // Save token to database
                    let update_result = sqlx::query(
                        "UPDATE users SET token = $1, token_expiry = $2 WHERE id = $3"
                    )
                    .bind(&token)
                    .bind(expiry_datetime)
                    .bind(user_id)
                    .execute(pool.get_ref())
                    .await;

                    match update_result {
                        Ok(_) => {
                            println!("LOGIN SUCCESS");
                            println!("Username: {}", username);
                            println!("Role: {}", role);
                            println!("Token saved to database");

                            HttpResponse::Ok().json(LoginResponse {
                                success: true,
                                message: "Login successful".to_string(),
                                role,
                                username,
                                token,
                            })
                        }
                        Err(e) => {
                            println!("DATABASE ERROR while saving token: {}", e);
                            HttpResponse::InternalServerError().json(
                                serde_json::json!({
                                    "success": false,
                                    "message": "Failed to save authentication token",
                                    "role": "",
                                    "username": "",
                                    "token": ""
                                }),
                            )
                        }
                    }
                }
                Err(e) => {
                    println!("TOKEN GENERATION ERROR: {}", e);
                    HttpResponse::InternalServerError().json(
                        serde_json::json!({
                            "success": false,
                            "message": "Failed to generate authentication token",
                            "role": "",
                            "username": "",
                            "token": ""
                        }),
                    )
                }
            }
        }

        Ok(None) => {
            println!("LOGIN FAILED: User not found: {}", login_data.username);
            HttpResponse::Unauthorized().json(
                serde_json::json!({
                    "success": false,
                    "message": "Invalid username or password",
                    "role": "",
                    "username": "",
                    "token": ""
                }),
            )
        }

        Err(e) => {
            println!("DATABASE ERROR: {}", e);

            HttpResponse::InternalServerError().json(
                serde_json::json!({
                    "success": false,
                    "message": format!("Database error: {}", e),
                    "role": "",
                    "username": "",
                    "token": ""
                }),
            )
        }
    }
}