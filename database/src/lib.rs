use ::entity::{beer, review};
use entity::beer::Relation;
pub use migration;
pub use migration::sea_orm_migration::MigratorTrait;
use migration::DbErr;
use sea_orm::prelude::Decimal;
use sea_orm::{ActiveModelTrait, ActiveValue, QueryFilter};
use sea_orm::{ColumnTrait, PaginatorTrait};
use sea_orm::{DatabaseTransaction, TransactionTrait};
use sea_orm::{DbConn, EntityTrait, ModelTrait};
use sea_orm::{Set, TryIntoModel};

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
        let txn = db.begin().await?;

        review.beer_id = id;
        let mut new_review = review::ActiveModel::from(review);
        new_review.id = ActiveValue::NotSet;
        new_review.date = ActiveValue::NotSet;
        let new_review = new_review.save(&txn).await?;

        calc_avg_review(&txn, id, *new_review.rating.as_ref()).await?;

        txn.commit().await?;

        Ok(new_review.try_into_model()?)
    }
}

async fn calc_avg_review(
    db: &DatabaseTransaction,
    beer_id: i32,
    new_rating: i32,
) -> Result<(), sea_orm::DbErr> {
    let review_count: Decimal = review::Entity::find()
        .filter(review::Column::BeerId.eq(beer_id))
        .count(db)
        .await?
        .into();

    let reviewed_beer = beer::Entity::find_by_id(beer_id)
        .one(db)
        .await?
        .expect("the reviewed beer to exist");

    let current_average_rating = reviewed_beer.average_rating;
    let old_total = current_average_rating
        .checked_mul(review_count)
        .expect("it to be possible to multiply the two numbers");

    let new_total = old_total
        .checked_add(Decimal::from(new_rating))
        .expect("the additional to be possible");

    let new_average = new_total
        .checked_div(Decimal::from(review_count) + Decimal::from(1))
        .unwrap_or(Decimal::from(0));

    let mut reviewed_beer = beer::ActiveModel::from(reviewed_beer);
    reviewed_beer.average_rating = Set(new_average);

    reviewed_beer.save(db).await?;

    Ok(())
}

pub use sea_orm;
