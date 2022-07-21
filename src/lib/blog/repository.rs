use sqlx::{PgPool, Postgres, Result};

use crate::models::Post;

#[derive(Clone)]
pub struct BlogRepository {
    pub pool: PgPool,
}

impl BlogRepository {
    pub fn new(pool: PgPool) -> BlogRepository {
        BlogRepository { pool }
    }

    pub async fn get_posts(&self, limit: i32, cursor: Option<i32>) -> Result<Vec<Post>> {
        let cursor_condition = if let Some(cursor) = cursor {
            format!("posts.id < {cursor}")
        } else {
            format!("true")
        };

        let query_str = format!(
            r#"
SELECT posts.id, title, headline, slug, published, posts.created_at, edited_at, published_at
FROM posterior.posts AS posts
WHERE {}
ORDER BY posts.created_at DESC, edited_at DESC, title ASC
LIMIT $1
"#,
            cursor_condition
        );

        let posts = sqlx::query::<Postgres>(&query_str)
            .bind(&limit)
            .fetch_all(&self.pool)
            .await?;

        let posts = posts.into_iter().map(|row| row.into()).collect();

        Ok(posts)
    }
}
