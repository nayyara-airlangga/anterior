use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthToken {
    exp: i64,
    iat: i64,
    pub id: u64,
    pub username: String,
    pub name: String,
}

impl AuthToken {
    pub fn new(id: u64, username: String, name: String, exp: i64) -> AuthToken {
        AuthToken {
            id,
            username,
            name,
            exp,
            iat: chrono::offset::Local::now().timestamp(),
        }
    }
}
