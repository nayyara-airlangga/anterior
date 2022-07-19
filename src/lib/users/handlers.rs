use actix_web::{http::StatusCode, web, HttpMessage, HttpRequest, HttpResponse};
use jsonwebtoken::TokenData;

use crate::{errors::ErrorResponse, jwt::payload::AuthToken};

use super::UserService;

pub async fn get_self(req: HttpRequest, service: web::Data<UserService>) -> HttpResponse {
    let AuthToken { id, .. } = req
        .extensions()
        .get::<TokenData<AuthToken>>()
        .unwrap()
        .claims;
    let id = id as i32;

    match service.as_ref().get_self(id).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(sqlx::Error::RowNotFound) => {
            ErrorResponse::new(StatusCode::NOT_FOUND, String::from("User not found"))
        }
        Err(err) => {
            log::error!("{err}");

            ErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Internal server error"),
            )
        }
    }
}
