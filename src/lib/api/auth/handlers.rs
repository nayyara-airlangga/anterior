use actix_web::{web, HttpResponse};
use fancy_regex::Regex;
use serde_json::json;
use sqlx::{Pool, Postgres};

use crate::{
    crypto::hash::{create_hash, verify_hash},
    jwt::{handlers::create_auth_token, payload::AuthToken},
    models::user::UserWithPassword,
};

use super::payloads::{LoginPayload, RegisterPayload};

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

    if let Some(user) = user {
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
    } else {
        HttpResponse::NotFound().json(json!({
        "message": "User not found"
        }))
    }
}

pub async fn register(body: web::Json<RegisterPayload>, pool: web::Data<DbPool>) -> HttpResponse {
    let user = match sqlx::query_as::<Postgres, UserWithPassword>(
        "SELECT * FROM posterior.users WHERE username = $1 OR email = $2",
    )
    .bind(&body.username)
    .bind(&body.email)
    .fetch_optional(&**pool)
    .await
    {
        Ok(query) => query,
        Err(err) => {
            log::error!("{err}");
            return HttpResponse::InternalServerError().finish();
        }
    };

    if let Some(_) = user {
        HttpResponse::Forbidden().json(json!({
        "message": "User already exists"
        }))
    } else {
        let username_regex = Regex::new(r"^[A-Za-z0-9._-]{5,}$").unwrap();

        if !username_regex.is_match(&body.username).unwrap() {
            return HttpResponse::BadRequest().json(json!({
                "message": "Username can only contain letters, numbers, dots, dash, and underscores and has at least 5 characters"
            }));
        }

        if body.email.trim().len() == 0 || body.name.trim().len() == 0 {
            return HttpResponse::BadRequest().json(json!({
                "message": "Email and name can't be empty"
            }));
        }

        let password_regex =
            Regex::new(r"^(?=.*?[A-Z])(?=.*?[a-z])(?=.*?[0-9])(?=.*?[#?!@$%^&*-]).{8,24}$")
                .unwrap();

        if !password_regex.is_match(&body.password).unwrap() {
            return HttpResponse::BadRequest().json(json!({
                "message": "Password must be 8-24 characters, contain at least one upper and lowercase alphabets, one numeric character, and one special character"
            }));
        }

        let password_hash = create_hash(&body.password);

        let user = match sqlx::query_as::<Postgres, UserWithPassword>(
            "
INSERT INTO posterior.users (username, name, email, password)
VALUES($1, $2, $3, $4)
RETURNING *
",
        )
        .bind(&body.username)
        .bind(&body.name)
        .bind(&body.email)
        .bind(&password_hash)
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
        } else {
            HttpResponse::InternalServerError().finish()
        }
    }
}
