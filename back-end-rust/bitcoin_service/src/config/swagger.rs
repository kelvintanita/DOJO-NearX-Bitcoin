use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::services::polar::get_block,
        crate::services::polar::get_transaction,
        crate::services::polar::get_node_status,
        crate::services::polar::create_wallet,
        crate::services::polar::mine_blocks,
        crate::services::polar::add_funds,
        crate::services::polar::get_wallet,
        crate::services::polar::send_bitcoins,
        crate::services::polar::list_wallets
    ),
    components(schemas(
        crate::models::block::BlockRequest,
        crate::models::add_funds::AddFundsRequest,
        crate::models::send::SendBitcoinRequest,
        crate::models::transaction::Transaction,
        crate::models::wallet_response::WalletResponse
    )),
    tags(
        (name = "Bitcoin RPC API", description = "API for Bitcoin RPC operations")
    )
)]
pub struct ApiDoc;
