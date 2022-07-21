use actix_web::HttpResponse;
use serde::Deserialize;

use crate::models::PostsWithMeta;

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
