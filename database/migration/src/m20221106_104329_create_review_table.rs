use sea_orm_migration::prelude::*;

use crate::m20221106_103001_create_beer_table::Beer;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Review::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Review::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Review::ReviewerName).string().not_null())
                    .col(ColumnDef::new(Review::ReviewText).string().not_null())
                    .col(ColumnDef::new(Review::Rating).integer().not_null())
                    .col(ColumnDef::new(Review::Date).timestamp().not_null())
                    .col(ColumnDef::new(Review::BeerId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_beer")
                            .from(Review::Table, Review::BeerId)
                            .to(Beer::Table, Beer::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Review::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Review {
    Table,
    Id,
    ReviewerName,
    Rating,
    ReviewText,
    Date,
    BeerId,
}
