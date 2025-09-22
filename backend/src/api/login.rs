use axum::{
    extract::{State, Json},
    response::IntoResponse,
};
use sqlx::PgPool;
use bcrypt::verify;
use serde_json::json;
use crate::models::user_types::{LoginPayload, User};

pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginPayload>,
) -> impl IntoResponse {
    let user_result = sqlx::query_as::<_, User>(
        r#"
        SELECT * FROM users WHERE email = $1
        "#
    )
    .bind(payload.email.trim())
    .fetch_one(&pool)
    .await;

    match user_result {
        Ok(user) => {
            let valid = verify(&payload.password, &user.password).unwrap_or(false);

            if valid {
                Json(json!({
                    "message": "Login successful",
                    "user_id": user.id,
                    "username": user.username,
                    "email": user.email,
                    "user_type":user.user_type
                }))
            } else {
                Json(json!({ "error": "Invalid password" }))
            }
        }
        Err(err) => {
            eprintln!("Login error: {:?}", err);
            Json(json!({ "error": "User not found" }))
        }
    }
}
