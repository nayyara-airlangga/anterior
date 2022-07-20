use actix_web::{guard, web};

use super::{
    handlers::{get_self, login, register},
    middlewares::validate_user_token,
};

pub fn users_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(
                web::resource("/self")
                    .wrap_fn(validate_user_token)
                    .route(web::get().to(get_self)),
            )
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
