use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePostPayload {
    pub title: String,
    pub headline: String,
    pub content: String,
    pub published: Option<bool>,
}
