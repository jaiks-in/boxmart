use axum::{
    extract::State,
    Json,
};
use sqlx::PgPool;
use bcrypt::{hash, DEFAULT_COST};
use crate::models::user_types::{SignupRequest, UserResponse};
use axum::http::StatusCode;

pub async fn signup(
    State(pool): State<PgPool>,
    Json(payload): Json<SignupRequest>,
) -> Result<Json<UserResponse>, (StatusCode, String)> {

    // Hash password
    let password_hash = hash(&payload.password, DEFAULT_COST)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to hash password".to_string()))?;

    // Set default user_type if empty

    // Insert into DB
    let user = sqlx::query_as!(
        UserResponse,
        r#"
        INSERT INTO users (username, email, password_hash, user_type, org_name)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, username, email, user_type, org_name
        "#,
        payload.username,
        payload.email,
        password_hash,
        payload.user_type,
        payload.org_name
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::BAD_REQUEST, format!("Database error: {}", e)))?;

    Ok(Json(user))
}
