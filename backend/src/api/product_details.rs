use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use crate::models::paper_type::Brand;
pub async fn get_all_brands(
    State(pool): State<PgPool>
) -> impl IntoResponse {
    let brands = sqlx::query_as::<_, Brand>(
        "SELECT * FROM brands ORDER BY name, variety, gsm,cost_per_kg"
    )
    .fetch_all(&pool)
    .await;

    match brands {
        Ok(list) => Ok(Json(list)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e))),    }
}
