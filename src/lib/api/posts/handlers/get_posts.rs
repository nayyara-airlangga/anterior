use actix_web::{web, HttpResponse};
use serde_json::json;
use sqlx::{postgres::PgRow, Pool, Postgres, Row};

use crate::models::Post;

type DbPool = Pool<Postgres>;

pub async fn get_posts(pool: web::Data<DbPool>) -> HttpResponse {
    let posts = match sqlx::query::<Postgres>("
SELECT posts.id, title, headline, slug, published, posts.created_at, edited_at, published_at, author_id
FROM posterior.posts AS posts
LEFT JOIN posterior.users AS users
ON posts.author_id = users.id
ORDER BY posts.created_at DESC, edited_at DESC, title ASC
",
    ).map(|row: PgRow| Post {
        id: row.get("id"),
        title: row.get("title"),
        headline: row.get("headline"),
        slug: row.get("slug"),
        published: row.get("published"),
        created_at: row.get("created_at"),
        edited_at: row.get("edited_at"),
        published_at: row.get("published_at"),
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
