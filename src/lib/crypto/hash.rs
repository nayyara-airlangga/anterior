use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};

pub fn create_hash(text: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    argon2
        .hash_password(text.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

pub fn verify_hash(text: &str, hash_str: &str) -> bool {
    let hash = PasswordHash::new(hash_str).unwrap();

    Argon2::default()
        .verify_password(text.as_bytes(), &hash)
        .is_ok()
}
