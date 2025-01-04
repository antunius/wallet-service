use crate::repository::crud::Crud;
use entity::position_wallet;
use entity::position_wallet::Model;
use sea_orm::prelude::Decimal;
use sea_orm::sqlx::types::chrono::NaiveDateTime;
use sea_orm::{DatabaseConnection, DbErr, DeleteResult, IntoActiveModel, Set};
use uuid::Uuid;

pub struct PositionWalletRepository;

impl PositionWalletRepository {
    /// Inserts a new position wallet entry.
    pub async fn insert_entry(
        db: &DatabaseConnection,
        id: Uuid,
        codigo_ativo: String,
        quantidade: i32,
        preco_medio: Decimal,
        valor_total: Decimal,
        data_ultima_atualizacao: NaiveDateTime,
    ) -> Result<Model, DbErr> {
        let active_model = position_wallet::ActiveModel {
            id: Set(id),
            codigo_ativo: Set(codigo_ativo),
            quantidade: Set(quantidade),
            preco_medio: Set(preco_medio),
            valor_total: Set(valor_total),
            data_ultima_atualizacao: Set(data_ultima_atualizacao),
        };

        match position_wallet::Entity::read_one(db, id).await {
            Ok(Some(model)) => Ok(model),
            Ok(None) => position_wallet::Entity::create(db, active_model).await,
            Err(err) => Err(err),
        }
    }

    /// Retrieves all position wallet entries.
    pub async fn get_entries(db: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
        position_wallet::Entity::read_all(db).await
    }

    /// Retrieves a single position wallet entry by ID.
    pub async fn get_entry(db: &DatabaseConnection, id: Uuid) -> Result<Option<Model>, DbErr> {
        position_wallet::Entity::read_one(db, id).await
    }

    /// Updates an existing position wallet entry.
    pub async fn update_entry(
        db: &DatabaseConnection,
        id: Uuid,
        new_codigo_ativo: Option<String>,
        new_quantidade: Option<i32>,
        new_preco_medio: Option<Decimal>,
        new_valor_total: Option<Decimal>,
        new_data_ultima_atualizacao: Option<NaiveDateTime>,
    ) -> Result<Model, DbErr> {
        // Fetch the existing entry by its ID
        if let Some(existing_model) = position_wallet::Entity::read_one(db, id).await? {
            let mut active_model = existing_model.into_active_model();

            // Update fields as per new input
            if let Some(codigo_ativo) = new_codigo_ativo {
                active_model.codigo_ativo = Set(codigo_ativo);
            }
            if let Some(quantidade) = new_quantidade {
                active_model.quantidade = Set(quantidade);
            }
            if let Some(preco_medio) = new_preco_medio {
                active_model.preco_medio = Set(preco_medio);
            }
            if let Some(valor_total) = new_valor_total {
                active_model.valor_total = Set(valor_total);
            }
            if let Some(_data_ultima_atualizacao) = new_data_ultima_atualizacao {
                active_model.data_ultima_atualizacao = Set(new_data_ultima_atualizacao.unwrap());
            }

            position_wallet::Entity::update_one(db, active_model).await
        } else {
            Err(DbErr::RecordNotFound(format!(
                "Position wallet entry with id '{}' not found",
                id
            )))
        }
    }

    /// Deletes a position wallet entry by ID.
    pub async fn delete_entry(db: &DatabaseConnection, id: Uuid) -> Result<DeleteResult, DbErr> {
        <entity::prelude::PositionWallet as Crud<position_wallet::Entity, Uuid>>::delete(db, id)
            .await
    }
}

// Implement the generic CRUD trait for position_wallet::Entity
impl Crud<'_, position_wallet::Entity, Uuid> for position_wallet::Entity {}
