use argon2::Argon2;
use password_hash::{
    rand_core::OsRng,
    PasswordHash,
    PasswordHasher,
    PasswordVerifier,
    SaltString,
};
pub fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);

    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| e.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    println!("VERIFYING PASSWORD");
    println!("Password: {}", password);
    println!("Hash: {}", hash);

    let parsed_hash = match PasswordHash::new(hash) {
        Ok(parsed) => parsed,
        Err(e) => {
            println!("HASH PARSE ERROR: {:?}", e);
            return false;
        }
    };

    match Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => {
            println!("PASSWORD MATCH");
            true
        }
        Err(e) => {
            println!("PASSWORD VERIFY ERROR: {:?}", e);
            false
        }
    }
}