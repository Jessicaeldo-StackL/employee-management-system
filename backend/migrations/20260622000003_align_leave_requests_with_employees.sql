-- Align leave_requests schema with employee management requirements
-- Ensures leave records are linked to employees(id)

DO $$
BEGIN
    IF EXISTS (
        SELECT 1
        FROM information_schema.table_constraints
        WHERE constraint_name = 'fk_leave_requests_employee'
          AND table_name = 'leave_requests'
    ) THEN
        ALTER TABLE leave_requests DROP CONSTRAINT fk_leave_requests_employee;
    END IF;

    IF EXISTS (
        SELECT 1
        FROM information_schema.table_constraints
        WHERE constraint_name = 'fk_leave_requests_employee_id'
          AND table_name = 'leave_requests'
    ) THEN
        ALTER TABLE leave_requests DROP CONSTRAINT fk_leave_requests_employee_id;
    END IF;
END $$;

-- Remove orphaned rows, if any exist before adding the new FK
DELETE FROM leave_requests lr
WHERE NOT EXISTS (
    SELECT 1 FROM employees e WHERE e.id = lr.employee_id
);

ALTER TABLE leave_requests
    ADD CONSTRAINT fk_leave_requests_employee_id
    FOREIGN KEY (employee_id)
    REFERENCES employees(id)
    ON DELETE CASCADE;

ALTER TABLE leave_requests
    ALTER COLUMN status SET DEFAULT 'Pending';

DO $$
BEGIN
    IF EXISTS (
        SELECT 1
        FROM information_schema.table_constraints
        WHERE constraint_name = 'chk_leave_type'
          AND table_name = 'leave_requests'
    ) THEN
        ALTER TABLE leave_requests DROP CONSTRAINT chk_leave_type;
    END IF;

    IF EXISTS (
        SELECT 1
        FROM information_schema.table_constraints
        WHERE constraint_name = 'check_leave_type'
          AND table_name = 'leave_requests'
    ) THEN
        ALTER TABLE leave_requests DROP CONSTRAINT check_leave_type;
    END IF;
END $$;

ALTER TABLE leave_requests
    ADD CONSTRAINT chk_leave_type
    CHECK (leave_type IN ('Sick Leave', 'Vacation', 'Personal Leave', 'Maternity Leave', 'Other'));

CREATE INDEX IF NOT EXISTS idx_leave_requests_employee_id_status
    ON leave_requests(employee_id, status);