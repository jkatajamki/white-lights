use actix_web::{HttpResponse, error::ResponseError, http::StatusCode};
use serde::{Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WLError {
    #[error("Requested entity was not found")]
    NotFound,
    #[error("Access denied to requested entity")]
    Forbidden,
    #[error("Failed to process entity passed in request")]
    UnprocessableEntity,
    #[error("Unknown internal error occurred")]
    InternalServerError,
    #[error("Registration failed: requested email is already registered")]
    RegistrationErrorEmailTaken,
}

#[derive(Serialize)]
struct WLErrorResponse {
    code: u16,
    error: String,
    message: String,
}

impl WLError {
    pub fn name(&self) -> String {
        match self {
            Self::NotFound => "NotFound".to_string(),
            Self::Forbidden => "Forbidden".to_string(),
            Self::UnprocessableEntity => "UnprocessableEntity".to_string(),
            Self::InternalServerError => "InternalServerError".to_string(),
            Self::RegistrationErrorEmailTaken => "RegistrationErrorEmailTaken".to_string(),
        }
    }
}

impl ResponseError for WLError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::UnprocessableEntity => StatusCode::UNPROCESSABLE_ENTITY,
            Self::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::RegistrationErrorEmailTaken => StatusCode::CONFLICT,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = WLErrorResponse{
            code: status_code.as_u16(),
            message: self.to_string(),
            error: self.name(),
        };

        HttpResponse::build(status_code).json(error_response)
    }
}
