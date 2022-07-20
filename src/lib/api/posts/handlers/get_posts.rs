use actix_web::{web, HttpResponse};
use serde_json::json;
use sqlx::{postgres::PgRow, Pool, Postgres, Row};

use crate::models::{post::Post, user::User};

type DbPool = Pool<Postgres>;

pub async fn get_posts(pool: web::Data<DbPool>) -> HttpResponse {
    let posts = match sqlx::query::<Postgres>("
SELECT posts.id, title, headline, slug, content, published, posts.created_at, edited_at, published_at, author_id, users.id, username, name, email, users.created_at
FROM posterior.posts AS posts
LEFT JOIN posterior.users AS users
ON posts.author_id = users.id
ORDER BY posts.created_at DESC, edited_at DESC, title ASC
",
    ).map(|row: PgRow| Post {
        id: row.get(0),
        title: row.get(1),
        headline: row.get(2),
        slug: row.get(3),
        content: row.get(4),
        published: row.get(5),
        created_at: row.get(6),
        edited_at: row.get(7),
        published_at: row.get(8),
        author_id: row.get(9),
            author: User {
                id: row.get(10),
                username: row.get(11),
                name: row.get(12),
                email: row.get(13),
                created_at: row.get(14)
            }
    })
    .fetch_all(&**pool)
    .await
    {
        Ok(query) => query,
        Err(err) => {
            log::error!("{err}");
            return HttpResponse::InternalServerError().finish();
        }
    };

    HttpResponse::Ok().json(json!({ "posts": posts }))
}
