use actix_web::HttpResponse;

use crate::models::post::Post;

pub struct GetPostsResponse;

impl GetPostsResponse {
    pub fn new(posts: Vec<Post>) -> HttpResponse {
        HttpResponse::Ok().json(posts)
    }
}
