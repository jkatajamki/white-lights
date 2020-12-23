use diesel::pg::PgConnection;
use crate::diesel::Connection;
use dotenv::dotenv;
use std::env;

pub fn db_connect() -> PgConnection {
    dotenv().ok();

    let db_name_env_variable = "DATABASE_URL";

    let db_url = env::var(db_name_env_variable)
        .expect("DATABASE_URL env variable must be set");

    PgConnection::establish(&db_url)
        .expect(&format!("Error connecting to {}", db_url))
}
