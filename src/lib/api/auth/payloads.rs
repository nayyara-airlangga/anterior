use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
    pub remember_me: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterPayload {
    pub username: String,
    pub email: String,
    pub name: String,
    pub password: String,
    pub remember_me: Option<bool>,
}
