pub enum GetPostsError {
    InternalServerError,
    InvalidCursor,
}

pub enum GetPostDetailError {
    InternalServerError,
    PostNotFound,
}

pub enum CreatePostError<'a> {
    InternalServerError,
    BadRequest(&'a str),
}
