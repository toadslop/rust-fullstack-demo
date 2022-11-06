pub use sea_orm_migration::prelude::*;
mod m20221106_103001_create_beer_table;
mod m20221106_104329_create_review_table;

pub use sea_orm_migration;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221106_103001_create_beer_table::Migration),
            Box::new(m20221106_104329_create_review_table::Migration),
        ]
    }
}
