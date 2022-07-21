use actix_web::{http::StatusCode, web, HttpMessage, HttpRequest, HttpResponse};
use jsonwebtoken::TokenData;

use crate::{errors::ErrorResponse, jwt::payload::AuthToken};

use super::{
    errors::{CreatePostError, GetPostDetailError, GetPostsError},
    payloads::{
        CreatePostPayload, CreatePostResponse, GetPostDetailResponse, GetPostsQuery,
        GetPostsResponse,
    },
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

pub async fn create_post(
    req: HttpRequest,
    service: web::Data<BlogService>,
    body: web::Json<CreatePostPayload>,
) -> HttpResponse {
    let AuthToken { id: author_id, .. } = req
        .extensions()
        .get::<TokenData<AuthToken>>()
        .unwrap()
        .claims;

    match service.as_ref().create_post(author_id as i32, body).await {
        Ok(_) => CreatePostResponse::new(),
        Err(CreatePostError::InternalServerError) => {
            ErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal sever error")
        }
        Err(CreatePostError::BadRequest(err)) => ErrorResponse::new(StatusCode::BAD_REQUEST, err),
    }
}
