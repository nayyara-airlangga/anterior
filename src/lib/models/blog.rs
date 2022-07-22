use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Row};

use super::{Metadata, User};

#[derive(Debug, Deserialize, Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub headline: String,
    pub slug: String,
    pub published: bool,

    pub created_at: chrono::DateTime<chrono::Local>,
    pub edited_at: chrono::DateTime<chrono::Local>,
    pub published_at: Option<chrono::DateTime<chrono::Local>>,
}

impl From<PgRow> for Post {
    fn from(row: PgRow) -> Self {
        Post {
            id: row.get("id"),
            title: row.get("title"),
            headline: row.get("headline"),
            slug: row.get("headline"),
            published: row.get("published"),
            created_at: row.get("created_at"),
            edited_at: row.get("edited_at"),
            published_at: row.get("published_at"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostDetail {
    pub id: i32,
    pub title: String,
    pub headline: String,
    pub slug: String,
    pub published: bool,
    pub content: String,

    pub created_at: chrono::DateTime<chrono::Local>,
    pub edited_at: chrono::DateTime<chrono::Local>,
    pub published_at: Option<chrono::DateTime<chrono::Local>>,

    pub author_id: i32,
    pub author: User,
}

impl From<PgRow> for PostDetail {
    fn from(row: PgRow) -> Self {
        PostDetail {
            id: row.get("p_id"),
            title: row.get("title"),
            headline: row.get("headline"),
            slug: row.get("headline"),
            published: row.get("published"),
            content: row.get("content"),
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

#[derive(Serialize)]
pub struct PostsWithMeta {
    pub metadata: Metadata,
    pub posts: Vec<Post>,
}

impl PostsWithMeta {
    pub fn new(posts: Vec<Post>, metadata: Metadata) -> Self {
        Self { posts, metadata }
    }
}
