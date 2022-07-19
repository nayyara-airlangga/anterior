use actix_web::{http::StatusCode, web, HttpMessage, HttpRequest, HttpResponse};
use jsonwebtoken::TokenData;

use crate::{errors::ErrorResponse, jwt::payload::AuthToken};

use super::{
    errors::LoginError,
    payloads::{GetSelfResponse, LoginPayload, LoginResponse},
    UserService,
};

pub async fn get_self(req: HttpRequest, service: web::Data<UserService>) -> HttpResponse {
    let AuthToken { id, .. } = req
        .extensions()
        .get::<TokenData<AuthToken>>()
        .unwrap()
        .claims;
    let id = id as i32;

    match service.as_ref().get_self(id).await {
        Ok(user) => GetSelfResponse::new(user),
        Err(sqlx::Error::RowNotFound) => {
            ErrorResponse::new(StatusCode::NOT_FOUND, "User not found")
        }
        Err(err) => {
            log::error!("{err}");

            ErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
        }
    }
}

pub async fn login(body: web::Json<LoginPayload>, service: web::Data<UserService>) -> HttpResponse {
    match service.as_ref().login(body).await {
        Ok(token) => LoginResponse::new(token),
        Err(err) => match err {
            LoginError::NotFound => ErrorResponse::new(StatusCode::NOT_FOUND, "User not found"),
            LoginError::IncorrectPassword => {
                ErrorResponse::new(StatusCode::FORBIDDEN, "Incorrect password")
            }
            LoginError::InternalServerError => {
                ErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
        },
    }
}
