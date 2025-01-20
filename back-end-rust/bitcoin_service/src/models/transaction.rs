use serde::Serialize;
#[derive(Serialize)]
pub struct Transaction {
    pub(crate) txid: String,
    pub(crate) amount: f64,
    pub(crate) confirmations: u64,
}