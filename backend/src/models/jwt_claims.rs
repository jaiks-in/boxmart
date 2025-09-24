use serde::{Serialize,Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user email or id
    pub exp: usize,  // expiration as timestamp
}
