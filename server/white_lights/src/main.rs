use actix_web::{web, App, HttpServer};
use std::env;

#[macro_use] extern crate diesel;
extern crate dotenv;

mod db;
mod users;
pub mod schema;

use dotenv::dotenv;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");

    dotenv().expect("Failed to read .env file");

    // TODO: refactor to a separate function
    let host = env::var("API_HOST").expect("Failed to load host address from env");
    let port = env::var("API_PORT").expect("Failed to load host port from env");
    let bind_addr = format!("{}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            .route("/users", web::get().to(users::routes::get_users))
            .route("/users", web::post().to(users::routes::register_new_user))
    })
    .bind(bind_addr)?
    .run()
    .await
}
