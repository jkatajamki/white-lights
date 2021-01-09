use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn get_pool() -> Pool {
    let database_url = std::env::var("DATABASE_URL")
        .expect("Failed to load DATABASE_URL from env");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create DB pool")
}
