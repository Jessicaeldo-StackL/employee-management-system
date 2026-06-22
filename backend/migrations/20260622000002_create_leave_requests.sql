-- Create leave_requests table for Leave Management System
-- Created: 2026-06-22

CREATE TABLE IF NOT EXISTS leave_requests (
    id SERIAL PRIMARY KEY,
    employee_id INTEGER NOT NULL,
    employee_username VARCHAR(255) NOT NULL,
    leave_type VARCHAR(50) NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    reason TEXT NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'Pending',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    -- Foreign key constraint
    CONSTRAINT fk_leave_requests_employee 
        FOREIGN KEY (employee_id) 
        REFERENCES users(id) 
        ON DELETE CASCADE,
    
    -- Constraints
    CONSTRAINT chk_dates 
        CHECK (end_date >= start_date),
    
    CONSTRAINT chk_status 
        CHECK (status IN ('Pending', 'Approved', 'Rejected')),
    
    CONSTRAINT chk_leave_type 
        CHECK (leave_type IN ('Sick Leave', 'Vacation', 'Personal Leave', 'Maternity Leave', 'Other'))
);

-- Create indexes for better query performance
CREATE INDEX idx_leave_requests_employee_id 
    ON leave_requests(employee_id);

CREATE INDEX idx_leave_requests_status 
    ON leave_requests(status);

CREATE INDEX idx_leave_requests_created_at 
    ON leave_requests(created_at DESC);

CREATE INDEX idx_leave_requests_employee_id_status 
    ON leave_requests(employee_id, status);

-- Add comment to table
COMMENT ON TABLE leave_requests IS 
    'Stores leave/vacation requests from employees with approval status';

COMMENT ON COLUMN leave_requests.status IS 
    'Leave request status: Pending, Approved, or Rejected';

COMMENT ON COLUMN leave_requests.leave_type IS 
    'Type of leave: Sick Leave, Vacation, Personal Leave, Maternity Leave, Other';
