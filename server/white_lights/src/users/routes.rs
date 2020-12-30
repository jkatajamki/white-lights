use rocket_contrib::json::{JsonValue, Json};
use super::wl_users::{self, WLUser};
use rocket::http::Status;
use crate::api::response::send_json_response;

// TODO: Require authentication for this route
#[get("/users")]
pub fn get_users() -> Json<Result<Vec<WLUser>, String>> {
    let users_result = wl_users::db_get_users();

    match users_result {
        Ok(users) => Json(Ok(users)),
        Err(err) => {
            // TODO: Request logging!
            eprintln!("Error getting user results: {}", err);

            Json(Err(err.to_string()))
        }
    }
}

#[post("/register", data = "<register_form>")]
pub fn register_new_user(
    register_form: Json<wl_users::CreateUserRequest>
) -> JsonValue {
    let result = wl_users::handle_registration(register_form.0);

    match result {
        Ok(user) => send_json_response(user, Status::Ok),
        Err(err) => {
            eprintln!("Error handling registration: {}", err);

            send_json_response(err.to_string(), Status::InternalServerError)
        },
    }
}
