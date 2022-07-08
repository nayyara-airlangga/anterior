use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use jsonwebtoken::TokenData;
use serde_json::json;
use sqlx::{Pool, Postgres};

use crate::{jwt::payload::AuthToken, models::user::User};

type DbPool = Pool<Postgres>;

pub async fn me(req: HttpRequest, pool: web::Data<DbPool>) -> HttpResponse {
    let AuthToken { id, .. } = req
        .extensions()
        .get::<TokenData<AuthToken>>()
        .unwrap()
        .claims;
    let id = id as i32;

    let user = match sqlx::query_as::<Postgres, User>(
        "
SELECT id, username, name, email FROM posterior.users
WHERE id = $1
",
    )
    .bind(&id)
    .fetch_optional(&**pool)
    .await
    {
        Ok(query) => query,
        Err(err) => {
            log::error!("{err}");
            return HttpResponse::InternalServerError().finish();
        }
    };

    if let Some(user) = user {
        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::NotFound().json(json!({
            "message": "User not found"
        }))
    }
}
