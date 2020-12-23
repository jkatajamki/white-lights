use rocket_contrib::json::{JsonValue};
use super::wl_users;

#[get("/users")]
pub fn get_users() -> JsonValue {
    let users_result = wl_users::db_get_users();

    match users_result {
        Ok(users) => json!({
            "status": 200,
            "users": users,
        }),
        Err(error) => {
            // TODO: Request logging!
            println!("Error getting user results: {}", error);

            json!({
                "status": 500,
                "error": "Failed to get users :(",
            })
        }
    }
}
