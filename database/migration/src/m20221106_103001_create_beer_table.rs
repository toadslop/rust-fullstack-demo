use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Beer::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Beer::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Beer::Name).string().not_null())
                    .col(ColumnDef::new(Beer::Description).string().not_null())
                    .col(ColumnDef::new(Beer::Brewery).string().not_null())
                    .col(ColumnDef::new(Beer::BreweryLocation).string().not_null())
                    .col(
                        ColumnDef::new(Beer::AlcoholContent)
                            .decimal_len(3, 1)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Beer::AverageRating)
                            .decimal_len(3, 2)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Beer::ImageUrl).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Beer::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Beer {
    Table,
    Id,
    Name,
    Brewery,
    BreweryLocation,
    AlcoholContent,
    AverageRating,
    Description,
    ImageUrl,
}
