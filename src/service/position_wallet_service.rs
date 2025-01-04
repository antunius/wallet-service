use crate::application::position_wallet_dto::PositionWalletDto as PositionWalletDto;
use crate::repository::position_wallet_repository;
use entity::position_wallet::Model;
use log;
use position_wallet_repository::PositionWalletRepository;
use sea_orm::{DatabaseConnection, DbErr, DeleteResult};
use uuid::Uuid;

/// Service for managing PositionWallet
pub struct PositionWalletService;

impl PositionWalletService {
    /// Create a new position wallet entry
    pub async fn create_position_wallet(
        db: &DatabaseConnection,
        position_wallet_data: PositionWalletDto,
    ) -> Result<Model, DbErr> {
        log::info!("Creating a new position wallet entry");
        PositionWalletRepository::insert_entry(
            db,
            Uuid::new_v4(),
            position_wallet_data.codigo_ativo,
            position_wallet_data.quantidade,
            position_wallet_data.preco_medio,
            position_wallet_data.valor_total,
            position_wallet_data.data_ultima_atualizacao.into(),
        )
        .await
    }

    /// Retrieve all position wallet entries
    pub async fn fetch_all_positions(db: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
        log::info!("Fetching all position wallet entries");
        PositionWalletRepository::get_entries(db).await
    }

    /// Retrieve a position wallet entry by asset code
    pub async fn fetch_position_by_code(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<Model>, DbErr> {
        log::info!("Fetching position wallet by asset code: {}", id);
        PositionWalletRepository::get_entry(db, id).await
    }

    /// Update an existing position wallet entry
    pub async fn update_position_wallet(
        db: &DatabaseConnection,
        id: Uuid,
        updated_data: PositionWalletDto,
    ) -> Result<Model, DbErr> {
        log::info!("Updating position wallet for asset code: {}", id);
        PositionWalletRepository::update_entry(
            db,
            id,
            Option::from(updated_data.codigo_ativo),
            Option::from(updated_data.quantidade),
            Option::from(updated_data.preco_medio),
            Option::from(updated_data.valor_total),
            Some(updated_data.data_ultima_atualizacao.into()),
        )
        .await
    }

    /// Delete a position wallet entry by asset code
    pub async fn delete_position_wallet(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<DeleteResult, DbErr> {
        log::info!("Deleting position wallet for asset code: {}", id);
        PositionWalletRepository::delete_entry(db, id).await
    }
}
