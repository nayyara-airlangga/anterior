use actix_web::main;

use dotenv::dotenv;
use sqlx::error::BoxDynError;

use posterior::{
    db::{connection::create_connection, migrate::apply_migrations},
    log::init_logger,
};

#[main]
async fn main() -> Result<(), BoxDynError> {
    // Init env vars
    dotenv().ok();

    // Init logger
    init_logger();

    let pool = create_connection(8).await?;

    // Run migrations to db
    apply_migrations(&pool).await?;

    Ok(())
}
