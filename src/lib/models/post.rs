use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Row};

use super::user::User;

#[derive(Debug, Deserialize, Serialize)]
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
    pub author: User,
}

impl From<PgRow> for Post {
    fn from(row: PgRow) -> Self {
        Post {
            id: row.get("p_id"),
            title: row.get("title"),
            headline: row.get("headline"),
            slug: row.get("headline"),
            content: row.get("content"),
            published: row.get("published"),
            created_at: row.get("p_created_at"),
            edited_at: row.get("edited_at"),
            published_at: row.get("published_at"),
            author_id: row.get("author_id"),
            author: User {
                id: row.get("u_id"),
                username: row.get("username"),
                name: row.get("name"),
                email: row.get("email"),
                created_at: row.get("u_created_at"),
            },
        }
    }
}
