use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use jsonwebtoken::TokenData;
use serde_json::json;
use sqlx::{postgres::PgRow, Pool, Postgres, Row};

use crate::{
    api::posts::payloads::create_post::CreatePostPayload, jwt::payload::AuthToken, models::Post,
};

type DbPool = Pool<Postgres>;

pub async fn create_post(
    req: HttpRequest,
    body: web::Json<CreatePostPayload>,
    pool: web::Data<DbPool>,
) -> HttpResponse {
    let AuthToken { id, .. } = req
        .extensions()
        .get::<TokenData<AuthToken>>()
        .unwrap()
        .claims;
    let id = id as i32;

    let slug = slug::slugify(&body.title);
    let published = if let Some(published) = body.published {
        published
    } else {
        false
    };

    let post = match sqlx::query::<Postgres>(
        "
WITH post AS (
INSERT INTO posterior.posts (title, slug, headline, content, published, author_id)
VALUES($1, $2, $3, $4, $5, $6)
RETURNING *
)
SELECT post.id, title, headline, slug, published, post.created_at, edited_at, published_at, author_id 
FROM post
LEFT JOIN posterior.users AS users ON post.author_id = users.id
",
    )
    .bind(&body.title)
    .bind(&slug)
    .bind(&body.headline)
    .bind(&body.content)
    .bind(&published)
    .bind(&id)
    .map(|row: PgRow| Post {
        id: row.get("id"),
        title: row.get("title"),
        headline: row.get("headline"),
        slug: row.get("slug"),
        published: row.get("published"),
        created_at: row.get("created_at"),
        edited_at: row.get("edited_at"),
        published_at: row.get("published_at"),
    })
    .fetch_optional(&**pool)
    .await
    {
        Ok(query) => query,
        Err(err) => {
            if let Some(err) = err.as_database_error() {
                // Duplicate on unique constraint status
                if err.code().unwrap() == "23505" {
                    return HttpResponse::Forbidden().json(json!({
                        "message": format!("Post with slug '{slug}' already exists")
                    }));
                } else {
                    log::error!("{err}");
                    return HttpResponse::InternalServerError().finish();
                }
            } else {
                log::error!("{err}"); 
                return HttpResponse::InternalServerError().finish();
            }
        }
    };

    let post = post.unwrap();

    HttpResponse::Created().json(post)
}
