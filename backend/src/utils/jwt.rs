use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,   // username
    pub role: String,  // "admin" or "employee"
    pub exp: i64,      // expiry timestamp (Unix)
    pub iat: i64,      // issued-at timestamp
}

fn secret() -> Vec<u8> {
    env::var("JWT_SECRET")
        .unwrap_or_else(|_| "changeme_default_secret_32bytes!!".to_string())
        .into_bytes()
}

fn get_expiry_hours() -> i64 {
    env::var("JWT_EXPIRY_HOURS")
        .unwrap_or_else(|_| "24".to_string())
        .parse::<i64>()
        .unwrap_or(24)
}

/// Generate a signed JWT for the given username and role.
/// Token expiry is configurable via JWT_EXPIRY_HOURS env var (default: 24 hours).
/// Returns (token, expiry_timestamp)
pub fn generate_token(username: &str, role: &str) -> Result<(String, i64), String> {
    let now = Utc::now();
    let expiry_hours = get_expiry_hours();
    let expiry_time = now + Duration::hours(expiry_hours);
    
    let claims = Claims {
        sub: username.to_string(),
        role: role.to_string(),
        iat: now.timestamp(),
        exp: expiry_time.timestamp(),
    };
    
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(&secret()),
    )
    .map_err(|e| format!("Token generation failed: {e}"))?;
    
    Ok((token, expiry_time.timestamp()))
}

/// Validate a JWT string and return its claims.
pub fn validate_token(token: &str) -> Result<TokenData<Claims>, String> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(&secret()),
        &Validation::default(),
    )
    .map_err(|e| format!("Token validation failed: {e}"))
}
