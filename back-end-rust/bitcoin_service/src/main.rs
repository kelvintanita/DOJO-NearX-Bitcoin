// 1. Crates Externos
use actix_web::{web, App, HttpServer};
use log::info;
use reqwest::Client;
use dotenv::dotenv;

// 2. Módulos Internos
mod models;
mod services;
mod config; 

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("========= Servidor iniciado na porta 3000 =========");
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
