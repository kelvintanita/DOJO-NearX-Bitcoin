use utoipa::{IntoParams, ToSchema};
use serde::Deserialize;

#[derive(Deserialize, ToSchema, IntoParams)]
pub struct BlockRequest {
    pub block_number: u64,
}
