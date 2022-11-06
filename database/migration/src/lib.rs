use sea_orm_migration::{
    async_trait::{self},
    MigrationTrait, MigratorTrait,
};

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![]
    }
}
