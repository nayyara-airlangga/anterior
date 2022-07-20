pub enum LoginError {
    InternalServerError,
    IncorrectPassword,
    UserNotFound,
}

pub enum RegisterError<'a> {
    InternalServerError,
    BadRequest(&'a str),
    UserAlreadyExists,
}
