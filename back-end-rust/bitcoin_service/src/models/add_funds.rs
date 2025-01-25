use utoipa::{IntoParams, ToSchema};
use serde::Deserialize;

#[derive(Deserialize, ToSchema, IntoParams)]
pub struct AddFundsRequest {
    pub address: String,
    pub num_blocks: u64,
}
