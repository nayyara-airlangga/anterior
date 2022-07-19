use actix_web::web;

use crate::api::auth::middlewares::auth_token::AuthToken;

use super::handlers::me;

pub fn users_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .wrap(AuthToken)
            .service(web::resource("/me").route(web::get().to(me))),
    );
}
