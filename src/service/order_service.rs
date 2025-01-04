use crate::application::order_dto::{OrderDto, PartialOrder};
use crate::repository::order_repository;
use entity::order;
use log;
use sea_orm::{DatabaseConnection, DbErr, DeleteResult};

pub struct OrderService;

impl OrderService {
    /// Create a new order
    pub async fn create_order(
        db: &DatabaseConnection,
        order_data: OrderDto,
    ) -> Result<order::Model, DbErr> {
        // Call repository to insert the order
        log::info!("Creating a new order");
        order_repository::insert_order(db, order_data.into()).await
    }

    /// Retrieve all orders
    pub async fn fetch_all_orders(db: &DatabaseConnection) -> Result<Vec<order::Model>, DbErr> {
        log::info!("Fetching all orders");
        order_repository::fetch_all_orders(db).await
    }

    /// Retrieve order by ID
    pub async fn fetch_order_by_id(
        db: &DatabaseConnection,
        order_id: String,
    ) -> Result<Option<order::Model>, DbErr> {
        log::info!("Fetching order with ID: {}", order_id);
        order_repository::fetch_order_by_id(db, order_id).await
    }

    /// Update an existing order
    pub async fn update_order(
        db: &DatabaseConnection,
        order_id: String,
        update_data: PartialOrder,
    ) -> Result<Option<order::Model>, DbErr> {
        log::info!("Updating order with ID: {}", order_id);
        order_repository::update_order(db, order_id, update_data).await
    }

    /// Delete order by ID
    pub async fn delete_order_by_id(
        db: &DatabaseConnection,
        order_id: String,
    ) -> Result<DeleteResult, DbErr> {
        log::info!("Deleting order with ID: {}", order_id);
        order_repository::delete_order_by_id(db, order_id).await
    }
}
