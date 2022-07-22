use actix_web::HttpResponse;
use serde::Deserialize;
use serde_json::json;

use crate::models::{PostDetail, PostsWithMeta};

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct GetPostsQuery {
    pub limit: i32,
    pub cursor: Option<i32>,
}
impl Default for GetPostsQuery {
    fn default() -> Self {
        GetPostsQuery {
            limit: 10,
            cursor: None,
        }
    }
}

pub struct GetPostsResponse;
impl GetPostsResponse {
    pub fn new(posts: PostsWithMeta) -> HttpResponse {
        HttpResponse::Ok().json(posts)
    }
}

pub struct GetPostDetailResponse;
impl GetPostDetailResponse {
    pub fn new(post: PostDetail) -> HttpResponse {
        HttpResponse::Ok().json(post)
    }
}

#[derive(Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub headline: String,
    pub content: String,
    pub published: Option<bool>,
}

pub struct CreatePostResponse;
impl CreatePostResponse {
    pub fn new() -> HttpResponse {
        HttpResponse::Created().json(json!({
            "message": "Post created successfully"
        }))
    }
}
