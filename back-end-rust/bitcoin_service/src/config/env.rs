use dotenv::dotenv;
use std::env;

pub fn load_env() {
    dotenv().ok(); // Carrega o arquivo .env
    println!("Environment variables loaded");
}

pub fn get_env_value(key: &str) -> String {
    load_env();
    let env = match env::var(key) {
        Ok(h) => h,
        Err(_) => {
            return format!("Environment variable {} not found", key);
        }
    };
    env
}

