use argonautica::{Hasher, input::Salt};
use async_graphql::{ErrorExtensions, Result};
use crate::error::Error;
use jsonwebtoken as jwt;
use rand::Rng;
use serde;
use std::env;
use std::fmt;

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
    let mut hasher = Hasher::default();
    let argo2_key = env::var("ARGO2_KEY").expect("Couldn't get argo2 hash key");

    hasher
        .with_password(password)
        .with_secret_key(argo2_key)
        .hash()
        .unwrap()
}

pub fn generate_rand_salt() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                           abcdefghijklmnopqrstuvwxyz\
                           0123456789)(*&^%$#@!~";
    
    const PASSWORD_LEN: usize = 30;
    let mut rng = rand::thread_rng();

    let salt: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    salt
}

pub fn create_jwt(user_id: &str, role: &Role) -> Result<String, Error> {
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
        .map_err(|_| Error::JWTTokenCreationError)
}