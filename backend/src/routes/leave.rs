use actix_web::web;
use crate::handlers::leave_handlers::*;
use crate::middleware::auth::JwtAuth;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .wrap(JwtAuth::new())
            .route("/leave-requests", web::post().to(apply_leave))
            .route("/leave-requests/my", web::get().to(get_my_leaves))
            .route("/leave-requests", web::get().to(get_all_leaves))
            .route("/leave-requests/{id}/withdraw", web::put().to(withdraw_leave))
            .route("/leave-requests/{id}/approve", web::put().to(approve_leave))
            .route("/leave-requests/{id}/reject", web::put().to(reject_leave)),
    );
}
