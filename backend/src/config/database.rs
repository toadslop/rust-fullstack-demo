use std::time::Duration;

use database::sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

pub async fn get_db_config() -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(
        "postgres://ratebeer_app:password@localhost:5432/ratebeer_clone".to_owned(),
    );
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path("my_schema".into()); // Setting default PostgreSQL schema

    let db = Database::connect(opt).await?;

    Ok(db)
}
