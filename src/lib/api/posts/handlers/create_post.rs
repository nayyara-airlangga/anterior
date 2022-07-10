use actix_web::{web, HttpResponse};
use sqlx::{Pool, Postgres};

type DbPool = Pool<Postgres>;

pub async fn create_post(_pool: web::Data<DbPool>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
