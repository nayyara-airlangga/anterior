use serde::Serialize;

#[derive(Serialize)]
pub struct Pagination {
    pub has_next: bool,
    pub next_page: Option<i32>,
}

#[derive(Serialize)]
pub struct Metadata {
    pub count: i32,
    pub pagination: Pagination,
}
