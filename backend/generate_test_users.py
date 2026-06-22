#!/usr/bin/env python3
"""
Utility script to generate Argon2 password hashes for testing JWT authentication.

Usage:
    python3 generate_test_users.py

This script helps generate password hashes for test users to use with the Employee Management System.
"""

try:
    from argon2 import PasswordHasher
    from argon2.exceptions import InvalidHash
except ImportError:
    print("Error: argon2-cffi is not installed.")
    print("Install it with: pip install argon2-cffi")
    exit(1)

def generate_hash(password: str) -> str:
    """Generate an Argon2 password hash."""
    ph = PasswordHasher()
    return ph.hash(password)

def verify_hash(password: str, hash_str: str) -> bool:
    """Verify a password against an Argon2 hash."""
    ph = PasswordHasher()
    try:
        ph.verify(hash_str, password)
        return True
    except InvalidHash:
        return False

if __name__ == "__main__":
    print("=" * 60)
    print("JWT Authentication Test User Generator")
    print("=" * 60)
    print()
    
    # Generate test users - 1 admin + 4 employees
    test_users = [
        ("admin", "admin123", "admin"),
        ("user1", "password123", "employee"),
        ("user2", "password456", "employee"),
        ("jessica", "jessica123", "employee"),
        ("john", "john456", "employee"),
        ("sarah", "sarah789", "employee"),
    ]
    
    print("Generated Test Users (SQL INSERT):")
    print("-" * 60)
    
    for username, password, role in test_users:
        password_hash = generate_hash(password)
        
        sql = f"""INSERT INTO users (username, password, role) 
VALUES ('{username}', '{password_hash}', '{role}')
ON CONFLICT (username) DO UPDATE SET password = EXCLUDED.password;"""
        
        print(f"\nUser: {username}")
        print(f"Password: {password}")
        print(f"Role: {role}")
        print(f"Hash: {password_hash}")
        print()
        print("SQL Command:")
        print(sql)
        print("-" * 60)
    
    print()
    print("Instructions:")
    print("1. Copy the SQL commands above")
    print("2. Connect to PostgreSQL: psql -U postgres -d employee_management")
    print("3. Paste and execute the SQL commands")
    print()
    print("Then test the login with curl:")
    print('  curl -X POST http://localhost:8081/login \\')
    print('    -H "Content-Type: application/json" \\')
    print('    -d \'{"username":"admin","password":"admin123"}\'')
    print()
