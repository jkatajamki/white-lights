use actix_web::{HttpResponse};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct ApiStatus {
    status: String,
}

pub async fn get_status() -> HttpResponse {
    HttpResponse::Ok().json(ApiStatus{
        status: String::from("Up and running!"),
    })   
}
