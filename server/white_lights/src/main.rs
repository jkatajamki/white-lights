use actix_web::{web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;

#[macro_use] extern crate diesel;
extern crate dotenv;

mod db;
mod pool;
mod users;
mod env_vars;
mod api;
mod wl_error;
mod auth;
pub mod schema;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");

    let bind_addr = env_vars::get_bind_addr();

    let db_pool = pool::get_pool();

    HttpServer::new(move || {
        let auth_wrapper = HttpAuthentication::bearer(auth::validate_auth);

        App::new()
            .data(db_pool.clone())
            .route("/", web::get().to(api::get_status))
            .service(
                web::scope("/users")
                    .wrap(auth_wrapper)
                    .route("/", web::get().to(users::routes::get_users_route))
                    .route("/", web::post().to(users::routes::register_new_user_route))
            )

    })
        .bind(bind_addr)?
        .run()
        .await
}
