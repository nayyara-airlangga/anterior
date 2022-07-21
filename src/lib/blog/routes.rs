use actix_web::web;

use crate::users::middlewares::{validate_super_user, validate_user_token};

use super::handlers::{get_post_detail, get_posts};

pub fn blog_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/posts")
            .service(web::resource("").route(web::get().to(get_posts)))
            .service(
                web::resource("/create")
                    .wrap_fn(validate_super_user)
                    .wrap_fn(validate_user_token), // .route(web::post().to(create_post)),
            )
            .service(web::resource("/{id}").route(web::get().to(get_post_detail))),
    );
}
