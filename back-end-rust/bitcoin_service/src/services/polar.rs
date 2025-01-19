
use actix_web::{web, HttpResponse, Responder};
use serde_json::{json, Value};
use reqwest::Client;
use std::env;

use crate::models::block::BlockRequest;
pub async fn get_block(info: web::Path<BlockRequest>, client: web::Data<Client>) -> impl Responder {
    // Leitura do host e porta para a conexão RPC
    let host = match env::var("HOST") {
        Ok(h) => h,
        Err(_) => {
            return HttpResponse::InternalServerError().body("HOST environment variable not set")
        }
    };

    // Leitura da porta para a conexão RPC
    let port = match env::var("PORT") {
        Ok(port) => port,
        Err(_) => {
            return HttpResponse::InternalServerError().body("PORT environment variable not set")
        }
    };

    // Leitura do usuário e senha para autenticação
    let rpc_user = match env::var("BITCOIN_RPC_USER") {
        Ok(user) => user,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .body("BITCOIN_RPC_USER environment variable not set")
        }
    };
    let rpc_password = match env::var("BITCOIN_RPC_PASS") {
        Ok(password) => password,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .body("BITCOIN_RPC_PASS environment variable not set")
        }
    };
    // Montagem da URL para a chamada RPC
    let rpc_url = format!("http://{}:{}/", host, port);

    // Montagem do payload JSON para a chamada RPC
    let payload = json!({
        "jsonrpc": "1.0",
        "id": "postman",
        "method": "getblockhash",
        "params": [info.block_number]
    });

    // Envio da requisição POST com autenticação básica
    let response = match client
        .post(&rpc_url)
        .json(&payload)
        .basic_auth(rpc_user, Some(rpc_password))
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