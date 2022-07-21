pub enum GetPostsError {
    InternalServerError,
}

pub enum GetPostDetailError {
    InternalServerError,
    PostNotFound,
}

pub enum CreatePostError {
    InternalServerError,
}
