use super::env::Env;
use database::{
    migration::Migrator,
    sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr},
    MigratorTrait,
};
use std::time::Duration;

pub async fn get_db_config(env: &Env) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(env.get_db_url().to_string());

    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);

    let db = Database::connect(opt).await?;
    Migrator::up(&db, None).await.unwrap();
    Ok(db)
}
