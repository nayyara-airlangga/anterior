use std::{env, io::Result, process};

use actix_web::{middleware, web, App, HttpServer};
use sqlx::PgPool;

use crate::{
    api::{cors::config_cors, routes::routes},
    users::{UserRepository, UserService},
};

pub async fn run_server(pool: PgPool) -> Result<()> {
    let host = env::var("HOST").unwrap_or_else(|err| {
        log::error!("{err}");
        process::exit(1)
    });
    let port = env::var("PORT").unwrap_or_else(|err| {
        log::error!("{err}");
        process::exit(1)
    });

    let user_repository = UserRepository::new(pool.clone());
    let user_service = UserService::new(user_repository.clone());

    let server = match HttpServer::new(move || {
        let cors = config_cors();

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(user_service.clone()))
            .configure(routes)
    })
    .bind(format!("{host}:{port}"))
    {
        Ok(server) => {
            log::info!("Server successfully created");
            server
        }
        Err(err) => {
            log::error!("{err}");
            process::exit(1)
        }
    };

    log::info!("Running server...");
    server.run().await
}
