use argonautica::{Hasher, Verifier};
use async_graphql::{ErrorExtensions, Result};
use crate::error::Error;
use jsonwebtoken as jwt;
use serde;
use std::env;
use std::fmt;

const BEARER: &str = "Bearer ";

#[derive(Clone, PartialEq)]
pub enum Role {
    User,
}

impl Role {
    pub fn from_str(role: &str) -> Role {
        match role {
            _ => Role::User,
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::User => write!(f, "User"),
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Claims {
    sub: String,
    role: String,
    exp: usize,
}

pub fn hash_password(password: String) -> String {
    let argo2_key = env::var("ARGO2_KEY").expect("Couldn't get argo2 hash key");

    Hasher::new()
        .with_password(password)
        .with_secret_key(argo2_key)
        .hash()
        .unwrap()
}

pub fn verify_password(password: String, hash: String) -> Result<bool, Error> {
    let argo2_key = env::var("ARGO2_KEY").expect("Couldn't get argo2 hash key");

    let mut verifier = Verifier::new();

    let is_valid = verifier
        .with_secret_key(argo2_key)
        .with_hash(hash)
        .with_password(password)
        .verify()
        .expect("Unable to verify password");

    Ok(is_valid)
}

pub fn create_jwt(user_id: &str, role: &Role) -> Result<String> {
    let jwt_secret = env::var("JWT_SECRET").expect("Failed to get JWT_SECRET from env");

    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::seconds(30))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id.to_owned(),
        role: role.to_string(),
        exp: expiration as usize,
    };

    let header = jwt::Header::new(jwt::Algorithm::HS512);
    jwt::encode(&header, &claims, &jwt::EncodingKey::from_secret(jwt_secret.as_bytes()))
        .map_err(|_| Error::JWTTokenCreation.extend())
}