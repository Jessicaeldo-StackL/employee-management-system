use actix_web::web;
use crate::handlers::employee_handlers::{
    get_employees,
    get_employee,
    add_employee,
    update_employee,
    delete_employee,
    employee_profile,
};

pub fn employee_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_employees)
        .service(get_employee)
        .service(add_employee)
        .service(update_employee)
        .service(delete_employee)
        .service(employee_profile);
}