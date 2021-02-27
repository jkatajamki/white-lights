use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use crate::env_vars;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn get_pool() -> Pool {
    let database_url = env_vars::get_db_url();

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB pool")
}
