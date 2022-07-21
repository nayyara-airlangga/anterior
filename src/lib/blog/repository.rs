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

    pub async fn get_posts(&self) -> Result<Vec<Post>> {
        let posts = sqlx::query::<Postgres>(
            r#"
SELECT posts.id "p_id", title, headline, slug, content, published, posts.created_at "p_created_at", edited_at, published_at, author_id, users.id "u_id", username, name, email, users.created_at "u_created_at"
FROM posterior.posts AS posts
LEFT JOIN posterior.users AS users
ON posts.author_id = users.id
ORDER BY posts.created_at DESC, edited_at DESC, title ASC
"#,
        ).fetch_all(&self.pool).await?;

        let posts = posts.into_iter().map(|row| row.into()).collect();

        Ok(posts)
    }
}
