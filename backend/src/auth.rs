use argonautica::Hasher;
use rand::Rng;
use std::env;

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