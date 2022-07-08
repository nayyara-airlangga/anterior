use actix_web::{http::header::ContentType, web, HttpResponse};

use super::payloads::RegisterPayload;

pub async fn register(body: web::Json<RegisterPayload>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(body)
}
