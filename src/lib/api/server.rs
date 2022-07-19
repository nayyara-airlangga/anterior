use std::{env, io::Result, process};

use actix_web::{middleware, web, App, HttpServer};
use sqlx::{Pool, Postgres};

use crate::api::{cors::config_cors, routes::routes};

pub async fn run_server(pool: Pool<Postgres>) -> Result<()> {
    let port = env::var("PORT")
        .unwrap_or_else(|err| {
            log::error!("{err}");
            process::exit(1)
        })
        .parse::<u16>()
        .unwrap_or_else(|err| {
            log::error!("{err}");
            process::exit(1)
        });

    let server = match HttpServer::new(move || {
        let cors = config_cors();

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .configure(routes)
    })
    .bind(("127.0.0.1", port))
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
