use actix_web::{http::StatusCode, HttpResponse};
use serde_json::json;

pub struct ErrorResponse;

impl ErrorResponse {
    pub fn new(code: StatusCode, message: &str) -> HttpResponse {
        HttpResponse::build(code).json(json!({ "message": message }))
    }
}
