use actix_web::{http::StatusCode, web, HttpResponse};

use crate::errors::ErrorResponse;

use super::{
    errors::{GetPostDetailError, GetPostsError},
    payloads::{GetPostDetailResponse, GetPostsQuery, GetPostsResponse},
    BlogService,
};

pub async fn get_posts(
    service: web::Data<BlogService>,
    query: web::Query<GetPostsQuery>,
) -> HttpResponse {
    match service.as_ref().get_posts(query).await {
        Ok(posts) => GetPostsResponse::new(posts),
        Err(GetPostsError::InternalServerError) => {
            ErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
        }
    }
}

pub async fn get_post_detail(
    service: web::Data<BlogService>,
    path: web::Path<String>,
) -> HttpResponse {
    let slug = path.into_inner();

    match service.as_ref().get_post_detail(slug).await {
        Ok(post) => GetPostDetailResponse::new(post),
        Err(GetPostDetailError::PostNotFound) => {
            ErrorResponse::new(StatusCode::NOT_FOUND, "Post not found")
        }
        Err(GetPostDetailError::InternalServerError) => {
            ErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
        }
    }
}
