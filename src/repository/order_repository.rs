use crate::application::order_dto::PartialOrder;
use crate::repository::crud::Crud;
use entity::order;
use entity::order::Model;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    prelude::*, DatabaseConnection, DeleteResult, IntoActiveModel,
};
use std::option::Option;

pub async fn insert_order(
    db: &DatabaseConnection,
    new_order: Model,
) -> Result<order::Model, DbErr> {
    let active_model = order::ActiveModel {
        id: Set(new_order.id.parse().unwrap()),
        symbol_id: Set(new_order.symbol_id.parse().unwrap()),
        r#type: Set(new_order.r#type.to_string()),
        quantity: Set(new_order.quantity),
        price: Set(new_order.price),
        date: Set(new_order.date),
        ..Default::default()
    };
    order::Entity::create(db, active_model).await
}

/// Fetches all orders from the database.
pub async fn fetch_all_orders(db: &DatabaseConnection) -> Result<Vec<order::Model>, DbErr> {
    order::Entity::read_all(db).await
}

/// Fetches a single order by ID.
pub async fn fetch_order_by_id(
    db: &DatabaseConnection,
    order_id: String,
) -> Result<Option<Model>, DbErr> {
    match order::Entity::read_one(db, order_id.parse().unwrap()).await {
        Ok(Some(order)) => Ok(Some(order)),
        Ok(None) => Ok(None),
        Err(err) => Err(err),
    }
}
pub async fn update_order(
    db: &DatabaseConnection,
    order_id: String,
    partial_order_update: PartialOrder,
) -> Result<Option<order::Model>, DbErr> {
    // Introduced helper function to reduce duplicate logic.
    async fn handle_order_fetch(
        db: &DatabaseConnection,
        order: Option<order::Model>,
        partial_order_update: PartialOrder,
    ) -> Result<Option<order::Model>, DbErr> {
        if let Some(existing_order) = order {
            let updated_order = update(db, partial_order_update, Some(existing_order)).await?;
            Ok(Some(updated_order)) // Return updated order
        } else {
            Ok(None) // Order not found
        }
    }

    // Handle potential errors and process appropriately
    match order::Entity::read_one(db, order_id.parse().unwrap()).await {
        Ok(order) => handle_order_fetch(db, order, partial_order_update).await,
        Err(err) => Err(err), // Propagate database errors
    }
}

async fn update(
    db: &DatabaseConnection,
    updated_order: PartialOrder,
    order: Option<Model>,
) -> Result<Model, DbErr> {
    let mut active_model: order::ActiveModel = order.unwrap().into_active_model();

    if let Some(symbol) = updated_order.symbol {
        active_model.symbol_id = Set(symbol);
    }
    if let Some(quantity) = updated_order.quantity {
        active_model.quantity = Set(quantity);
    }
    if let Some(price) = updated_order.price {
        active_model.price = Set(price);
    }
    if let Some(date) = updated_order.date {
        active_model.date = Set(date);
    }
    <entity::order::Entity as Crud<entity::order::Entity, String>>::update_one(db, active_model)
        .await
}

/// Deletes an order by ID from the database.
pub async fn delete_order_by_id(
    db: &DatabaseConnection,
    order_id: String,
) -> Result<DeleteResult, DbErr> {
    <entity::order::Entity as Crud<entity::order::Entity, String>>::delete(db, order_id.into())
        .await
}

impl Crud<'_, entity::order::Entity, String> for entity::order::Entity {}
