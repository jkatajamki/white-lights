use actix_web::{HttpResponse, web::{self}};
use serde::{Serialize, Deserialize};
use std::future::Future;
use crate::wl_error::WLError;
use crate::pool::Pool;
use crate::db;

#[derive(Serialize, Deserialize)]
struct ApiStatus {
    status: String,
}

pub async fn get_status() -> HttpResponse {
    HttpResponse::Ok().json(ApiStatus{
        status: String::from("Up and running!"),
    })   
}

pub async fn run_route_with_db_pool_and_payload<Fut, T>(
    f: impl FnOnce(db::DbConnection, T) -> Fut,
    pool: web::Data<Pool>,
    payload: T
) -> Result<HttpResponse, WLError>
    where Fut: Future<Output = Result<HttpResponse, WLError>>,
{
    let conn_result = pool.get();

    let conn = match conn_result {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("Error getting connection from pool! {}", err);

            return Err(WLError::InternalServerError)
        }
    };
    
    f(conn, payload).await
}

pub async fn run_route_with_db_pool<Fut>(
    f: impl FnOnce(db::DbConnection) -> Fut,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, WLError>
    where Fut: Future<Output = Result<HttpResponse, WLError>>,
{
    let conn_result = pool.get();

    let conn = match conn_result {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("Error getting connection from pool! {}", err);

            return Err(WLError::InternalServerError)
        }
    };
    
    f(conn).await
}
