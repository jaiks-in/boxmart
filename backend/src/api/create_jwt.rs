use chrono::{Duration, Utc};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use once_cell::sync::Lazy;
use crate::models::jwt_claims::Claims;
static SECRET: &[u8] = b"my_super_secret_key_32_bytes_min";

static ENCODING_KEY: Lazy<EncodingKey> = Lazy::new(|| EncodingKey::from_secret(SECRET));
static DECODING_KEY: Lazy<DecodingKey> = Lazy::new(|| DecodingKey::from_secret(SECRET));


pub fn create_jwt(user_id: &str) -> anyhow::Result<String> {
    let exp = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims { sub: user_id.to_string(), exp };
    let token = encode(&Header::default(), &claims, &ENCODING_KEY)?;
    Ok(token)
}

pub fn verify_jwt(token: &str) -> anyhow::Result<Claims> {
    let token_data = decode::<Claims>(token, &DECODING_KEY, &Validation::default())?;
    Ok(token_data.claims)
}
