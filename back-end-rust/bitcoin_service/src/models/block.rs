use serde::Deserialize;

#[derive(Deserialize)]
pub struct BlockRequest {
    pub block_number: u64,
}