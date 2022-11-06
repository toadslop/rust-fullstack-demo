pub use migration;
pub use migration::sea_orm_migration::MigratorTrait;
use migration::DbErr;
pub use sea_orm;

use ::entity::{beer, beer::Entity as Beer};
use sea_orm::{DbConn, EntityTrait};

pub struct BeerQueries;

impl BeerQueries {
    pub async fn find_all(db: &DbConn) -> Result<Vec<beer::Model>, DbErr> {
        let beers = Beer::find().all(db).await?;
        Ok(beers)
    }
}
