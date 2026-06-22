-- Fix legacy employee schemas missing updated_at and ensure update trigger is valid
-- Root cause addressed: trigger function references NEW.updated_at during employee UPDATE,
-- but some existing databases can have employees table created before updated_at existed.

ALTER TABLE employees
    ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();

CREATE OR REPLACE FUNCTION trigger_set_employees_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_updated_at_employees ON employees;

CREATE TRIGGER set_updated_at_employees
    BEFORE UPDATE ON employees
    FOR EACH ROW
    EXECUTE FUNCTION trigger_set_employees_updated_at();
