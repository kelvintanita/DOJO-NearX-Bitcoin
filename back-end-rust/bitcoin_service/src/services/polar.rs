use crate::config::env_globals::{RPC_PASSWORD, RPC_URL, RPC_USER};
use actix_web::{web, HttpResponse, Responder};
use reqwest::Client;
use serde_json::{json, Value};

use crate::models::block::BlockRequest;

pub async fn get_block(info: web::Path<BlockRequest>, client: web::Data<Client>) -> impl Responder {
    // Montagem do payload JSON para a chamada RPC
    let payload = json!({
        "jsonrpc": "1.0",
        "id": "postman",
        "method": "getblockhash",
        "params": [info.block_number]
    });

    // Envio da requisição POST com autenticação básica
    let response = match client
        .post(&*RPC_URL)
        .json(&payload)
        .basic_auth(&*RPC_USER, Some(&*RPC_PASSWORD))
        .send()
        .await
    {
        Ok(res) => res,
        Err(err) => {
            eprintln!("Request error: {}", err);
            return HttpResponse::InternalServerError().body("Failed to send request");
        }
    };

    // Parse da resposta como JSON
    let response_json: Value = match response.json().await {
        Ok(json) => json,
        Err(err) => {
            eprintln!("Failed to parse JSON: {}", err);
            return HttpResponse::InternalServerError().body("Invalid response from RPC server");
        }
    };

    // Retorna a resposta como JSON
    HttpResponse::Ok().json(response_json)
}
