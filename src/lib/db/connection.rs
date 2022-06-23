use std::env;

use sqlx::{error::BoxDynError, postgres::PgPoolOptions, Pool, Postgres};

use crate::db::error::ZeroConnection;

pub async fn create_connection(num: u32) -> Result<Pool<Postgres>, BoxDynError> {
    if num == 0 {
        log::error!("{}", ZeroConnection);
        return Err(Box::new(ZeroConnection));
    }

    log::info!("Creating pool with {num} connections...");

    let url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(err) => {
            log::error!("{err}");
            return Err(Box::new(err));
        }
    };

    match PgPoolOptions::new()
        .max_connections(num)
        .connect(&url)
        .await
    {
        Ok(pool) => {
            log::info!("Db pool created");
            Ok(pool)
        }
        Err(err) => {
            log::error!("{err}");
            Err(Box::new(err))
        }
    }
}
