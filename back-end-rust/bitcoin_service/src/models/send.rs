#[derive(serde::Deserialize)]
pub struct SendBitcoinRequest {
    pub from_address: String,
    pub to_address: String,
    pub amount: f64,
}