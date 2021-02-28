use dotenv::dotenv;
use std::env;

pub fn get_bind_addr() -> String {
    dotenv().ok();

    let host = env::var("API_HOST").expect("Failed to load host address from env");
    let port = env::var("API_PORT").expect("Failed to load host port from env");

    format!("{}:{}", host, port)
}

pub fn get_db_url() -> String {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("Failed to load database url from env");

    db_url
}

pub fn get_authority() -> String {
    dotenv().ok();

    let authority = env::var("AUTHORITY").expect("Failed to load authority from env");

    authority
}
