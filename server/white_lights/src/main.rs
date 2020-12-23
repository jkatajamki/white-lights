#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket_contrib;
extern crate dotenv;

mod db;
mod users;
pub mod schema;

use dotenv::dotenv;

#[get("/")]
fn index() -> &'static str {
    "Three white lights!"
}

fn main() {
    dotenv().ok();

    rocket::ignite().mount("/", routes![
        index,
        users::routes::get_users,
    ]).launch();
}
