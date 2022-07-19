use actix_web::web;

use super::{handlers::me, middlewares::validate_user_token};

pub fn users_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .wrap_fn(validate_user_token)
            .service(web::resource("/me").route(web::get().to(me))),
    );
}
