use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Row};

use super::Metadata;

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
            id: row.get("p_id"),
            title: row.get("title"),
            headline: row.get("headline"),
            slug: row.get("headline"),
            published: row.get("published"),
            created_at: row.get("p_created_at"),
            edited_at: row.get("edited_at"),
            published_at: row.get("published_at"),
        }
    }
}

#[derive(Serialize)]
pub struct PostsWithMeta {
    pub posts: Vec<Post>,
    pub metadata: Metadata,
}

impl PostsWithMeta {
    pub fn new(posts: Vec<Post>, metadata: Metadata) -> Self {
        Self { posts, metadata }
    }
}
