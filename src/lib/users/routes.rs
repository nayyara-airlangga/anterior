use actix_web::web;

use super::{handlers::get_self, middlewares::validate_user_token};

pub fn users_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users").service(
            web::resource("/self")
                .wrap_fn(validate_user_token)
                .route(web::get().to(get_self)),
        ),
    );
}
