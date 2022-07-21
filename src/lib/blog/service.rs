use actix_web::web;

use crate::models::{Metadata, Pagination, PostsWithMeta};

use super::{errors::GetPostsError, payloads::GetPostsQuery, repository::BlogRepository};

#[derive(Clone)]
pub struct BlogService {
    pub repository: BlogRepository,
}

impl BlogService {
    pub fn new(repository: BlogRepository) -> BlogService {
        BlogService { repository }
    }

    pub async fn get_posts(
        &self,
        query: web::Query<GetPostsQuery>,
    ) -> Result<PostsWithMeta, GetPostsError> {
        let mut posts = match self
            .repository
            .get_posts(query.limit + 1, query.cursor)
            .await
        {
            Ok(posts) => posts,
            Err(err) => {
                log::error!("{err}");

                return Err(GetPostsError::InternalServerError);
            }
        };

        let mut has_next = false;

        if posts.len() > query.limit as usize {
            has_next = true;
            posts = posts.into_iter().take(query.limit as usize).collect();
        }

        let post_count = posts.len();
        let cursor = if post_count > 0 {
            Some(posts.get(post_count - 1).unwrap().id)
        } else {
            None
        };

        let metadata = Metadata {
            count: post_count as i32,
            pagination: Pagination { has_next, cursor },
        };

        Ok(PostsWithMeta::new(posts, metadata))
    }
}
