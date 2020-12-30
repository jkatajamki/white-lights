use rocket_contrib::json::{JsonValue};
use rocket::http::Status;
use serde::Serialize;

pub fn send_json_response<T: Serialize>(data: T, status_code: Status) -> JsonValue {
    json!({
        "data": data,
        "status": status_code.code,
    })
}
