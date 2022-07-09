use actix_web::web;

use crate::api::auth::middleware::AuthTokenService;

use super::handlers::me;

pub fn users_services(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .wrap(AuthTokenService)
            .service(web::resource("/me").route(web::get().to(me))),
    );
}
