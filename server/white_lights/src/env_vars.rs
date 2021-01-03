use dotenv::dotenv;
use std::env;

pub fn get_bind_addr() -> String {
    dotenv().ok();

    let host = env::var("API_HOST").expect("Failed to load host address from env");
    let port = env::var("API_PORT").expect("Failed to load host port from env");

    format!("{}:{}", host, port)
}
