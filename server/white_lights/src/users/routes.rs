use actix_web::{Error, HttpResponse, web::{block, Json}};
use super::wl_users::{self, CreateUserRequest};

// TODO: Require authentication for this route
pub async fn get_users() -> Result<HttpResponse, Error> {
    let users_result = block(move || wl_users::db_get_users())
        .await;

    Ok(match users_result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => {
            eprintln!("Error getting user results: {}", err);

            HttpResponse::InternalServerError().json(err.to_string())
        }
    })
}

pub async fn register_new_user(input_user: Json<CreateUserRequest>) ->
    Result<HttpResponse, Error>
{
    let create_user_req = CreateUserRequest{
        create_user_email: String::from(&input_user.create_user_email),
        create_user_secret: String::from(&input_user.create_user_secret),
    };

    let result = block(move || {
            wl_users::handle_registration(create_user_req)
        })
        .await;

    Ok(match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => {
            eprintln!("Error handling registration: {}", err);

            HttpResponse::InternalServerError().json(err.to_string())
        },
    })
}
