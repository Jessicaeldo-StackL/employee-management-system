# Adding Test Users to Employee Management System

## Quick Start - 2 Minutes

### Option 1: Using pgAdmin (Easiest) ✅

1. **Open pgAdmin** in your browser (usually http://localhost:5050)
2. **Navigate to**: Databases → employee_management → Schemas → public → Tables → users
3. **Right-click on "users"** → **Query Tool**
4. **Copy and paste the SQL below:**

```sql
-- Insert all test users
INSERT INTO users (username, password, role) 
VALUES ('admin', '$argon2id$v=19$m=65536,t=3,p=4$kD+Mp/ZX6FkYI27f5U6KDQ$Z8IFHHzwoDpzsWHGR/Fh+b9lBLJS1XWxlH1P8+C94O8', 'admin')
ON CONFLICT (username) DO UPDATE SET password = EXCLUDED.password;

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
```

5. **Press F5 or click Execute** (lightning bolt icon)
6. **Done!** All 6 users are now in the database ✅

---

## Test Credentials

| Username | Password | Role | Status |
|----------|----------|------|--------|
| **admin** | admin123 | Admin | ✅ Admin dashboard access |
| **user1** | password123 | Employee | ✅ Employee dashboard |
| **user2** | password456 | Employee | ✅ Employee dashboard |
| **jessica** | jessica123 | Employee | ✅ Employee dashboard |
| **john** | john456 | Employee | ✅ Employee dashboard |
| **sarah** | sarah789 | Employee | ✅ Employee dashboard |

---

## Test Login

Once users are added:

1. **Go to**: http://localhost:3000/login
2. **Try logging in with any credentials** from the table above
3. **Should redirect** to:
   - `/admin` if admin
   - `/employee` if employee

---

## Verify Users Were Added

In pgAdmin, run this query:

```sql
SELECT username, role FROM users ORDER BY username;
```

Should show all 6 users ✅

---

## Adding More Users Later

To add more employees in the future, you have two options:

### Option A: Add via Backend Script
Edit `backend/generate_test_users.py` and add users to the list:
```python
test_users = [
    ("username", "password", "role"),
    # Add more here
]
```

### Option B: Direct SQL Insert
```sql
INSERT INTO users (username, password, role) 
VALUES ('newuser', 'argon2_hash_here', 'employee')
ON CONFLICT (username) DO UPDATE SET password = EXCLUDED.password;
```

To generate Argon2 hashes for new users:
```bash
cd backend
python3 generate_test_users.py
# Add new user to the list, run again, copy the generated hash
```

---

## Troubleshooting

**"ERROR: duplicate key value violates unique constraint"**
- This is normal! It means users already exist. The `ON CONFLICT` clause updates them instead.

**"ERROR: column 'username' does not exist"**
- Make sure migrations were applied: `sqlx migrate run`
- Check table exists: `SELECT * FROM users LIMIT 1;`

**Can't log in after adding users**
- Verify user was added: `SELECT username, role FROM users WHERE username='jessica';`
- Check password is correct (case-sensitive)
- Check backend is running on http://127.0.0.1:8081

---

**All set!** 🎉 You now have 6 test users ready to log in. Try logging in as Jessica now! 👍
