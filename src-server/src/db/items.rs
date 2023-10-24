//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "Items")]
pub struct Model {
    #[sea_orm(column_name = "Id", primary_key)]
    pub id: i32,
    #[sea_orm(column_name = "Name")]
    pub name: String,
    #[sea_orm(column_name = "Level")]
    pub level: i32,
    #[sea_orm(column_name = "CanBeHQ")]
    pub can_be_hq: i32,
    #[sea_orm(column_name = "CategoryId")]
    pub category_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::categories::Entity",
        from = "Column::CategoryId",
        to = "super::categories::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Categories,
    #[sea_orm(has_many = "super::item_with_amount::Entity")]
    ItemWithAmount,
}

impl Related<super::categories::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Categories.def()
    }
}

impl Related<super::item_with_amount::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ItemWithAmount.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
