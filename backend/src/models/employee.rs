use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Employee {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub department: String,
    pub salary: String,

    pub mobile_number: String,
    pub address: String,
    pub username: String,
    pub password: String,
    pub id_proof_1: String,
    pub id_proof_2: String,
    pub profile_image: String,
}

#[derive(Deserialize)]
pub struct CreateEmployeeRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub department: String,
    pub salary: f64,

    pub mobile_number: String,
    pub address: String,
    pub username: String,
    pub password: String,
    pub id_proof_1: String,
    pub id_proof_2: String,
    pub profile_image: String,
}

#[derive(Deserialize)]
pub struct UpdateEmployeeRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub department: String,
    pub salary: f64,

pub mobile_number: Option<String>,
pub address: Option<String>,
pub password: Option<String>,
pub id_proof_1: Option<String>,
pub id_proof_2: Option<String>,
pub profile_image: Option<String>,
}