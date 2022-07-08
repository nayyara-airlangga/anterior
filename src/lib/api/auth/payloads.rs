use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterPayload {
    pub username: String,
    pub email: String,
    pub name: String,
    pub password: String,
    pub remember_me: Option<bool>,
}
