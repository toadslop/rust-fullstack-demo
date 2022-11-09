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

    pub async fn find_one(db: &DbConn, id: i32) -> Result<Option<beer::Model>, DbErr> {
        let beer = Beer::find_by_id(id).one(db).await?;
        Ok(beer)
    }
}
