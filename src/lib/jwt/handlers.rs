use std::{env, process};

use jsonwebtoken::{
    decode, encode, errors::Result, DecodingKey, EncodingKey, Header, TokenData, Validation,
};

use crate::jwt::payload::AuthToken;

pub fn create_auth_token(payload: &AuthToken) -> Result<String> {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|err| {
        log::error!("{err}");
        process::exit(1)
    });

    encode::<AuthToken>(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn decode_auth_token(token: &str) -> Result<TokenData<AuthToken>> {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|err| {
        log::error!("{err}");
        process::exit(1)
    });

    decode::<AuthToken>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
}
