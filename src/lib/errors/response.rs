use actix_web::{http::StatusCode, HttpResponse};
use serde_json::json;

pub struct ErrorResponse {
    pub code: StatusCode,
    pub message: String,
}

impl ErrorResponse {
    pub fn new(code: StatusCode, message: String) -> HttpResponse {
        HttpResponse::build(code).json(json!({ "message": message }))
    }
}
