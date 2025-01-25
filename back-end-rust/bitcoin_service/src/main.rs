// 1. Crates Externos
use crate::config::env;
use crate::config::swagger::ApiDoc;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use log::info;
use reqwest::Client;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// 2. Módulos Internos
mod config;
mod models;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("========= Servidor iniciado na porta 3000 =========");
    dotenv().ok();

    validate_env_vars();

    let client = Client::new();

    HttpServer::new(move || {
        App::new()
            .service(
                SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()), // Rota do Swagger
            )
            .app_data(web::Data::new(client.clone())) // Cliente compartilhado
            .route(
                "/block/{block_number}",
                web::get().to(services::polar::get_block),
            ) // Obter bloco por número
            .route(
                "/transaction/{txid}",
                web::get().to(services::polar::get_transaction),
            ) // Obter transação por TXID
            .route(
                "/node/status",
                web::get().to(services::polar::get_node_status),
            ) // Status do nó
            .route(
                "/wallet/create",
                web::post().to(services::polar::create_wallet),
            ) // Criar carteira
            .route("/wallets", web::get().to(services::polar::list_wallets)) // Listar carteiras
            .route("/send", web::post().to(services::polar::send_bitcoins)) // Enviar bitcoins
            .route("/mine/blocks", web::post().to(services::polar::mine_blocks)) // Minerar blocos
            .route("/funds/add", web::post().to(services::polar::add_funds)) // Adicionar fundos minerando
            .route(
                "/wallet/{address}",
                web::get().to(services::polar::get_wallet),
            ) // Obter informações de uma carteira
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}

fn validate_env_vars() {
    let required_vars = vec![
        "BITCOIN_RPC_USER",
        "BITCOIN_RPC_PASS",
        "BITCOIN_RPC_HOST",
        "BITCOIN_RPC_PORT",
    ];

    for var in required_vars {
        if env::get_env_value(var).starts_with("Environment variable") {
            panic!("Required environment variable {} is missing", var);
        }
    }
}
