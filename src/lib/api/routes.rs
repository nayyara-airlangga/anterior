use actix_web::web;

use crate::{blog::routes::blog_routes, users::routes::users_routes};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(users_routes)
            .configure(blog_routes),
    );
}
