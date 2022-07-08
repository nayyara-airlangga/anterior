use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Debug, Deserialize, Serialize)]
pub struct UserWithPassword {
    pub id: i32,
    pub username: String,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(FromRow, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub name: String,
    pub email: String,
}
