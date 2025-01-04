mod application;
mod repository;
mod service;
mod symbol;
mod transaction;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use log::LevelFilter;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database};
use std::time::Duration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    dotenv::dotenv().ok();
    let address = (get_address(), get_port());

    let mut opt = ConnectOptions::new(database_url());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(LevelFilter::Info);
    // .set_schema_search_path("public"); // Setting default PostgreSQL schema

    let db = Database::connect(opt)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)
        .unwrap();

    Migrator::up(&db, None)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)
        .unwrap();

    /* let transactions = transaction::read_csv("negociacao-2024-12-28-20-51-24.csv");
    let symbols = transactions
        .iter()
        .map(|t| Symbol {
            id: t.negotiation_code.clone(),
            name: t.negotiation_code.clone(),
            last_price: None,
        })
        .collect::<Vec<Symbol>>();
    let orders = transactions
        .iter()
        .map(|t| OrderDto {
            order_id: uuid::Uuid::new_v4().to_string(),
            symbol: t.negotiation_code.clone(),
            order_type: t.transaction_type.clone(),
            quantity: t.quantity as i32,
            price: t.price,
            date: Option::from(t.deal_date),
        })
        .collect::<Vec<OrderDto>>();



    for s in symbols {
        if let Err(e) = SymbolRepository::insert_symbol(&db.clone(), s).await {
            error!("Error to insert symbol {:?}", e)
        }
    }
    for o in orders {
        if let Err(e) = repository::order_repository::insert_order(&db.clone(), o.into()).await {
            eprintln!("Error inserting order: {:?}", e);
        }
    }

    */
    let db_data = web::Data::new(db);
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(db_data.clone())
            .service(application::order_controller::create_order)
            .service(application::order_controller::get_orders)
            .service(application::order_controller::get_order)
            .service(application::order_controller::update_order)
            .service(application::order_controller::delete_order)
            .service(application::position_wallet_controller::get_all_wallets)
            .service(application::position_wallet_controller::create_wallet)
    })
    .bind(address)?
    .run()
    .await
}

fn database_url() -> String {
    dotenv::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://wallet:1234@localhost:5432/wallet".to_string())
        .parse()
        .unwrap()
}

fn get_address() -> String {
    dotenv::var("ADDRESS")
        .unwrap_or("0.0.0.0".to_string())
        .parse()
        .unwrap()
}

fn get_port() -> u16 {
    dotenv::var("PORT")
        .unwrap_or("3000".parse().unwrap())
        .parse()
        .unwrap()
}
