use actix_web::web;

use crate::users::routes::users_routes;

use super::posts::config::posts_services;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(users_routes)
            .configure(posts_services),
    );
}
