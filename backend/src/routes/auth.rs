use actix_web::web;
use crate::handlers::auth_handlers::login;

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
}