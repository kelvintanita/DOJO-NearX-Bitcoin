// 1. Crates Externos
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use log::info;
use reqwest::Client;

// 2. Módulos Internos
mod models;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Servidor iniciado na porta 3000");
    dotenv().ok();

    let client = Client::new();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .route("/block/{block_number}", web::get().to(services::polar::get_block))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
