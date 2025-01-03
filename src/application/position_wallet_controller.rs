use crate::application::position_wallet_dto::PositionWalletDto;
use crate::service::position_wallet_service::PositionWalletService;
use actix_web::{delete, get, post, put, web, Error, HttpResponse, Responder};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

#[post("/position")]
async fn create_wallet(
    db: web::Data<DatabaseConnection>,
    wallet: web::Json<PositionWalletDto>,
) -> Result<HttpResponse, Error> {
    match PositionWalletService::create_position_wallet(&db, wallet.into_inner()).await {
        Ok(created_wallet) => {
            Ok(HttpResponse::Created().json(Into::<PositionWalletDto>::into(created_wallet)))
        }
        Err(err) => Ok(HttpResponse::InternalServerError().body({ "Error" })),
    }
}

#[get("/position")]
async fn get_all_wallets(db: web::Data<DatabaseConnection>) -> impl Responder {
    match PositionWalletService::fetch_all_positions(db.get_ref()).await {
        Ok(wallets) => HttpResponse::Ok().json(
            wallets
                .into_iter()
                .map(Into::into)
                .collect::<Vec<PositionWalletDto>>(),
        ),
        Err(err) => HttpResponse::InternalServerError().body({ "Error" }),
    }
}

#[get("/position/{id}")]
async fn get_wallet(db: web::Data<DatabaseConnection>, path: web::Path<Uuid>) -> impl Responder {
    let id = path.into_inner();
    match PositionWalletService::fetch_position_by_code(db.as_ref(), id).await {
        Ok(wallet) => HttpResponse::Ok().json(wallet.map(Into::<PositionWalletDto>::into)),
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}

#[put("/position/{id}")]
async fn update_wallet(
    db: web::Data<DatabaseConnection>,
    path: web::Path<Uuid>,
    wallet: web::Json<PositionWalletDto>,
) -> Result<HttpResponse, Error> {
    let id = path.into_inner();
    match PositionWalletService::update_position_wallet(db.get_ref(), id, wallet.into_inner()).await
    {
        Ok(updated_wallet) => {
            Ok(HttpResponse::Ok().json(Into::<PositionWalletDto>::into(updated_wallet)))
        }
        Err(err) => Ok(HttpResponse::InternalServerError().body(err.to_string())),
    }
}

#[delete("/position/{id}")]
async fn delete_wallet(db: web::Data<DatabaseConnection>, path: web::Path<Uuid>) -> impl Responder {
    let id = path.into_inner();
    match PositionWalletService::delete_position_wallet(db.get_ref(), id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
