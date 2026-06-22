mod handlers;
mod config;
mod db;
mod middleware;
mod models;
mod routes;
mod utils;

use crate::db::database::create_pool;
use crate::routes::auth::auth_routes;
use crate::routes::employee::employee_routes;
use crate::routes::leave;
use crate::middleware::auth::JwtAuth;

use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_web::{
    error::{ErrorBadRequest, ErrorNotFound},
    get,
    http::header::{ContentDisposition, DispositionParam, DispositionType},
    middleware::Logger,
    web,
    App,
    HttpServer,
    Responder,
};
use serde::Deserialize;
use std::path::PathBuf;

fn uploads_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("uploads")
}

#[get("/")]
async fn home() -> impl Responder {
    "Employee Management System API"
}

#[derive(Debug, Deserialize)]
struct FilePathQuery {
    path: String,
}

fn resolve_upload_file_path(path: &str) -> Option<(PathBuf, String)> {
    let normalized = path.trim();
    let relative = normalized
        .strip_prefix("/uploads/")
        .or_else(|| normalized.strip_prefix("uploads/"))?;

    if relative.is_empty() || relative.contains('/') || relative.contains('\\') || relative.contains("..") {
        return None;
    }

    let file_path = uploads_dir().join(relative);
    Some((file_path, relative.to_string()))
}

#[get("/view")]
async fn view_uploaded_file(query: web::Query<FilePathQuery>) -> actix_web::Result<NamedFile> {
    let (file_path, file_name) = resolve_upload_file_path(&query.path)
        .ok_or_else(|| ErrorBadRequest("Invalid file path"))?;

    let mime = mime_guess::from_path(&file_name).first_or_octet_stream();
    let file = NamedFile::open_async(file_path)
        .await
        .map_err(|_| ErrorNotFound("File not found"))?
        .set_content_type(mime)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Inline,
            parameters: vec![],
        });

    Ok(file)
}

#[get("/download")]
async fn download_uploaded_file(query: web::Query<FilePathQuery>) -> actix_web::Result<NamedFile> {
    let (file_path, file_name) = resolve_upload_file_path(&query.path)
        .ok_or_else(|| ErrorBadRequest("Invalid file path"))?;

    let mime = mime_guess::from_path(&file_name).first_or_octet_stream();
    let file = NamedFile::open_async(file_path)
        .await
        .map_err(|_| ErrorNotFound("File not found"))?
        .set_content_type(mime)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![DispositionParam::Filename(file_name)],
        });

    Ok(file)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    env_logger::init();

    let uploads_path = uploads_dir();
    std::fs::create_dir_all(&uploads_path)?;

    let pool = create_pool().await;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    let json_config = web::JsonConfig::default().limit(35 * 1024 * 1024);

    println!("Database Connected & Migrations Applied");
    println!("Server running on http://127.0.0.1:8081");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .app_data(web::Data::new(pool.clone()))
            .app_data(json_config.clone())
            .service(Files::new("/uploads", uploads_path.clone()))
            .service(
                web::scope("/files")
                    .wrap(JwtAuth::new())
                    .service(view_uploaded_file)
                    .service(download_uploaded_file)
            )
            .service(home)
            .configure(auth_routes)
            .configure(employee_routes)
            .configure(leave::configure)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}