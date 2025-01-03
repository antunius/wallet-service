use entity::position_wallet::Model;
use sea_orm::prelude::{DateTime, Decimal};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PositionWalletDto {
    pub codigo_ativo: String,
    pub quantidade: i32,
    pub preco_medio: Decimal,
    pub valor_total: Decimal,
    pub data_ultima_atualizacao: DateTime,
}

impl From<PositionWalletDto> for Model {
    fn from(value: PositionWalletDto) -> Self {
        Self {
            id: Default::default(),
            codigo_ativo: value.codigo_ativo,
            quantidade: value.quantidade,
            preco_medio: value.preco_medio,
            valor_total: value.valor_total,
            data_ultima_atualizacao: value.data_ultima_atualizacao,
        }
    }
}

impl From<Model> for PositionWalletDto {
    fn from(value: Model) -> Self {
        Self {
            codigo_ativo: value.codigo_ativo,
            quantidade: value.quantidade,
            preco_medio: value.preco_medio,
            valor_total: value.valor_total,
            data_ultima_atualizacao: value.data_ultima_atualizacao,
        }
    }
}
