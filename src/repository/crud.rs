use sea_orm::{
    prelude::*, ActiveModelTrait, DatabaseConnection, DeleteResult, EntityTrait, IntoActiveModel,
};

pub trait Crud<'async_trait, E, ID>
where
    E: EntityTrait + Sync + Send,
    E::Model: IntoActiveModel<E::ActiveModel> + Send,
    E::ActiveModel: ActiveModelTrait + Send,
    ID: Into<<E::PrimaryKey as PrimaryKeyTrait>::ValueType> + Send + Clone + 'async_trait,
{
    /// Create a new record
    async fn create(
        db: &DatabaseConnection,
        active_model: E::ActiveModel,
    ) -> Result<E::Model, DbErr> {
        active_model.insert(db).await
    }

    /// Fetch all records
    async fn read_all(db: &DatabaseConnection) -> Result<Vec<E::Model>, DbErr> {
        E::find().all(db).await
    }

    /// Fetch a record by its primary key
    async fn read_one(db: &DatabaseConnection, id: ID) -> Result<Option<E::Model>, DbErr> {
        E::find_by_id(id).one(db).await
    }

    /// Update a record
    async fn update_one(
        db: &DatabaseConnection,
        active_model: E::ActiveModel,
    ) -> Result<E::Model, DbErr> {
        active_model.update(db).await
    }

    /// Delete a record by its primary key
    async fn delete(db: &DatabaseConnection, id: ID) -> Result<DeleteResult, DbErr> {
        E::delete_by_id(id).exec(db).await
    }
}
