use crate::transaction::TransactionType;
use entity::order::Model;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct OrderDto {
    pub(crate) order_id: String,
    pub(crate) symbol: String,
    pub(crate) order_type: TransactionType,
    pub(crate) quantity: i32,
    pub(crate) price: f32,
    pub(crate) date: Option<chrono::NaiveDate>,
}

impl From<Model> for OrderDto {
    fn from(model: Model) -> Self {
        Self {
            order_id: model.id,
            symbol: model.symbol_id,
            order_type: model.r#type.into(),
            quantity: model.quantity,
            price: model.price,
            date: Some(model.date),
        }
    }
}
impl From<OrderDto> for Model {
    fn from(dto: OrderDto) -> Self {
        Model {
            id: dto.order_id,
            symbol_id: dto.symbol,
            r#type: dto.order_type.to_string(), // Converte o tipo de transação
            quantity: dto.quantity,
            price: dto.price,
            date: dto.date.unwrap_or_else(|| chrono::Local::now().naive_local().date()), // Usa `Default` caso não tenha data
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct PartialOrder {
    order_id: Option<String>,
    pub(crate) symbol: Option<String>,
    pub(crate) quantity: Option<i32>,
    pub(crate) price: Option<f32>,
    pub(crate) date: Option<chrono::NaiveDate>,
}
