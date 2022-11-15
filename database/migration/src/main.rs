use migration::Migrator;
use sea_orm_migration::prelude::*;
use shared::{init_database_url, DATABASE_URL_KEY};

#[async_std::main]
async fn main() {
    std::env::set_var(DATABASE_URL_KEY, init_database_url().as_str());
    cli::run_cli(Migrator).await;
}
