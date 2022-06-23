use sqlx::{migrate::MigrateError, Pool, Postgres};

pub async fn apply_migrations(pool: &Pool<Postgres>) -> Result<(), MigrateError> {
    if cfg!(not(debug_assertions)) {
        log::info!("Applying migrations...");

        match sqlx::migrate!().run(pool).await {
            Ok(()) => {
                log::info!("Migrations applied");
                Ok(())
            }
            Err(err) => {
                log::error!("{err}");
                Err(err)
            }
        }
    } else {
        log::info!("Running in debug, no need to apply migrations in app");
        Ok(())
    }
}
