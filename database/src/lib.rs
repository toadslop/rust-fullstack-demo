use ::entity::{beer, review};
use entity::beer::Relation;
pub use migration;
pub use migration::sea_orm_migration::MigratorTrait;
use migration::DbErr;
use sea_orm::ActiveModelTrait;
use sea_orm::{DbConn, EntityTrait, ModelTrait};

pub struct BeerQueries;

impl BeerQueries {
    pub async fn find_all(db: &DbConn) -> Result<Vec<beer::Model>, DbErr> {
        beer::Entity::find().all(db).await
    }

    pub async fn find_one(db: &DbConn, id: i32) -> Result<Option<beer::Model>, DbErr> {
        beer::Entity::find_by_id(id).one(db).await
    }

    pub async fn find_with_related(
        db: &DbConn,
        id: i32,
        relation: Relation,
    ) -> Result<(Option<beer::Model>, Vec<review::Model>), sea_orm::DbErr> {
        let beer = beer::Entity::find_by_id(id).one(db).await?;

        let reviews = if let Some(beer) = &beer {
            match relation {
                Relation::Review => beer.find_related(review::Entity).all(db).await?,
            }
        } else {
            vec![]
        };

        Ok((beer, reviews))
    }

    pub async fn add_review(
        db: &DbConn,
        id: i32,
        mut review: review::Model,
    ) -> Result<review::Model, DbErr> {
        review.beer_id = id;
        let new_review = review::ActiveModel::from(review);

        new_review.insert(db).await
    }
}

pub use sea_orm;
