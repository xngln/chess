use argonautica::Hasher;
use async_graphql::Result;
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