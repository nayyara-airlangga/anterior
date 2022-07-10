use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::user::User;

#[derive(FromRow, Debug, Deserialize, Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub headline: String,
    pub slug: String,
    pub content: String,
    pub published: bool,

    pub created_at: chrono::DateTime<chrono::Local>,
    pub edited_at: chrono::DateTime<chrono::Local>,
    pub published_at: Option<chrono::DateTime<chrono::Local>>,

    pub author_id: i32,
}

#[derive(FromRow, Debug, Deserialize, Serialize)]
pub struct PostWithAuthor {
    pub id: i32,
    pub title: String,
    pub headline: String,
    pub slug: String,
    pub content: String,
    pub published: bool,

    pub created_at: chrono::DateTime<chrono::Local>,
    pub edited_at: chrono::DateTime<chrono::Local>,
    pub published_at: Option<chrono::DateTime<chrono::Local>>,

    pub author_id: i32,
    pub author: User,
}
