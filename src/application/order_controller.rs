use crate::application::order_dto::{OrderDto, PartialOrder};
use crate::service;
use actix_web::{delete, get, post, put, web, Error, HttpResponse};
use sea_orm::DatabaseConnection;
use service::order_service::OrderService;

#[post("/order")]
async fn create_order(
    body: web::Json<OrderDto>,
    db: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, Error> {
    match OrderService::create_order(db.get_ref(), body.0).await {
        Ok(order) => Ok(HttpResponse::Created().json(Into::<OrderDto>::into(order))),
        Err(_) => {
            log::error!("Failed to create order");
            Err(actix_web::error::ErrorInternalServerError(
                "Failed to create order",
            ))
        }
    }
}

#[get("/orders")]
async fn get_orders(db: web::Data<DatabaseConnection>) -> Result<HttpResponse, Error> {
    match OrderService::fetch_all_orders(db.get_ref()).await {
        Ok(orders) => Ok(HttpResponse::Ok().json(orders.into_iter().map(OrderDto::from).collect::<Vec<OrderDto>>())),
        Err(e) => {
            log::error!("Failed to fetch orders: {:?}", e);
            Err(actix_web::error::ErrorInternalServerError(
                "Failed to fetch orders",
            ))
        }
    }
}

#[get("/order/{id}")]
async fn get_order(
    order_id: web::Path<String>,
    db: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, Error> {
    match OrderService::fetch_order_by_id(db.get_ref(), order_id.into_inner()).await {
        Ok(Some(order)) => Ok(HttpResponse::Ok().json(Into::<OrderDto>::into(order))),
        Ok(None) => Ok(HttpResponse::NotFound().finish()), // Return 404 if order not found
        Err(e) => {
            log::error!("Failed to fetch order: {:?}", e);
            Err(actix_web::error::ErrorInternalServerError(
                "Failed to fetch order",
            ))
        }
    }
}

#[put("/order/{id}")]
async fn update_order(
    order_id: web::Path<String>,
    payload: web::Json<PartialOrder>,
    db: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, Error> {
    match OrderService::update_order(db.get_ref(), order_id.into_inner(), payload.into_inner())
        .await
    {
        Ok(Some(updated_order)) => Ok(HttpResponse::Ok().json(<entity::order::Model as Into<OrderDto>>::into(updated_order))),
        Ok(None) => Ok(HttpResponse::NotFound().body("Order not found")),
        Err(e) => {
            log::error!("Failed to update order: {:?}", e);
            Err(actix_web::error::ErrorInternalServerError(
                "Failed to update order",
            ))
        }
    }
}

#[delete("/order/{id}")]
async fn delete_order(
    order_id: web::Path<String>,
    db: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, Error> {
    match OrderService::delete_order_by_id(db.get_ref(), order_id.into_inner()).await {
        Ok(deleted) => {
            if deleted.rows_affected > 0 {
                Ok(HttpResponse::Ok().finish()) // Successfully deleted
            } else {
                Ok(HttpResponse::NotFound().finish()) // Order not found
            }
        }
        Err(e) => {
            log::error!("Failed to delete order: {:?}", e);
            Err(actix_web::error::ErrorInternalServerError(
                "Failed to delete order",
            ))
        }
    }
}
