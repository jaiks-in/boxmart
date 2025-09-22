use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime,Utc};
#[derive(Debug, Deserialize)]
pub struct SignupRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub user_type:Option<String>,
    pub org_name:Option<String>,
}

// src/models/user.rs
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub user_type: Option<String>,
    pub org_name:Option<String>,
}

#[derive(Debug,Deserialize,Serialize,FromRow)]
pub struct LoginPayload{
    pub email:String,
    pub password:String
}
#[derive(Serialize)]
pub struct LoginResponse {
    pub message: String,
    pub user_id: i32,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[sqlx(rename = "password_hash")]
    pub password: String,  // this will map password_hash column
    pub user_type: String,
    pub org_name:Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
