use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use serde::{Deserialize, Serialize};

/// Struct for holding data from the JWT.
#[derive(Debug, Deserialize, Serialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

pub fn hash_password(password: &str) -> String {
    // Create an instance of the Argon2 hasher
    let argon2 = Argon2::default();

    // Generate a secure random salt
    let salt = SaltString::generate(&mut OsRng);

    // Hash the password
    argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Password hashing failed")
        .to_string()
}

pub fn verify_password(password: &str, hashed_password: &str) -> bool {
    let is_valid = match PasswordHash::new(hashed_password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_err) => false,
    };

    is_valid
}

#[cfg(test)]
mod test {

    use crate::jwt::{hash_password, verify_password};

    #[test]
    fn test_token_claims() {
        let hash = hash_password("pass");
        println!("hash: {}", hash);
        let valid = verify_password("pass", &hash);
        assert!(valid);
    }
}
