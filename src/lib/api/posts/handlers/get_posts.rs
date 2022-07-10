use actix_web::{web, HttpResponse};
use sqlx::{Pool, Postgres};

type DbPool = Pool<Postgres>;

pub async fn get_posts(_pool: web::Data<DbPool>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
