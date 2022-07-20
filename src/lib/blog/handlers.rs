use actix_web::{http::StatusCode, web, HttpResponse};

use crate::errors::ErrorResponse;

use super::{errors::GetPostsError, payloads::GetPostsResponse, BlogService};

pub async fn get_posts(service: web::Data<BlogService>) -> HttpResponse {
    match service.as_ref().get_posts().await {
        Ok(posts) => GetPostsResponse::new(posts),
        Err(err) => match err {
            GetPostsError::InternalServerError => {
                ErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
        },
    }
}
