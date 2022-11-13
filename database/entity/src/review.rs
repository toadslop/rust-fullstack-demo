//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "review")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub reviewer_name: String,
    pub review_text: String,
    pub rating: i32,
    pub date: Option<DateTime>,
    pub beer_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::beer::Entity",
        from = "Column::BeerId",
        to = "super::beer::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Beer,
}

impl Related<super::beer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Beer.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
