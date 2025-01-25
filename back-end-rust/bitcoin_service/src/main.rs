// 1. Crates Externos
use actix_web::{web, App, HttpServer};
use log::info;
use reqwest::Client;
use dotenv::dotenv;
use utoipa_swagger_ui::SwaggerUi;
use crate::config::env;

// 2. Módulos Internos
mod models;
mod services;
mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("========= Servidor iniciado na porta 3000 =========");
    dotenv().ok(); 
    
    validate_env_vars();

    let client = Client::new();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", swagger::ApiDoc::openapi()))
            .route("/block/{block_number}", web::get().to(services::polar::get_block)) // Endpoint para obter bloco por número
            .route("/transaction/{txid}", web::get().to(services::polar::get_transaction)) // Obter transação por TXID
            .route("/node-status", web::get().to(services::polar::get_node_status)) // Consultar status do nó
            .route("/create-wallet", web::post().to(services::polar::create_wallet)) // Criar uma nova carteira
            .route("/list-wallets", web::get().to(services::polar::list_wallets)) // Listar todas as carteiras
            .route("/send", web::post().to(services::polar::send_bitcoins)) // Enviar bitcoins
            .route("/mine-blocks", web::post().to(services::polar::mine_blocks)) // Minerar blocos
            .route("/add-funds", web::post().to(services::polar::add_funds)) // Adicionar fundos ao endereço minerando
            .route("/wallet/{address}", web::get().to(services::polar::get_wallet)) // Obter informações de uma carteira
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