use axum::{Json, extract::State, routing::post, Router};
use sqlx::PgPool;

pub async fn create_product(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateProductRequest>,
) -> Result<Json<Product>, String> {
    iif payload.product_type=="duplex"{
        

    }
    let final_cost = payload.material_cost + payload.labour_cost + payload.margin;

    let product = sqlx::query_as::<_, Product>(
        r#"
        INSERT INTO products 
        (user_id, product_type, name, description,
        length, width, height,
        paper_gsm, paper_type, roll_type, ply,
        has_printing, printing_type, has_lamination, lamination_type,
        material_cost, labour_cost, margin, final_cost,
        has_logo, is_urgent)
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,
                $12,$13,$14,$15,$16,$17,$18,$19,$20,$21)
        RETURNING *
        "#
    )
    .bind(payload.user_id)
    .bind(payload.product_type.to_string())
    .bind(payload.name)
    .bind(payload.description)
    .bind(payload.length)
    .bind(payload.width)
    .bind(payload.height)
    .bind(payload.paper_gsm)
    .bind(payload.paper_type)
    .bind(payload.roll_type)
    .bind(payload.ply)
    .bind(payload.has_printing)
    .bind(payload.printing_type)
    .bind(payload.has_lamination)
    .bind(payload.lamination_type)
    .bind(payload.material_cost)
    .bind(payload.labour_cost)
    .bind(payload.margin)
    .bind(final_cost)
    .bind(payload.has_logo)
    .bind(payload.is_urgent)
    .fetch_one(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(Json(product))
}
