use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct BoxType {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateBoxType {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBoxType {
    pub name: Option<String>,
}
