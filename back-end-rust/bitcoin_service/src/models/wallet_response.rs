use utoipa::ToSchema;
use serde::Serialize;
use crate::models::transaction::Transaction;

#[derive(Serialize, ToSchema)]
pub struct WalletResponse {
    pub address: String,
    pub balance: f64,
    pub transactions: Vec<Transaction>,
}