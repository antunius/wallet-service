use crate::repository::symbol_repository::SymbolRepository;
use crate::symbol::Symbol;
use entity::symbol::Model;
use sea_orm::{DatabaseConnection, DbErr};

// Generic CRUD operation for the 'symbol' entity.
pub struct SymbolService;

impl SymbolService {
    pub async fn create(db: &DatabaseConnection, new_symbol: Symbol) -> Result<Model, DbErr> {
        SymbolRepository::insert_symbol(db, new_symbol).await
    }

    pub async fn read_by_id(db: &DatabaseConnection, id: String) -> Result<Option<Model>, DbErr> {
        SymbolRepository::get_symbol(db, id).await
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: String,
        updated_symbol: Symbol,
    ) -> Result<Model, DbErr> {
        SymbolRepository::update_symbol(db, id, updated_symbol).await
    }

    pub async fn delete(db: &DatabaseConnection, id: String) -> Result<bool, DbErr> {
        let deletion_result = SymbolRepository::delete_symbol(db, id).await?;
        Ok(deletion_result.rows_affected > 0)
    }
}
