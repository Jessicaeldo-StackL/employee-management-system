use actix_web::{HttpMessage, HttpRequest, HttpResponse, web};
use chrono::Utc;
use sqlx::PgPool;

use crate::models::leave::{
    ApplyLeaveRequest, LeaveListResponse, LeaveRequest, LeaveResponse,
};
use crate::utils::jwt::Claims;

fn claims_from_request(req: &HttpRequest) -> Result<Claims, HttpResponse> {
    req.extensions().get::<Claims>().cloned().ok_or_else(|| {
        HttpResponse::Unauthorized().json(LeaveResponse {
            success: false,
            message: "Unauthorized".to_string(),
            data: None,
        })
    })
}

/// Apply for a leave request (Employee)
pub async fn apply_leave(
    req: HttpRequest,
    db: web::Data<PgPool>,
    body: web::Json<ApplyLeaveRequest>,
) -> HttpResponse {
    let claims = match claims_from_request(&req) {
        Ok(c) => c,
        Err(resp) => return resp,
    };

    if claims.role != "employee" {
        return HttpResponse::Forbidden().json(LeaveResponse {
            success: false,
            message: "Only employees can apply for leave".to_string(),
            data: None,
        });
    }

    // Validate input
    if body.leave_type.trim().is_empty() {
        return HttpResponse::BadRequest().json(LeaveResponse {
            success: false,
            message: "Leave type is required".to_string(),
            data: None,
        });
    }

    if body.reason.trim().is_empty() {
        return HttpResponse::BadRequest().json(LeaveResponse {
            success: false,
            message: "Reason is required".to_string(),
            data: None,
        });
    }

    if body.end_date < body.start_date {
        return HttpResponse::BadRequest().json(LeaveResponse {
            success: false,
            message: "End date must be greater than or equal to start date".to_string(),
            data: None,
        });
    }

    // Get employee_id from employees table using username from JWT claims
    let employee = match sqlx::query_as::<_, (i32, String)>(
        "SELECT id, username FROM employees WHERE username = $1"
    )
    .bind(&claims.sub)
    .fetch_optional(db.get_ref())
    .await
    {
        Ok(Some(row)) => row,
        Ok(None) => {
            return HttpResponse::NotFound().json(LeaveResponse {
                success: false,
                message: "Employee not found".to_string(),
                data: None,
            })
        }
        Err(_) => {
            return HttpResponse::InternalServerError().json(LeaveResponse {
                success: false,
                message: "Database error".to_string(),
                data: None,
            })
        }
    };

    // Insert leave request
    let now = Utc::now();
    let leave_request = match sqlx::query_as::<_, LeaveRequest>(
        "INSERT INTO leave_requests (employee_id, employee_username, leave_type, start_date, end_date, reason, status, created_at, updated_at) 
         VALUES ($1, $2, $3, $4, $5, $6, 'Pending', $7, $8)
         RETURNING id, employee_id, employee_username, leave_type, start_date, end_date, reason, status, created_at, updated_at"
    )
    .bind(employee.0)
    .bind(&employee.1)
    .bind(&body.leave_type)
    .bind(body.start_date)
    .bind(body.end_date)
    .bind(&body.reason)
    .bind(now)
    .bind(now)
    .fetch_one(db.get_ref())
    .await
    {
        Ok(row) => row,
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            return HttpResponse::InternalServerError().json(LeaveResponse {
                success: false,
                message: "Failed to create leave request".to_string(),
                data: None,
            });
        }
    };

    HttpResponse::Created().json(LeaveResponse {
        success: true,
        message: "Leave request submitted successfully".to_string(),
        data: Some(leave_request),
    })
}

/// Get employee's own leave requests (Employee)
pub async fn get_my_leaves(
    req: HttpRequest,
    db: web::Data<PgPool>,
) -> HttpResponse {
    let claims = match claims_from_request(&req) {
        Ok(c) => c,
        Err(_) => {
            return HttpResponse::Unauthorized().json(LeaveListResponse {
                success: false,
                message: "Unauthorized".to_string(),
                data: vec![],
                count: 0,
            })
        }
    };

    if claims.role != "employee" {
        return HttpResponse::Forbidden().json(LeaveListResponse {
            success: false,
            message: "Only employees can view their leave requests".to_string(),
            data: vec![],
            count: 0,
        });
    }

    // Get employee's leave requests
    let leaves = match sqlx::query_as::<_, LeaveRequest>(
        "SELECT id, employee_id, employee_username, leave_type, start_date, end_date, reason, status, created_at, updated_at 
         FROM leave_requests 
         WHERE employee_username = $1 
         ORDER BY created_at DESC"
    )
    .bind(&claims.sub)
    .fetch_all(db.get_ref())
    .await
    {
        Ok(rows) => rows,
        Err(_) => {
            return HttpResponse::InternalServerError().json(LeaveListResponse {
                success: false,
                message: "Failed to fetch leave requests".to_string(),
                data: vec![],
                count: 0,
            })
        }
    };

    let count = leaves.len();
    HttpResponse::Ok().json(LeaveListResponse {
        success: true,
        message: "Leave requests fetched successfully".to_string(),
        data: leaves,
        count,
    })
}

/// Get all leave requests (Admin only)
pub async fn get_all_leaves(
    req: HttpRequest,
    db: web::Data<PgPool>,
) -> HttpResponse {
    let claims = match claims_from_request(&req) {
        Ok(c) => c,
        Err(_) => {
            return HttpResponse::Unauthorized().json(LeaveListResponse {
                success: false,
                message: "Unauthorized".to_string(),
                data: vec![],
                count: 0,
            })
        }
    };

    if claims.role != "admin" {
        return HttpResponse::Forbidden().json(LeaveListResponse {
            success: false,
            message: "Only admins can view all leave requests".to_string(),
            data: vec![],
            count: 0,
        });
    }

    // Get all leave requests
    let leaves = match sqlx::query_as::<_, LeaveRequest>(
        "SELECT id, employee_id, employee_username, leave_type, start_date, end_date, reason, status, created_at, updated_at 
         FROM leave_requests 
         ORDER BY created_at DESC"
    )
    .fetch_all(db.get_ref())
    .await
    {
        Ok(rows) => rows,
        Err(_) => {
            return HttpResponse::InternalServerError().json(LeaveListResponse {
                success: false,
                message: "Failed to fetch leave requests".to_string(),
                data: vec![],
                count: 0,
            })
        }
    };

    let count = leaves.len();
    HttpResponse::Ok().json(LeaveListResponse {
        success: true,
        message: "All leave requests fetched successfully".to_string(),
        data: leaves,
        count,
    })
}

/// Withdraw a leave request (Employee, pending only)
pub async fn withdraw_leave(
    req: HttpRequest,
    db: web::Data<PgPool>,
    path: web::Path<i32>,
) -> HttpResponse {
    let leave_id = path.into_inner();

    let claims = match claims_from_request(&req) {
        Ok(c) => c,
        Err(resp) => return resp,
    };

    if claims.role != "employee" {
        return HttpResponse::Forbidden().json(LeaveResponse {
            success: false,
            message: "Only employees can withdraw leave requests".to_string(),
            data: None,
        });
    }

    let employee_id = match sqlx::query_scalar::<_, i32>(
        "SELECT id FROM employees WHERE username = $1"
    )
    .bind(&claims.sub)
    .fetch_optional(db.get_ref())
    .await
    {
        Ok(Some(id)) => id,
        Ok(None) => {
            return HttpResponse::NotFound().json(LeaveResponse {
                success: false,
                message: "Employee not found".to_string(),
                data: None,
            })
        }
        Err(e) => {
            eprintln!("withdraw_leave employee lookup error for user {}: {:?}", claims.sub, e);
            return HttpResponse::InternalServerError().json(LeaveResponse {
                success: false,
                message: "Failed to validate employee account".to_string(),
                data: None,
            });
        }
    };

    let existing = match sqlx::query_as::<_, LeaveRequest>(
        "SELECT id, employee_id, employee_username, leave_type, start_date, end_date, reason, status, created_at, updated_at
         FROM leave_requests
         WHERE id = $1"
    )
    .bind(leave_id)
    .fetch_optional(db.get_ref())
    .await
    {
        Ok(Some(row)) => row,
        Ok(None) => {
            return HttpResponse::NotFound().json(LeaveResponse {
                success: false,
                message: "Leave request not found".to_string(),
                data: None,
            })
        }
        Err(e) => {
            eprintln!("withdraw_leave select error for leave_id {}: {:?}", leave_id, e);
            return HttpResponse::InternalServerError().json(LeaveResponse {
                success: false,
                message: "Failed to load leave request".to_string(),
                data: None,
            });
        }
    };

    if existing.employee_id != employee_id {
        return HttpResponse::Forbidden().json(LeaveResponse {
            success: false,
            message: "You can only withdraw your own leave requests".to_string(),
            data: None,
        });
    }

    if existing.status != "Pending" {
        return HttpResponse::BadRequest().json(LeaveResponse {
            success: false,
            message: "Only pending leave requests can be withdrawn".to_string(),
            data: None,
        });
    }

    let now = Utc::now();
    let leave_request = match sqlx::query_as::<_, LeaveRequest>(
        "UPDATE leave_requests
         SET status = 'Withdrawn', updated_at = $1
         WHERE id = $2 AND employee_id = $3 AND status = 'Pending'
         RETURNING id, employee_id, employee_username, leave_type, start_date, end_date, reason, status, created_at, updated_at"
    )
    .bind(now)
    .bind(leave_id)
    .bind(employee_id)
    .fetch_optional(db.get_ref())
    .await
    {
        Ok(Some(row)) => row,
        Ok(None) => {
            return HttpResponse::BadRequest().json(LeaveResponse {
                success: false,
                message: "Only pending leave requests can be withdrawn".to_string(),
                data: None,
            })
        }
        Err(e) => {
            eprintln!("withdraw_leave update error for leave_id {}: {:?}", leave_id, e);
            return HttpResponse::InternalServerError().json(LeaveResponse {
                success: false,
                message: "Failed to withdraw leave request".to_string(),
                data: None,
            })
        }
    };

    HttpResponse::Ok().json(LeaveResponse {
        success: true,
        message: "Leave request withdrawn successfully".to_string(),
        data: Some(leave_request),
    })
}

/// Approve a leave request (Admin only)
pub async fn approve_leave(
    req: HttpRequest,
    db: web::Data<PgPool>,
    path: web::Path<i32>,
) -> HttpResponse {
    let leave_id = path.into_inner();

    let claims = match claims_from_request(&req) {
        Ok(c) => c,
        Err(resp) => return resp,
    };

    if claims.role != "admin" {
        return HttpResponse::Forbidden().json(LeaveResponse {
            success: false,
            message: "Only admins can approve leave requests".to_string(),
            data: None,
        });
    }

    // Update leave request status
    let now = Utc::now();
    let leave_request = match sqlx::query_as::<_, LeaveRequest>(
        "UPDATE leave_requests 
         SET status = 'Approved', updated_at = $1 
         WHERE id = $2 AND status = 'Pending'
         RETURNING id, employee_id, employee_username, leave_type, start_date, end_date, reason, status, created_at, updated_at"
    )
    .bind(now)
    .bind(leave_id)
    .fetch_optional(db.get_ref())
    .await
    {
        Ok(Some(row)) => row,
        Ok(None) => {
            return HttpResponse::BadRequest().json(LeaveResponse {
                success: false,
                message: "Leave request not found or already processed".to_string(),
                data: None,
            })
        }
        Err(_) => {
            return HttpResponse::InternalServerError().json(LeaveResponse {
                success: false,
                message: "Failed to approve leave request".to_string(),
                data: None,
            })
        }
    };

    HttpResponse::Ok().json(LeaveResponse {
        success: true,
        message: "Leave request approved successfully".to_string(),
        data: Some(leave_request),
    })
}

/// Reject a leave request (Admin only)
pub async fn reject_leave(
    req: HttpRequest,
    db: web::Data<PgPool>,
    path: web::Path<i32>,
) -> HttpResponse {
    let leave_id = path.into_inner();

    let claims = match claims_from_request(&req) {
        Ok(c) => c,
        Err(resp) => return resp,
    };

    if claims.role != "admin" {
        return HttpResponse::Forbidden().json(LeaveResponse {
            success: false,
            message: "Only admins can reject leave requests".to_string(),
            data: None,
        });
    }

    // Update leave request status
    let now = Utc::now();
    let leave_request = match sqlx::query_as::<_, LeaveRequest>(
        "UPDATE leave_requests 
         SET status = 'Rejected', updated_at = $1 
         WHERE id = $2 AND status = 'Pending'
         RETURNING id, employee_id, employee_username, leave_type, start_date, end_date, reason, status, created_at, updated_at"
    )
    .bind(now)
    .bind(leave_id)
    .fetch_optional(db.get_ref())
    .await
    {
        Ok(Some(row)) => row,
        Ok(None) => {
            return HttpResponse::BadRequest().json(LeaveResponse {
                success: false,
                message: "Leave request not found or already processed".to_string(),
                data: None,
            })
        }
        Err(_) => {
            return HttpResponse::InternalServerError().json(LeaveResponse {
                success: false,
                message: "Failed to reject leave request".to_string(),
                data: None,
            })
        }
    };

    HttpResponse::Ok().json(LeaveResponse {
        success: true,
        message: "Leave request rejected successfully".to_string(),
        data: Some(leave_request),
    })
}
