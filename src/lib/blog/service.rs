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
        let offset = (query.page - 1) * query.limit;

        let mut posts = match self.repository.get_posts(query.limit + 1, offset).await {
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

        let metadata = Metadata {
            count: posts.len() as i32,
            pagination: Pagination {
                has_next,
                next_page: if has_next { Some(query.page + 1) } else { None },
            },
        };

        Ok(PostsWithMeta::new(posts, metadata))
    }
}
