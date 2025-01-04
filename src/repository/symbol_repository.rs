use crate::repository::crud::Crud;
use crate::symbol::Symbol;
use entity::symbol;
use entity::symbol::Model;
use sea_orm::{DatabaseConnection, DbErr, DeleteResult, IntoActiveModel, Set};

pub struct SymbolRepository;

impl SymbolRepository {
    pub async fn insert_symbol(
        db: &DatabaseConnection,
        new_symbol: Symbol,
    ) -> Result<Model, DbErr> {
        let default_price = new_symbol.last_price.unwrap_or(0.0);

        let symbol_model = symbol::ActiveModel {
            id: Set(new_symbol.id.clone()),
            name: Set(new_symbol.name.clone()),
            last_price: Set(default_price),
            ..Default::default()
        };

        match symbol::Entity::read_one(db, new_symbol.id.parse().unwrap()).await {
            Ok(Some(model)) => Ok(model),
            Ok(None) => symbol::Entity::create(db, symbol_model).await,
            Err(err) => Err(err),
        }
    }

    pub async fn get_symbols(db: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
        symbol::Entity::read_all(db).await
    }
    pub async fn get_symbol(db: &DatabaseConnection, id: String) -> Result<Option<Model>, DbErr> {
        symbol::Entity::read_one(db, id).await
    }

    pub async fn delete_symbol(db: &DatabaseConnection, id: String) -> Result<DeleteResult, DbErr> {
        <entity::prelude::Symbol as Crud<symbol::Entity, String>>::delete(db, id).await
    }
    pub async fn update_symbol(
        db: &DatabaseConnection,
        id: String,
        new_symbol: Symbol,
    ) -> Result<Model, DbErr> {
        if let Some(existing_symbol) = symbol::Entity::read_one(db, id.clone()).await? {
            let mut active_model: symbol::ActiveModel = existing_symbol.into_active_model();
            if let Some(last_price) = new_symbol.last_price {
                active_model.last_price = Set(last_price);
            }
            active_model.name = Set(new_symbol.name);
            symbol::Entity::update_one(db, active_model).await
        } else {
            Err(DbErr::RecordNotFound(format!(
                "Symbol with id '{}' not found",
                id
            )))
        }
    }
}
impl Crud<'_, symbol::Entity, String> for symbol::Entity {}
