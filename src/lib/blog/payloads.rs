use actix_web::HttpResponse;
use serde::Deserialize;

use crate::models::Post;

#[derive(Deserialize, Debug)]
pub struct GetPostsQuery {
    pub limit: Option<i32>,
}

pub struct GetPostsResponse;
impl GetPostsResponse {
    pub fn new(posts: Vec<Post>) -> HttpResponse {
        HttpResponse::Ok().json(posts)
    }
}
