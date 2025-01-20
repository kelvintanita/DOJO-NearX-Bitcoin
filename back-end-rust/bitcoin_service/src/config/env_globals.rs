use once_cell::sync::Lazy;
use crate::config::constants::{ENV_BITCOIN_RPC_HOST, ENV_BITCOIN_RPC_PORT, ENV_BITCOIN_RPC_USER, ENV_BITCOIN_RPC_PASS};
use crate::config::env;

// Inicialização das variáveis globais
pub static RPC_HOST: Lazy<String> = Lazy::new(|| env::get_env_value(ENV_BITCOIN_RPC_HOST));
pub static RPC_PORT: Lazy<String> = Lazy::new(|| env::get_env_value(ENV_BITCOIN_RPC_PORT));
pub static RPC_USER: Lazy<String> = Lazy::new(|| env::get_env_value(ENV_BITCOIN_RPC_USER));
pub static RPC_PASSWORD: Lazy<String> = Lazy::new(|| env::get_env_value(ENV_BITCOIN_RPC_PASS));

// URL montada como global
pub static RPC_URL: Lazy<String> = Lazy::new(|| format!("http://{}:{}/", *RPC_HOST, *RPC_PORT));
