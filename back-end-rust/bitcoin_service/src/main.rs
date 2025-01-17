use warp::Filter;
use bitcoin-rpc::{Auth, Client};
use serde::{Deserialize, Serialize};
use std::env;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    // Configuração do cliente RPC
    let rpc_client = Client::new(
        env::var("BITCOIN_RPC_URL").expect("BITCOIN_RPC_URL not set"),
        Auth::UserPass(
            env::var("BITCOIN_RPC_USER").expect("BITCOIN_RPC_USER not set"),
            env::var("BITCOIN_RPC_PASS").expect("BITCOIN_RPC_PASS not set"),
        ),
    )
    .unwrap();

    let client_filter = warp::any().map(move || rpc_client.clone());

    // Endpoints
    let get_block = warp::path!("block" / u64)
        .and(client_filter.clone())
        .and_then(get_block_handler);

    let get_transaction = warp::path!("transaction" / String)
        .and(client_filter.clone())
        .and_then(get_transaction_handler);

    let routes = warp::get().and(get_block.or(get_transaction));

    println!("Server is running on port 8080");
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}

// Handlers
async fn get_block_handler(block_number: u64, client: Client) -> Result<impl warp::Reply, warp::Rejection> {
    match client.call::<String>("getblockhash", &[block_number.into()]) {
        Ok(block_hash) => match client.call::<serde_json::Value>("getblock", &[block_hash.into()]) {
            Ok(block) => Ok(warp::reply::json(&block)),
            Err(e) => Err(warp::reject::custom(e)),
        },
        Err(e) => Err(warp::reject::custom(e)),
    }
}

async fn get_transaction_handler(txid: String, client: Client) -> Result<impl warp::Reply, warp::Rejection> {
    match client.call::<serde_json::Value>("gettransaction", &[txid.into()]) {
        Ok(transaction) => Ok(warp::reply::json(&transaction)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
use warp::Filter;
use bitcoin_rpc::{Auth, Client};
use serde::{Deserialize, Serialize};
use std::env;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    // Configuração do cliente RPC
    let rpc_client = Client::new(
        env::var("BITCOIN_RPC_URL").expect("BITCOIN_RPC_URL not set"),
        Auth::UserPass(
            env::var("BITCOIN_RPC_USER").expect("BITCOIN_RPC_USER not set"),
            env::var("BITCOIN_RPC_PASS").expect("BITCOIN_RPC_PASS not set"),
        ),
    )
    .unwrap();

    let client_filter = warp::any().map(move || rpc_client.clone());

    // Endpoints
    let get_block = warp::path!("block" / u64)
        .and(client_filter.clone())
        .and_then(get_block_handler);

    let get_transaction = warp::path!("transaction" / String)
        .and(client_filter.clone())
        .and_then(get_transaction_handler);

    let routes = warp::get().and(get_block.or(get_transaction));

    println!("Server is running on port 8080");
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}

// Handlers
async fn get_block_handler(block_number: u64, client: Client) -> Result<impl warp::Reply, warp::Rejection> {
    match client.call::<String>("getblockhash", &[block_number.into()]) {
        Ok(block_hash) => match client.call::<serde_json::Value>("getblock", &[block_hash.into()]) {
            Ok(block) => Ok(warp::reply::json(&block)),
            Err(e) => Err(warp::reject::custom(e)),
        },
        Err(e) => Err(warp::reject::custom(e)),
    }
}

async fn get_transaction_handler(txid: String, client: Client) -> Result<impl warp::Reply, warp::Rejection> {
    match client.call::<serde_json::Value>("gettransaction", &[txid.into()]) {
        Ok(transaction) => Ok(warp::reply::json(&transaction)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
