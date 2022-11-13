use ::entity::{beer, beer::Entity as Beer, review};
use entity::beer::Relation;
pub use entity::prelude::Review;
pub use migration;
pub use migration::sea_orm_migration::MigratorTrait;
use migration::DbErr;
pub use sea_orm;
use sea_orm::{DbConn, EntityTrait, ModelTrait};

pub struct BeerQueries;

impl BeerQueries {
    pub async fn find_all(db: &DbConn) -> Result<Vec<beer::Model>, DbErr> {
        Beer::find().all(db).await
    }

    pub async fn find_one(db: &DbConn, id: i32) -> Result<Option<beer::Model>, DbErr> {
        Beer::find_by_id(id).one(db).await
    }

    pub async fn find_with_related(
        db: &DbConn,
        id: i32,
        relation: Relation,
    ) -> Result<(Option<beer::Model>, Vec<review::Model>), sea_orm::DbErr> {
        let beer = Beer::find_by_id(id).one(db).await?;

        let reviews = if let Some(beer) = &beer {
            match relation {
                Relation::Review => beer.find_related(Review).all(db).await?,
            }
        } else {
            vec![]
        };

        Ok((beer, reviews))
    }
}
