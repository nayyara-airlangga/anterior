use actix_web::web;

use crate::api::auth::middleware::AuthToken;

use super::handlers::{create_post::create_post, get_posts::get_posts};

pub fn posts_services(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/posts")
            .service(web::resource("").route(web::get().to(get_posts)))
            .service(
                web::resource("/create")
                    .wrap(AuthToken)
                    .route(web::post().to(create_post)),
            ),
    );
}
