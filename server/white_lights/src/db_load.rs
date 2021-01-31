use std::fmt::Display;
use actix_web::{web::Data};
use crate::pool::Pool;
use crate::db::DbConnection;


pub fn run_query<F, T, E: Display>(pool: Data<Pool>, query_f: F) -> Result<T, String>
    where F: Fn(DbConnection) -> Result<T, E>
{
    let conn = match pool.get() {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("Error getting connection from pool! {}", err.to_string());

            return Err(err.to_string())
        }
    };

    match query_f(conn) {
        Ok(data) => Ok(data),
        Err(err) => {
            eprintln!("Error running query to DB! {}", err.to_string());

            Err(err.to_string())
        }
    }
}
