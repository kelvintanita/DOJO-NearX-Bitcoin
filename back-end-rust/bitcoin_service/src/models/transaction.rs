use utoipa::ToSchema;
use serde::Serialize;

#[derive(Serialize, ToSchema)]
pub struct Transaction {
    pub txid: String,
    pub amount: f64,
    pub confirmations: u64,
}