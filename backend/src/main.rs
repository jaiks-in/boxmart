use axum::{Router, routing::{get, post, delete}};
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{CorsLayer, Any};
use tokio::net::TcpListener;
use std::env;
use crate::api::users::{create_users, get_user, delete_user};
mod models;
mod api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect DB");

    println!("DB Connected!");

    // Combine all routes here
    let app = Router::new()
        .route("/api/auth/signup", post(create_users))
        .route("/api/auth/login", post(get_user))
        .route("/api/users/:id", delete(delete_user))
        .layer(cors)
        .with_state(pool); // Add this line to provide database pool state

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    println!("Running at http://localhost:3000");

    axum::serve(listener, app).await?; // Use await? instead of unwrap()
    Ok(())
}