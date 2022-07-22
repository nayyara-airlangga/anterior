use actix_web::web;

use crate::models::{Metadata, Pagination, PostDetail, PostsWithMeta};

use super::{
    errors::{CreatePostError, GetPostDetailError, GetPostsError},
    payloads::{CreatePostRequest, GetPostsQuery},
    repository::BlogRepository,
};

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
            Err(sqlx::Error::RowNotFound) => return Err(GetPostsError::InvalidCursor),
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

    pub async fn get_post_detail(&self, slug: String) -> Result<PostDetail, GetPostDetailError> {
        match self.repository.get_post_by_slug(&slug).await {
            Ok(post) => Ok(post),
            Err(sqlx::Error::RowNotFound) => Err(GetPostDetailError::PostNotFound),
            Err(err) => {
                log::error!("{err}");

                Err(GetPostDetailError::InternalServerError)
            }
        }
    }

    pub async fn create_post(
        &self,
        author_id: i32,
        body: web::Json<CreatePostRequest>,
    ) -> Result<(), CreatePostError> {
        if body.title.trim().len() == 0 {
            return Err(CreatePostError::BadRequest("Title can't be empty"));
        }

        if body.headline.trim().len() == 0 {
            return Err(CreatePostError::BadRequest("Headline can't be empty"));
        }

        if body.content.trim().len() == 0 {
            return Err(CreatePostError::BadRequest("Content can't be empty"));
        }

        let count = match self
            .repository
            .get_post_count_with_duplicate_title(&body.title)
            .await
        {
            Ok(count) => count,
            Err(err) => {
                log::error!("{err}");

                return Err(CreatePostError::InternalServerError);
            }
        };

        let mut slug = slug::slugify(&body.title);

        if count > 0 {
            slug.push_str(&format!("-{}", count));
        }

        match self
            .repository
            .insert_post(body.into_inner(), &slug, author_id)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => {
                log::error!("{err}");

                Err(CreatePostError::InternalServerError)
            }
        }
    }
}
