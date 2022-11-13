use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::prelude::Decimal;
use sea_orm_migration::sea_orm::{ActiveModelTrait, EntityTrait, Set};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let stone = entity::beer::ActiveModel {
            name: Set("Stone IPA".to_owned()),
            brewery: Set("Stone Brewing".to_owned()),
            brewery_location: Set("Escondido, California, United States".to_owned()),
            alcohol_content: Set(Decimal::new(69, 1)),
            average_rating: Set(Decimal::new(33, 1)),
            image_url: Set("https://res.cloudinary.com/ratebeer/image/upload/d_beer_img_default.png,f_auto/beer_422".to_owned()),
            description: Set("By definition, an India Pale Ale is hoppier and higher in alcohol than its little brother, pale ale-and we deliver in spades. Now one of the most well respected and best-selling IPAs in the country, this golden beauty explodes with citrusy flavor and hop aromas, all perfectly balanced by a subtle malt character. This crisp, extra hoppy brew is hugely refreshing on a hot day, but will always deliver no matter when you choose to drink it.".to_owned()),
            style: Set("IPA".to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        entity::review::ActiveModel {
            reviewer_name: Set("therock2011(333)".to_owned()),
            rating: Set(3),
            review_text: Set("Golden colour has a citrus smell anice citrus flavour with a nice happy citrus kick".to_owned()),
            beer_id: Set(stone.id as i32),
            ..Default::default()
        }
        .insert(db)
        .await?;

        entity::review::ActiveModel {
            reviewer_name: Set("p0rkch0p".to_owned()),
            rating: Set(4),
            review_text: Set("Pour from can is dark plum with faint frothy purple foam head. Probably not consumed at a cool enough temperature  but tastes just fine at room temp. Obviously berry sweet but not too much. Great carbonation with lively fizzy sound. Labeled as IPA but you wouldnt know unless they told you. Not a session beer but perfect for pairing with a meal or post meal.".to_owned()),
            beer_id: Set(stone.id as i32),
            ..Default::default()
        }
        .insert(db)
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        entity::review::Entity::delete_many().exec(db).await?;
        entity::beer::Entity::delete_many().exec(db).await?;

        Ok(())
    }
}
