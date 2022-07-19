use actix_web::web;

use crate::users::routes::users_routes;

use super::{auth::config::auth_services, posts::config::posts_services};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(auth_services)
            .configure(users_routes)
            .configure(posts_services),
    );
}
