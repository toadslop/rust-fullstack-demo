use csv::ReaderBuilder;
use entity::beer;
use fake::faker::internet::en::Username;
use fake::faker::lorem::en::Sentences;
use fake::Fake;
use include_dir::include_dir;
use rand::Rng;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::prelude::Decimal;
use sea_orm_migration::sea_orm::{ActiveModelTrait, EntityTrait, Set};

#[derive(DeriveMigrationName)]
pub struct Migration {
    pub beer_csv: Option<String>,
}

impl Migration {
    pub fn init_csv(&mut self) {
        self.beer_csv = Some(
            include_dir!("$CARGO_MANIFEST_DIR/data")
                .get_file("beers.csv")
                .unwrap()
                .contents_utf8()
                .unwrap()
                .to_string(),
        );
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let mut rng = rand::rngs::OsRng;
        let beer_csv = self.beer_csv.clone().unwrap();
        let mut reader = ReaderBuilder::new().from_reader(beer_csv.as_bytes());

        for result in reader.deserialize::<beer::Model>() {
            let record = result.map_err(|err| DbErr::Custom(err.to_string()))?;
            let active_record: beer::ActiveModel = record.into();
            let beer = active_record.insert(db).await?;

            let mut ratings_sum = 0;
            let mut ratings_count = 0;

            for _ in 0..rng.gen_range(0..5) {
                let review_text: Vec<String> = Sentences(1..10).fake_with_rng(&mut rng);
                let review = entity::review::ActiveModel {
                    reviewer_name: Set(Username().fake_with_rng(&mut rng)),
                    rating: Set(rng.gen_range(0..5)),
                    review_text: Set(review_text.join(" ")),
                    beer_id: Set(beer.id as i32),
                    ..Default::default()
                }
                .insert(db)
                .await?;

                ratings_sum += review.rating;
                ratings_count += 1;
            }

            let average_rating =
                Decimal::from_f32_retain((ratings_sum as f32) / ratings_count as f32)
                    .unwrap_or(Decimal::from(0));

            let mut active_beer = beer::ActiveModel::from(beer);
            active_beer.average_rating = Set(average_rating);

            active_beer.update(db).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        entity::review::Entity::delete_many().exec(db).await?;
        entity::beer::Entity::delete_many().exec(db).await?;

        Ok(())
    }
}
