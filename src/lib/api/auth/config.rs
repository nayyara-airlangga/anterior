use actix_web::{guard, web};

use super::handlers::{login, register};

pub fn auth_services(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(
                web::resource("/login").route(
                    web::route()
                        .guard(guard::Post())
                        .guard(guard::Header("Content-Type", "application/json"))
                        .to(login),
                ),
            )
            .service(
                web::resource("/register").route(
                    web::route()
                        .guard(guard::Post())
                        .guard(guard::Header("Content-Type", "application/json"))
                        .to(register),
                ),
            ),
    );
}
