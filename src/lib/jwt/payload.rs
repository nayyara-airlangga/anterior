use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthToken {
    exp: i64,
    iat: i64,
    id: u64,
    username: String,
    name: String,
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
