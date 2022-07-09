use actix_web::{guard, web};

use super::handlers::{login, register};

pub fn auth_services(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(
                web::resource("/login").route(
                    web::post()
                        .guard(guard::Header("Content-Type", "application/json"))
                        .to(login),
                ),
            )
            .service(
                web::resource("/register").route(
                    web::post()
                        .guard(guard::Header("Content-Type", "application/json"))
                        .to(register),
                ),
            ),
    );
}
