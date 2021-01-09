use diesel::pg::PgConnection;
use crate::diesel::Connection;
use dotenv::dotenv;
use crate::diesel::r2d2::{PooledConnection, ConnectionManager};
use crate::env_vars;

pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn db_connect() -> PgConnection {
    dotenv().ok();

    let db_url = env_vars::get_db_url();

    PgConnection::establish(&db_url)
        .expect(&format!("Error connecting to {}", db_url))
}
