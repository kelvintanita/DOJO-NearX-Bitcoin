use utoipa::{IntoParams, ToSchema};
use serde::Deserialize;

#[derive(Deserialize, ToSchema, IntoParams)]
pub struct SendBitcoinRequest {
    pub from_address: String,
    pub to_address: String,
    pub amount: f64,
}