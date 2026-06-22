use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// Leave request model representing a single leave request
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct LeaveRequest {
    pub id: i32,
    pub employee_id: i32,
    pub employee_username: String,
    pub leave_type: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub reason: String,
    pub status: String, // "Pending", "Approved", "Rejected", "Withdrawn"
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request body for applying a new leave request
#[derive(Debug, Deserialize)]
pub struct ApplyLeaveRequest {
    pub leave_type: String, // Sick Leave, Vacation, Personal Leave, Maternity Leave, Other
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub reason: String,
}

/// Response for leave request operations
#[derive(Debug, Serialize)]
pub struct LeaveResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<LeaveRequest>,
}

/// Response for list of leave requests
#[derive(Debug, Serialize)]
pub struct LeaveListResponse {
    pub success: bool,
    pub message: String,
    pub data: Vec<LeaveRequest>,
    pub count: usize,
}

