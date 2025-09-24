use axum::{
    extract::{State, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use serde_json::json;
use uuid::Uuid; // Add this import

use crate::api::create_jwt::create_jwt;
use crate::models::user_types::{SignupRequest, LoginPayload};
use argon2::PasswordHasher;
use argon2::{
    password_hash::{PasswordHash, PasswordVerifier, SaltString},
    Argon2,
};
use rand_core::OsRng;

pub async fn create_users(
    State(pool): State<PgPool>,
    Json(creds): Json<SignupRequest>,
) -> impl IntoResponse {
    if creds.username.is_empty() || creds.email.is_empty() || creds.password.is_empty() {
        return (StatusCode::BAD_REQUEST, Json(json!({ "error": "Invalid Credentials" })));
    }

    // check if username exists
    let exists = sqlx::query!("SELECT id FROM users WHERE username = $1", creds.username)
        .fetch_optional(&pool)
        .await
        .unwrap();

    if exists.is_some() {
        return (StatusCode::CONFLICT, Json(json!({ "error": "Username already exists" })));
    }

    // hash password
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(creds.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    // insert user
    match sqlx::query!(
        "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3)",
        creds.username,
        creds.email,
        password_hash
    )
    .execute(&pool)
    .await {
        Ok(_) => {
            let token = create_jwt(&creds.username).unwrap();
            (StatusCode::OK, Json(json!({ "token": token })))
        }
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": format!("Database error: {}", e) })))
        }
    }
}

pub async fn get_user(
    State(pool): State<PgPool>,
    Json(creds): Json<LoginPayload>,
) -> impl IntoResponse {
    let user = sqlx::query!(
        "SELECT username, password_hash FROM users WHERE username = $1",
        creds.email // or creds.username depending on your LoginPayload structure
    )
    .fetch_optional(&pool)
    .await;

    match user {
        Ok(Some(user)) => {
            let parsed_hash = PasswordHash::new(&user.password_hash).unwrap();
            if Argon2::default()
                .verify_password(creds.password.as_bytes(), &parsed_hash)
                .is_ok()
            {
                let token = create_jwt(&user.username).unwrap();
                (StatusCode::OK, Json(json!({ "token": token })))
            } else {
                (StatusCode::UNAUTHORIZED, Json(json!({ "error": "Wrong credentials" })))
            }
        }
        Ok(None) => {
            (StatusCode::UNAUTHORIZED, Json(json!({ "error": "Wrong credentials" })))
        }
        Err(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Database error" })))
        }
    }
}

pub async fn delete_user(
    Path(id): Path<Uuid>, // Change from i32 to Uuid
    State(pool): State<PgPool>,
) -> impl IntoResponse {
    let result = sqlx::query!("DELETE FROM users WHERE id = $1", id)
        .execute(&pool)
        .await;

    match result {
        Ok(result) => {
            if result.rows_affected() == 0 {
                (StatusCode::NOT_FOUND, Json(json!({ "error": "User not found" })))
            } else {
                (StatusCode::OK, Json(json!({ "message": "User deleted successfully" })))
            }
        }
        Err(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Database error" })))
        }
    }
}