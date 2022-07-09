use actix_web::{guard, web};

use crate::api::auth::guard::JwtGuard;

use super::handlers::me;

pub fn users_services(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users").service(web::resource("/me").route(web::get().guard(JwtGuard).to(me))),
    );
}
