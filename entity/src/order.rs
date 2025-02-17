//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "order")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub symbol_id: String,
    pub r#type: String,
    #[sea_orm(column_type = "Float")]
    pub price: f32,
    pub quantity: i32,
    pub date: Date,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::symbol::Entity",
        from = "Column::SymbolId",
        to = "super::symbol::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Symbol,
}

impl Related<super::symbol::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Symbol.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
