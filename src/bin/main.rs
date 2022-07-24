use actix_web;

use dotenv::dotenv;
use sqlx::error::BoxDynError;

use libosiris::{
    api::server::run_server,
    db::{connection::create_connection, migrate::apply_migrations},
    logger::init_logger,
};

#[actix_web::main]
async fn main() -> Result<(), BoxDynError> {
    // Init env vars
    dotenv().ok();

    // Init logger
    init_logger();

    let pool = create_connection(8).await?;

    // Run migrations to db
    apply_migrations(&pool).await?;

    // Start server and services
    run_server(pool).await?;

    Ok(())
}
