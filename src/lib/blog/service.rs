use crate::models::post::Post;

use super::{errors::GetPostsError, repository::BlogRepository};

#[derive(Clone)]
pub struct BlogService {
    pub repository: BlogRepository,
}

impl BlogService {
    pub fn new(repository: BlogRepository) -> BlogService {
        BlogService { repository }
    }

    pub async fn get_posts(&self) -> Result<Vec<Post>, GetPostsError> {
        match self.repository.get_posts().await {
            Ok(posts) => Ok(posts),
            Err(err) => {
                log::error!("{err}");

                Err(GetPostsError::InternalServerError)
            }
        }
    }
}
