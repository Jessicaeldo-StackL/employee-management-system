-- Allow withdrawn leave requests while keeping history records

DO $$
BEGIN
    IF EXISTS (
        SELECT 1
        FROM information_schema.table_constraints
        WHERE table_name = 'leave_requests'
          AND constraint_name = 'chk_status'
    ) THEN
        ALTER TABLE leave_requests DROP CONSTRAINT chk_status;
    END IF;

    IF EXISTS (
        SELECT 1
        FROM information_schema.table_constraints
        WHERE table_name = 'leave_requests'
          AND constraint_name = 'check_status'
    ) THEN
        ALTER TABLE leave_requests DROP CONSTRAINT check_status;
    END IF;
END $$;

ALTER TABLE leave_requests
    ADD CONSTRAINT chk_status
    CHECK (status IN ('Pending', 'Approved', 'Rejected', 'Withdrawn'));

COMMENT ON COLUMN leave_requests.status IS
    'Leave request status: Pending, Approved, Rejected, or Withdrawn';
