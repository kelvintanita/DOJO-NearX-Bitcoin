use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "Bitcoin RPC API", description = "API for Bitcoin RPC operations")
    ),
    paths(
        crate::services::polar::get_block,
        crate::services::polar::get_transaction,
        crate::services::polar::get_node_status,
        crate::services::polar::create_wallet,
        crate::services::polar::list_wallets,
        crate::services::polar::send_bitcoins,
        crate::services::polar::mine_blocks,
        crate::services::polar::add_funds,
        crate::services::polar::get_wallet
    )
)]
pub struct ApiDoc;
