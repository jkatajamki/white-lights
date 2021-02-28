use actix_web::{HttpResponse, ResponseError, web::{self, block, Json}};
use super::wl_users::{self, CreateUserRequest};
use crate::pool::Pool;
use crate::wl_error::WLError;
use crate::api;
use crate::db;

pub async fn get_users_route(pool: web::Data<Pool>) -> HttpResponse {
    api::run_route_with_db_pool(get_users, pool).await
}

// TODO: Require authentication for this route
async fn get_users(db_conn: db::DbConnection) -> HttpResponse {
    let users_result = block(move || wl_users::db_get_users(&db_conn))
        .await;

    match users_result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => {
            eprintln!("Error getting user results: {}", err);

            WLError::InternalServerError.error_response()
        }
    }
}

pub async fn register_new_user_route(pool: web::Data<Pool>, input_user: Json<CreateUserRequest>) -> HttpResponse {
    api::run_route_with_db_pool_and_payload(register_new_user, pool, input_user).await
}

async fn register_new_user(db_conn: db::DbConnection, input_user: Json<CreateUserRequest>) -> HttpResponse
{
    let create_user_req = CreateUserRequest{
        create_user_email: String::from(&input_user.create_user_email),
        create_user_secret: String::from(&input_user.create_user_secret),
    };

    let result = block(move || {
        wl_users::handle_registration(db_conn, create_user_req)
    })
    .await;

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => {
            eprintln!("Error handling registration: {}", err);

            return err.error_response()
        },
    }
}
