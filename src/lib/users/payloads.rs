use actix_web::HttpResponse;
use serde::Deserialize;
use serde_json::json;

use crate::models::user::User;

#[derive(Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
    pub remember_me: Option<bool>,
}

#[derive(Deserialize)]
pub struct RegisterPayload {
    pub username: String,
    pub email: String,
    pub name: String,
    pub password: String,
    pub remember_me: Option<bool>,
}

pub struct GetSelfResponse;

impl GetSelfResponse {
    pub fn new(user: User) -> HttpResponse {
        HttpResponse::Ok().json(user)
    }
}

pub struct TokenResponse;

impl TokenResponse {
    pub fn new(token: String) -> HttpResponse {
        HttpResponse::Created().json(json!({ "token": token }))
    }
}
