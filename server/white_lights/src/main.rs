use actix_web::{web, App, HttpServer};

#[macro_use] extern crate diesel;
extern crate dotenv;

mod db;
mod pool;
mod users;
mod env_vars;
pub mod schema;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");

    let bind_addr = env_vars::get_bind_addr();

    let db_pool = pool::get_pool();

    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .route("/users", web::get().to(users::routes::get_users))
            .route("/users", web::post().to(users::routes::register_new_user))
    })
        .bind(bind_addr)?
        .run()
        .await
}
