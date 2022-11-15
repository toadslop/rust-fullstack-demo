pub use sea_orm_migration::prelude::*;
mod m20221106_103001_create_beer_table;
mod m20221106_104329_create_review_table;
mod m20221106_115011_db_seeder;

pub use sea_orm_migration;
use shared::{init_database_url, DATABASE_URL_KEY};
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        std::env::set_var(DATABASE_URL_KEY, init_database_url().as_str());
        vec![
            Box::new(m20221106_103001_create_beer_table::Migration),
            Box::new(m20221106_104329_create_review_table::Migration),
            Box::new(m20221106_115011_db_seeder::Migration),
        ]
    }
}
