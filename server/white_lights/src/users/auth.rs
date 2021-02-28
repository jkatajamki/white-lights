use actix_web::{dev::ServiceRequest};
use actix_web_httpauth::extractors::bearer::{BearerAuth};
use alcoholic_jwt::{token_kid, validate, Validation, JWKS};
use serde::{Serialize, Deserialize};
use reqwest::{get, Error as ReqwestError};
use crate::wl_error::WLError;
use crate::env_vars;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

async fn fetch_jwks(uri: &str) -> Result<JWKS, ReqwestError> {
    let res = get(uri).await;

    let val = match res {
        Ok(x) => x,
        Err(err) => {
            return Err(err);
        }
    };

    val.json::<JWKS>().await
}

async fn get_jwks(authority: &String) -> Result<alcoholic_jwt::JWKS, WLError> {
    let uri = format!("{}{}", authority.as_str(), ".well-known/jwks.json");
    match fetch_jwks(&uri).await {
        Ok(res) => Ok(res),
        Err(err) => {
            eprintln!("Error fetching jwks! {}", err);
            return Err(WLError::JWKSFetchError);
        }
    }
}

pub async fn validate_token(token: &str) -> Result<bool, WLError> {
    let authority = env_vars::get_authority();

    let jwks_res = get_jwks(&authority).await;
    let jwks = match jwks_res {
        Ok(res) => res,
        Err(err) => {
            eprintln!("Error fetching jwks! {}", err);
            return Err(WLError::JWKSFetchError);
        }
    };

    let validations = vec![Validation::Issuer(authority), Validation::SubjectPresent];

    let kid_res = match token_kid(&token) {
        Ok(res) => res,
        Err(err) => {
            eprintln!("Error matching token kid! {:?}", err);
            return Err(WLError::JWKSFetchError);
        }
    };
    let kid = match kid_res {
        Some(val) => val,
        None => {
            eprintln!("Error decoding token kid!");
            return Err(WLError::JWKSFetchError);
        }
    };

    let jwk_res = jwks.find(&kid);
    let jwk = match jwk_res {
        Some(val) => val,
        None => {
            eprintln!("Error finding specified key in set!");
            return Err(WLError::JWKSFetchError);
        }
    };

    let res = validate(token, jwk, validations);
    Ok(res.is_ok())
}

pub async fn validate_auth(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, WLError> {
    match validate_token(credentials.token()).await {
        Ok(res) => {
            if res == true {
                Ok(req)
            } else {
                Err(WLError::AuthenticationError)
            }
        },
        Err(err) => {
            eprintln!("Error validating token! {}", err);

            return Err(WLError::AuthenticationError)
        }
    }
}
