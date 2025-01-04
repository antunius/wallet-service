use chrono::NaiveDate;
use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct TransactionCsv {
    #[serde(rename = "Data do Negócio")]
    deal_date: String,
    #[serde(rename = "Tipo de Movimentação")]
    transaction_type: String,
    #[serde(rename = "Mercado")]
    market: String,
    #[serde(rename = "Código de Negociação")]
    negotiation_code: String,
    #[serde(rename = "Quantidade")]
    quantity: u32,
    #[serde(rename = "Preço")]
    price: String,
    #[serde(rename = "Valor")]
    value: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub enum TransactionType {
    #[default]
    Buy,
    Sell,
}
impl From<String> for TransactionType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "BUY" => TransactionType::Buy,
            "SELL" => TransactionType::Sell,
            _ => TransactionType::default(),
        }
    }
}
// Implement Display for TransactionType
impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransactionType::Buy => write!(f, "BUY"),
            TransactionType::Sell => write!(f, "SELL"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Transaction {
    pub deal_date: NaiveDate,
    pub transaction_type: TransactionType,
    market: String,
    pub negotiation_code: String,
    pub quantity: u32,
    pub price: f32,
    pub value: f32,
}

// Reads the CSV file and returns Vec<Transaction>
pub(crate) fn read_csv(path: &str) -> Vec<Transaction> {
    let mut csv_reader = ReaderBuilder::new()
        .delimiter(b',')
        .from_path(path)
        .expect("Failed to read CSV file.");

    csv_reader
        .deserialize::<TransactionCsv>()
        .filter_map(Result::ok)
        .map(|tx_csv| Transaction::try_from(tx_csv).ok())
        .flatten()
        .collect()
}

impl FromStr for TransactionType {
    type Err = String;

    fn from_str(input: &str) -> Result<TransactionType, Self::Err> {
        match input {
            "Compra" => Ok(TransactionType::Buy),
            "Venda" => Ok(TransactionType::Sell),
            _ => Err(format!("Invalid transaction type: {}", input)),
        }
    }
}
fn parse_currency(value: &str) -> Result<f32, Box<dyn std::error::Error>> {
    // Trim, remove "R$" prefix, replace `.` with ``, and `,` with `.`
    let binding = value
        .replace(" R$", "")
        .replace(' ', "")
        .replace('.', "")
        .replace(',', ".");

    // Convert the string into f32
    Ok(binding.parse::<f32>()?)
}
impl TryFrom<TransactionCsv> for Transaction {
    type Error = Box<dyn std::error::Error>;

    fn try_from(csv: TransactionCsv) -> Result<Self, Self::Error> {
        // Parse deal date
        let deal_date = chrono::DateTime::parse_from_rfc3339(&csv.deal_date).unwrap();

        // Parse transaction type
        let transaction_type = TransactionType::from_str(&csv.transaction_type)?;

        // Parse price
        let price = parse_currency(&csv.price)?;

        // Parse value
        let value = parse_currency(&csv.value)?;

        // Create the Transaction
        Ok(Transaction {
            deal_date: deal_date.date_naive(),
            transaction_type,
            market: csv.market,
            negotiation_code: csv.negotiation_code,
            quantity: csv.quantity,
            price,
            value,
        })
    }
}
