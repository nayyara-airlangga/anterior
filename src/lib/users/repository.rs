use sqlx::{Pool, Postgres, Result};

use crate::models::user::{User, UserWithPassword};

#[derive(Clone)]
pub struct UserRepository {
    pub pool: Pool<Postgres>,
}

impl UserRepository {
    pub fn new(pool: Pool<Postgres>) -> UserRepository {
        UserRepository { pool }
    }

    pub async fn get_user_by_id(&self, id: i32) -> Result<User> {
        sqlx::query_as::<Postgres, User>(
            "
SELECT id, username, name, email, created_at
FROM posterior.users
WHERE id = $1
",
        )
        .bind(&id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_user_by_username_or_email(
        &self,
        username: &str,
        email: &str,
    ) -> Result<UserWithPassword> {
        sqlx::query_as::<Postgres, UserWithPassword>(
            "
SELECT *
FROM posterior.users
WHERE username = $1 OR email = $2
",
        )
        .bind(username)
        .bind(email)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn insert_user(
        &self,
        username: &str,
        name: &str,
        email: &str,
        hash: &str,
    ) -> Result<UserWithPassword> {
        sqlx::query_as(
            "
INSERT INTO posterior.users (username, name, email, password)
VALUES($1, $2, $3, $4)
RETURNING *
",
        )
        .bind(username)
        .bind(name)
        .bind(email)
        .bind(hash)
        .fetch_one(&self.pool)
        .await
    }
}
