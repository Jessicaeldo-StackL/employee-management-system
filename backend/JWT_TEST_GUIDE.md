# JWT Authentication Test Guide

## Prerequisites
- Backend running on http://localhost:8081
- PostgreSQL database with migrations applied
- A test user in the database

## Step 1: Create a Test User (if needed)

Use psql to connect to your database:
```bash
psql -U postgres -d employee_management
```

Then create a test user with a password hash. First, you need to hash a password using Argon2.

For testing, you can use Python:
```python
from argon2 import PasswordHasher
ph = PasswordHasher()
hashed = ph.hash("testpassword")
print(hashed)
```

Then insert the user:
```sql
INSERT INTO users (username, password, role) 
VALUES ('testuser', '$argon2id$v=19$m=19456,t=2,p=1$YOUR_HASH_HERE', 'employee')
ON CONFLICT (username) DO UPDATE SET password = EXCLUDED.password;
```

## Step 2: Test Login

### Using cURL:
```bash
curl -X POST http://localhost:8081/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "password": "testpassword"
  }'
```

### Using Postman or Thunder Client:
- Method: POST
- URL: http://localhost:8081/login
- Body (JSON):
```json
{
  "username": "testuser",
  "password": "testpassword"
}
```

## Step 3: Expected Response

### Success (200 OK):
```json
{
  "success": true,
  "message": "Login successful",
  "username": "testuser",
  "role": "employee",
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ0ZXN0dXNlciIsInJvbGUiOiJlbXBsb3llZSIsImV4cCI6MTY4NzU0MzIwMCwiaWF0IjoxNjg3NDU2ODAwfQ.abc123..."
}
```

### Failure - Invalid Password (401 Unauthorized):
```json
{
  "success": false,
  "message": "Invalid username or password",
  "role": "",
  "username": "",
  "token": ""
}
```

### Failure - User Not Found (401 Unauthorized):
```json
{
  "success": false,
  "message": "Invalid username or password",
  "role": "",
  "username": "",
  "token": ""
}
```

## Step 4: Verify Token in Database

Check that the token was saved to the database:
```sql
SELECT username, token, token_expiry FROM users WHERE username = 'testuser';
```

You should see:
- token: The JWT token string
- token_expiry: A timestamp approximately 24 hours in the future

## Step 5: Decode the JWT Token

Visit https://jwt.io and paste the token to verify its contents.

You should see the payload contains:
- `sub`: testuser (username)
- `role`: employee
- `exp`: Unix timestamp for expiry (24 hours from login)
- `iat`: Unix timestamp for issued-at

## Environment Variables

Configure these in `.env`:
- `JWT_SECRET`: The secret key used to sign tokens (change in production!)
- `JWT_EXPIRY_HOURS`: Token validity period (default: 24)
- `DATABASE_URL`: PostgreSQL connection string

## Configuration

Current settings:
- JWT_EXPIRY_HOURS: 24 hours
- JWT_SECRET: `ems_super_secret_jwt_key_change_in_production_2026!!` (CHANGE IN PRODUCTION)

To change expiry time, update `.env`:
```
JWT_EXPIRY_HOURS=48
```
