use axum::{extract::{Path, State}, Json};
use sqlx::PgPool;
use crate::models::box_type::{BoxType, CreateBoxType, UpdateBoxType};

pub async fn get_box_types(State(pool): State<PgPool>) -> Json<Vec<BoxType>> {
    let rows = sqlx::query_as::<_, BoxType>("SELECT * FROM box_types")
        .fetch_all(&pool)
        .await
        .unwrap();
    Json(rows)
}

pub async fn get_box_type(Path(id): Path<i32>, State(pool): State<PgPool>) -> Json<BoxType> {
    let row = sqlx::query_as::<_, BoxType>("SELECT * FROM box_types WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .unwrap();
    Json(row)
}

pub async fn create_box_type(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateBoxType>,
) -> Json<BoxType> {
    let row = sqlx::query_as::<_, BoxType>(
        "INSERT INTO box_types (name) VALUES ($1) RETURNING *"
    )
    .bind(payload.name)
    .fetch_one(&pool)
    .await
    .unwrap();
    Json(row)
}

pub async fn update_box_type(
    Path(id): Path<i32>,
    State(pool): State<PgPool>,
    Json(payload): Json<UpdateBoxType>,
) -> Json<BoxType> {
    let row = sqlx::query_as::<_, BoxType>(
        "UPDATE box_types SET name = COALESCE($1, name) WHERE id = $2 RETURNING *"
    )
    .bind(payload.name)
    .bind(id)
    .fetch_one(&pool)
    .await
    .unwrap();
    Json(row)
}

pub async fn delete_box_type(Path(id): Path<i32>, State(pool): State<PgPool>) -> Json<String> {
    sqlx::query("DELETE FROM box_types WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .unwrap();
    Json(format!("BoxType {} deleted", id))
}
