use actix_web::main;

use dotenv::dotenv;

use anterior::log::init_logger;
use sqlx::error::BoxDynError;

#[main]
async fn main() -> Result<(), BoxDynError> {
    // Init env vars
    dotenv().ok();

    // Init logger
    init_logger();

    // Run migrations on release
    if cfg!(not(debug_assertions)) {
        sqlx::migrate!();
    }

    Ok(())
}
