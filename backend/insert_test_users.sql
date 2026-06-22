-- ============================================================
-- JWT Authentication Test Users
-- Generated: 2026-06-22
-- ============================================================
-- Execute these SQL commands to insert all test users
-- 
-- Usage:
--   psql -U postgres -d employee_management -f insert_test_users.sql
--
-- Or paste directly into pgAdmin SQL editor
-- ============================================================

-- Admin User
INSERT INTO users (username, password, role) 
VALUES ('admin', '$argon2id$v=19$m=65536,t=3,p=4$kD+Mp/ZX6FkYI27f5U6KDQ$Z8IFHHzwoDpzsWHGR/Fh+b9lBLJS1XWxlH1P8+C94O8', 'admin')
ON CONFLICT (username) DO UPDATE SET password = EXCLUDED.password;

-- Employee Users
INSERT INTO users (username, password, role) 
VALUES ('user1', '$argon2id$v=19$m=65536,t=3,p=4$g5jUHwiCK0yBpYcM4sbu7g$pqNcXXZcMzQSh7bKoFVwQJxw2++c/l2+gMa7h/3WVgU', 'employee')
ON CONFLICT (username) DO UPDATE SET password = EXCLUDED.password;

INSERT INTO users (username, password, role) 
VALUES ('user2', '$argon2id$v=19$m=65536,t=3,p=4$uym7YNxl1zia6ycJxqGAVw$v9XpII1FLC5AEfumX1AR4hBeyI0wRrmlrLYQ2QdwNBg', 'employee')
ON CONFLICT (username) DO UPDATE SET password = EXCLUDED.password;

INSERT INTO users (username, password, role) 
VALUES ('jessica', '$argon2id$v=19$m=65536,t=3,p=4$Hgt8IEHYzvfvluklds9r2w$PXLRNffvwZ+0uKShcCJXBCOxvwHNwDo0B4esWZltW/k', 'employee')
ON CONFLICT (username) DO UPDATE SET password = EXCLUDED.password;

INSERT INTO users (username, password, role) 
VALUES ('john', '$argon2id$v=19$m=65536,t=3,p=4$MgUK8oJZwU9hZXMnK/tVNA$djARjmSgldm52ZlSKVjO6y443yhVOXTyGkTnMiaq4DY', 'employee')
ON CONFLICT (username) DO UPDATE SET password = EXCLUDED.password;

INSERT INTO users (username, password, role) 
VALUES ('sarah', '$argon2id$v=19$m=65536,t=3,p=4$Mr9x+QG1TEDOubu9HmAuvA$RhjvNfJFJ63HXo3gv4GXR2S+APg7OZBq/KEiA0VUMMs', 'employee')
ON CONFLICT (username) DO UPDATE SET password = EXCLUDED.password;

-- ============================================================
-- Test Credentials:
-- ============================================================
-- ADMIN:
--   Username: admin
--   Password: admin123
--   Role: admin
--
-- EMPLOYEES:
--   Username: user1      | Password: password123 | Role: employee
--   Username: user2      | Password: password456 | Role: employee
--   Username: jessica    | Password: jessica123  | Role: employee
--   Username: john       | Password: john456     | Role: employee
--   Username: sarah      | Password: sarah789    | Role: employee
-- ============================================================
