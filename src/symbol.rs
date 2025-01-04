use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Symbol {
    pub id: String,
    pub name: String,
    pub last_price: Option<f64>,
}
