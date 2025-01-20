use crate::config::env_globals::{RPC_PASSWORD, RPC_URL, RPC_USER};
use crate::models::add_funds::AddFundsRequest;
use crate::models::block::BlockRequest;
use crate::models::send::SendBitcoinRequest;
use crate::models::transaction::Transaction;
use actix_web::{web, HttpResponse, Responder};
use base64;
use reqwest::header::HeaderValue;
use reqwest::Client;
use serde_json::{json, Value};
use std::collections::HashMap;

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

pub async fn get_transaction(info: web::Path<String>, client: web::Data<Client>) -> impl Responder {
    let txid = info.into_inner();

    let payload = json!({
        "jsonrpc": "1.0",
        "id": "postman",
        "method": "gettransaction",
        "params": [txid]
    });

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

    let response_json: Value = match response.json().await {
        Ok(json) => json,
        Err(err) => {
            eprintln!("Failed to parse JSON: {}", err);
            return HttpResponse::InternalServerError().body("Invalid response from RPC server");
        }
    };

    HttpResponse::Ok().json(response_json)
}

pub async fn get_node_status(client: web::Data<Client>) -> impl Responder {
    let blockchain_info = json!({
        "jsonrpc": "1.0",
        "id": "postman",
        "method": "getblockchaininfo",
        "params": []
    });

    let network_info = json!({
        "jsonrpc": "1.0",
        "id": "postman",
        "method": "getnetworkinfo",
        "params": []
    });

    let blockchain_response = client
        .post(&*RPC_URL)
        .json(&blockchain_info)
        .basic_auth(&*RPC_USER, Some(&*RPC_PASSWORD))
        .send()
        .await;

    let network_response = client
        .post(&*RPC_URL)
        .json(&network_info)
        .basic_auth(&*RPC_USER, Some(&*RPC_PASSWORD))
        .send()
        .await;

    match (blockchain_response, network_response) {
        (Ok(b_res), Ok(n_res)) => {
            let blockchain_json: Value = b_res.json().await.unwrap_or_else(|_| json!({}));
            let network_json: Value = n_res.json().await.unwrap_or_else(|_| json!({}));

            HttpResponse::Ok().json(json!({
                "blockchain_info": blockchain_json,
                "network_info": network_json
            }))
        }
        _ => HttpResponse::InternalServerError().body("Failed to fetch node status"),
    }
}

pub async fn create_wallet(
    req_body: web::Json<serde_json::Value>,
    client: web::Data<Client>,
) -> impl Responder {
    let label = req_body.get("label").and_then(|l| l.as_str()).unwrap_or("");

    let payload = json!({
        "jsonrpc": "1.0",
        "id": "postman",
        "method": "getnewaddress",
        "params": [label]
    });

    let response = client
        .post(&*RPC_URL)
        .json(&payload)
        .basic_auth(&*RPC_USER, Some(&*RPC_PASSWORD))
        .send()
        .await;

    match response {
        Ok(res) => {
            let response_json: Value = res.json().await.unwrap_or_else(|_| json!({}));
            HttpResponse::Ok().json(response_json)
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            HttpResponse::InternalServerError().body("Failed to create wallet")
        }
    }
}

pub async fn mine_blocks(
    req_body: web::Json<serde_json::Value>,
    client: web::Data<Client>,
) -> impl Responder {
    let num_blocks = req_body
        .get("numBlocks")
        .and_then(|nb| nb.as_u64())
        .unwrap_or(1);

    let payload_address = json!({
        "jsonrpc": "1.0",
        "id": "postman",
        "method": "getnewaddress",
        "params": []
    });

    let address_response = client
        .post(&*RPC_URL)
        .json(&payload_address)
        .basic_auth(&*RPC_USER, Some(&*RPC_PASSWORD))
        .send()
        .await;

    if let Ok(address_res) = address_response {
        if let Ok(address_json) = address_res.json::<Value>().await {
            if let Some(address) = address_json["result"].as_str() {
                let payload_generate = json!({
                    "jsonrpc": "1.0",
                    "id": "postman",
                    "method": "generatetoaddress",
                    "params": [num_blocks, address]
                });

                let generate_response = client
                    .post(&*RPC_URL)
                    .json(&payload_generate)
                    .basic_auth(&*RPC_USER, Some(&*RPC_PASSWORD))
                    .send()
                    .await;

                if let Ok(gen_res) = generate_response {
                    let gen_json: Value = gen_res.json().await.unwrap_or_else(|_| json!({}));
                    return HttpResponse::Ok().json(gen_json);
                }
            }
        }
    }

    HttpResponse::InternalServerError().body("Failed to mine blocks")
}

pub async fn list_wallets(client: web::Data<Client>) -> impl Responder {
    // Requisição para obter os labels (carteiras)
    let payload_labels = json!({
        "jsonrpc": "1.0",
        "id": "listwallets",
        "method": "listlabels",
        "params": []
    });

    let response_labels = match client
        .post(&*crate::config::env_globals::RPC_URL)
        .json(&payload_labels)
        .basic_auth(
            &*crate::config::env_globals::RPC_USER,
            Some(&*crate::config::env_globals::RPC_PASSWORD),
        )
        .send()
        .await
    {
        Ok(res) => res,
        Err(err) => {
            eprintln!("Request error (listlabels): {}", err);
            return HttpResponse::InternalServerError().body("Failed to list wallets");
        }
    };

    // Parse da resposta como JSON
    let labels: Vec<String> = match response_labels.json::<Value>().await {
        Ok(json) => match json["result"].as_array() {
            Some(array) => array
                .iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect(),
            None => {
                eprintln!("Unexpected format for listlabels response");
                return HttpResponse::InternalServerError()
                    .body("Invalid response from RPC server");
            }
        },
        Err(err) => {
            eprintln!("Failed to parse listlabels response: {}", err);
            return HttpResponse::InternalServerError().body("Invalid response from RPC server");
        }
    };

    // Itera sobre os labels para obter os endereços associados
    let mut result = Vec::new();
    for label in labels {
        let payload_addresses = json!({
            "jsonrpc": "1.0",
            "id": "getaddressesbylabel",
            "method": "getaddressesbylabel",
            "params": [label]
        });

        let response_addresses = match client
            .post(&*crate::config::env_globals::RPC_URL)
            .json(&payload_addresses)
            .basic_auth(
                &*crate::config::env_globals::RPC_USER,
                Some(&*crate::config::env_globals::RPC_PASSWORD),
            )
            .send()
            .await
        {
            Ok(res) => res,
            Err(err) => {
                eprintln!(
                    "Request error (getaddressesbylabel) for label {}: {}",
                    label, err
                );
                continue;
            }
        };

        let addresses: HashMap<String, Value> = match response_addresses.json::<Value>().await {
            Ok(json) => match json["result"].as_object() {
                Some(map) => map.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
                None => {
                    eprintln!("Unexpected format for getaddressesbylabel response");
                    continue;
                }
            },
            Err(err) => {
                eprintln!("Failed to parse getaddressesbylabel response: {}", err);
                continue;
            }
        };

        result.push(json!({
            "label": label,
            "addresses": addresses.keys().cloned().collect::<Vec<String>>()
        }));
    }

    // Retorna o resultado como JSON
    HttpResponse::Ok().json(result)
}

pub async fn send_bitcoins(
    client: web::Data<reqwest::Client>,
    payload: web::Json<SendBitcoinRequest>,
) -> HttpResponse {
    let from_address = &payload.from_address;
    let to_address = &payload.to_address;
    let amount = payload.amount;
    let auth = reqwest::header::HeaderValue::from_str(&format!(
        "Basic {}",
        base64::encode(format!("{}:{}", &*RPC_USER, &*RPC_PASSWORD))
    ))
    .unwrap_or_else(|_| HeaderValue::from_static(""));

    let client = client.get_ref();

    // 1. Listar UTXOs para o endereço de origem
    let utxos = match client
        .post(&*RPC_URL)
        .header(reqwest::header::AUTHORIZATION, auth.clone())
        .json(&json!({
            "jsonrpc": "1.0",
            "id": "curltext",
            "method": "listunspent",
            "params": [1, 9999999, [from_address]]
        }))
        .send()
        .await
    {
        Ok(response) => match response.json::<Value>().await {
            Ok(res) => res["result"].as_array().unwrap_or(&vec![]).clone(),
            Err(_) => {
                return HttpResponse::InternalServerError()
                    .json(json!({"error": "Failed to parse UTXOs"}))
            }
        },
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json(json!({"error": "Failed to fetch UTXOs"}))
        }
    };

    let balance: f64 = utxos.iter().fold(0.0, |sum, utxo| {
        sum + utxo["amount"].as_f64().unwrap_or(0.0)
    });

    if balance < amount {
        return HttpResponse::BadRequest().json(json!({"error": "Insufficient funds"}));
    }

    // 2. Preparar inputs e outputs
    let inputs: Vec<HashMap<&str, Value>> = utxos
        .iter()
        .map(|utxo| {
            let mut input = HashMap::new();
            input.insert("txid", utxo["txid"].clone());
            input.insert("vout", utxo["vout"].clone());
            input
        })
        .collect();

    let mut outputs = HashMap::new();
    outputs.insert(to_address.clone(), Value::from(amount));
    let fee = 0.0001;
    if balance > amount + fee {
        outputs.insert(from_address.clone(), Value::from(balance - amount - fee));
    }

    // 3. Criar a transação
    let raw_tx = match client
        .post(&*RPC_URL)
        .header(reqwest::header::AUTHORIZATION, auth.clone())
        .json(&json!({
            "jsonrpc": "1.0",
            "id": "curltext",
            "method": "createrawtransaction",
            "params": [inputs, outputs]
        }))
        .send()
        .await
    {
        Ok(response) => match response.json::<Value>().await {
            Ok(res) => res["result"].as_str().unwrap_or("").to_string(),
            Err(_) => {
                return HttpResponse::InternalServerError()
                    .json(json!({"error": "Failed to create raw transaction"}))
            }
        },
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json(json!({"error": "Failed to create raw transaction"}))
        }
    };

    // 4. Assinar a transação
    let signed_tx = match client
        .post(&*RPC_URL)
        .header(reqwest::header::AUTHORIZATION, auth.clone())
        .json(&json!({
            "jsonrpc": "1.0",
            "id": "curltext",
            "method": "signrawtransactionwithwallet",
            "params": [raw_tx]
        }))
        .send()
        .await
    {
        Ok(response) => match response.json::<Value>().await {
            Ok(res) => res["result"]["hex"].as_str().unwrap_or("").to_string(),
            Err(_) => {
                return HttpResponse::InternalServerError()
                    .json(json!({"error": "Failed to sign transaction"}))
            }
        },
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json(json!({"error": "Failed to sign transaction"}))
        }
    };

    // 5. Enviar a transação
    let tx_id = match client
        .post(&*RPC_URL)
        .header(reqwest::header::AUTHORIZATION, auth.clone())
        .json(&json!({
            "jsonrpc": "1.0",
            "id": "curltext",
            "method": "sendrawtransaction",
            "params": [signed_tx]
        }))
        .send()
        .await
    {
        Ok(response) => match response.json::<Value>().await {
            Ok(res) => res["result"].as_str().unwrap_or("").to_string(),
            Err(_) => {
                return HttpResponse::InternalServerError()
                    .json(json!({"error": "Failed to send transaction"}))
            }
        },
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json(json!({"error": "Failed to send transaction"}))
        }
    };

    // Retornar ID da transação
    HttpResponse::Ok().json(json!({ "txId": tx_id }))
}

pub async fn add_funds(
    client: web::Data<reqwest::Client>,
    payload: web::Json<AddFundsRequest>,
) -> HttpResponse {
    let address = &payload.address;
    let num_blocks = payload.num_blocks;

    if address.is_empty() || num_blocks == 0 {
        return HttpResponse::BadRequest().json(json!({
            "error": "É necessário fornecer o endereço e o número de blocos a minerar."
        }));
    }

    let rpc_url = &*RPC_URL;
    let rpc_user = &*RPC_USER; 
    let rpc_password = &*RPC_PASSWORD; 

    let auth = reqwest::header::HeaderValue::from_str(&format!(
        "Basic {}",
        base64::encode(format!("{}:{}", rpc_user, rpc_password))
    ))
    .unwrap_or_else(|_| HeaderValue::from_static(""));

    let client = client.get_ref();

    // Chamar o método `generatetoaddress`
    match client
        .post(rpc_url)
        .header(reqwest::header::AUTHORIZATION, auth)
        .json(&json!({
            "jsonrpc": "1.0",
            "id": "addfunds",
            "method": "generatetoaddress",
            "params": [num_blocks, address]
        }))
        .send()
        .await
    {
        Ok(response) => match response.json::<serde_json::Value>().await {
            Ok(result) => HttpResponse::Ok().json(json!({
                "message": format!("Minerados {} blocos. Recompensa enviada para o endereço.", num_blocks),
                "minedBlocks": result["result"]
            })),
            Err(_) => HttpResponse::InternalServerError().json(json!({
                "error": "Erro ao processar a resposta do nó Bitcoin."
            })),
        },
        Err(error) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Erro ao comunicar-se com o nó Bitcoin: {}", error)
        })),
    }
}

pub async fn get_wallet(
    client: web::Data<reqwest::Client>,
    path: web::Path<String>,
) -> HttpResponse {
    let address = path.into_inner();

    if address.is_empty() {
        return HttpResponse::BadRequest().json(json!({
            "error": "É necessário fornecer o endereço da carteira."
        }));
    }

    let rpc_url = &*RPC_URL;
    let rpc_user = &*RPC_USER;
    let rpc_password = &*RPC_PASSWORD;

    let auth = reqwest::header::HeaderValue::from_str(&format!(
        "Basic {}",
        base64::encode(format!("{}:{}", rpc_user, rpc_password))
    ))
    .map_err(|_| HeaderValue::from_static("")).unwrap_or_else(|e| {
        eprintln!("Failed to create header value: {:?}", e);
        HeaderValue::from_static("")
    });

    let client = client.get_ref();

    // Realizar a chamada RPC para `listunspent`
    match client
        .post(rpc_url)
        .header(reqwest::header::AUTHORIZATION, auth)
        .json(&json!({
            "jsonrpc": "1.0",
            "id": "getwallet",
            "method": "listunspent",
            "params": [1, 9999999, [address.clone()]]
        }))
        .send()
        .await
    {
        Ok(response) => match response.json::<serde_json::Value>().await {
            Ok(result) => {
                if let Some(utxos) = result["result"].as_array() {
                    let balance: f64 = utxos
                        .iter()
                        .map(|utxo| utxo["amount"].as_f64().unwrap_or(0.0))
                        .sum();

                    let transactions: Vec<Transaction> = utxos
                        .iter()
                        .map(|utxo| Transaction {
                            txid: utxo["txid"].as_str().unwrap_or("").to_string(),
                            amount: utxo["amount"].as_f64().unwrap_or(0.0),
                            confirmations: utxo["confirmations"].as_u64().unwrap_or(0),
                        })
                        .collect();

                    HttpResponse::Ok().json(json!({
                        "address": address,
                        "balance": balance,
                        "transactions": transactions,
                    }))
                } else {
                    HttpResponse::InternalServerError().json(json!({
                        "error": "Erro ao processar a resposta do nó Bitcoin."
                    }))
                }
            }
            Err(_) => HttpResponse::InternalServerError().json(json!({
                "error": "Erro ao processar a resposta do nó Bitcoin."
            })),
        },
        Err(error) => HttpResponse::InternalServerError().json(json!({
            "error": format!("Erro ao comunicar-se com o nó Bitcoin: {}", error)
        })),
    }
}