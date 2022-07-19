use std::{env, process};

use argon2::{self, Config, Variant};

pub fn create_hash(text: &str) -> String {
    let salt = env::var("HASH_SALT").unwrap_or_else(|err| {
        log::error!("{err}");
        process::exit(1)
    });
    let salt = salt.as_bytes();

    let mut config = Config::default();
    config.variant = Variant::Argon2id;

    argon2::hash_encoded(text.as_bytes(), salt, &config).unwrap()
}

pub fn verify_hash(text: &str, hash_str: &str) -> bool {
    let mut config = Config::default();
    config.variant = Variant::Argon2id;

    argon2::verify_encoded(hash_str, text.as_bytes()).unwrap()
}
