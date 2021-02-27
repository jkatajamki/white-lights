use diesel::pg::PgConnection;
use crate::diesel::r2d2::{PooledConnection, ConnectionManager};

pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;
