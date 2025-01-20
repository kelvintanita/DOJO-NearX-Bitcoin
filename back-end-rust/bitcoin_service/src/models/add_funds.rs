#[derive(serde::Deserialize)]
pub struct AddFundsRequest {
    pub address: String,
    pub num_blocks: u64,
}