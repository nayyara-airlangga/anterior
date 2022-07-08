use actix_web::{web, HttpResponse};
use serde_json::json;
use sqlx::{Pool, Postgres};

use crate::{
    crypto::hash::verify_hash,
    jwt::{handlers::create_auth_token, payload::AuthToken},
    models::user::UserWithPassword,
};

use super::payloads::LoginPayload;

type DbPool = Pool<Postgres>;

pub async fn login(body: web::Json<LoginPayload>, pool: web::Data<DbPool>) -> HttpResponse {
    let user = match sqlx::query_as::<Postgres, UserWithPassword>(
        "SELECT * FROM posterior.users WHERE username = $1 OR email = $2",
    )
    .bind(&body.username)
    .bind(&body.username)
    .fetch_optional(&**pool)
    .await
    {
        Ok(query) => query,
        Err(err) => {
            log::error!("{err}");
            return HttpResponse::InternalServerError().finish();
        }
    };

    match user {
        None => HttpResponse::NotFound().json(json!({
        "message": "User not found"
        })),
        Some(user) => {
            if !verify_hash(&body.password, &user.password) {
                return HttpResponse::Forbidden().json(json!({
                "message": "Incorrect password"
                }));
            }

            let exp = if let Some(remember_me) = body.remember_me {
                if remember_me {
                    (chrono::offset::Local::now() + chrono::Duration::days(30)).timestamp()
                } else {
                    (chrono::offset::Local::now() + chrono::Duration::days(7)).timestamp()
                }
            } else {
                (chrono::offset::Local::now() + chrono::Duration::days(7)).timestamp()
            };

            let payload = AuthToken::new(user.id as u64, user.username, user.name, exp);

            match create_auth_token(&payload) {
                Ok(token) => HttpResponse::Created().json(json!({ "token": token })),
                Err(err) => {
                    log::error!("{err}");
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
    }
}
