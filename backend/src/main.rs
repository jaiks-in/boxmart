use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::{CorsLayer, Any, AllowOrigin, AllowMethods, AllowHeaders};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use std::env;
use std::time::Duration;
mod api;
mod models;
use crate::api::hello::hello;
use crate::api::signup::signup;
use crate::api::login::login;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();
     let cors = CorsLayer::new()
        .allow_origin(Any) // Allow all origins
        .allow_methods(Any) // Allow all HTTP methods
        .allow_headers(Any); // Allow all headers
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in environment or .env file");
    
    println!("🔧 Connecting to database...");
    
    // Create connection pool with proper configuration
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");
    
    println!("✅ Database connection established successfully!");

    let app = Router::new()
        .route("/api/auth/signup", post(signup))
        .route("/api/auth/login",post(login)).layer(cors)
        .with_state(pool);

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to address");
    
    println!("🚀 Server running at http://localhost:3000");
    
    axum::serve(listener, app.into_make_service())
        .await
        .expect("Server failed to start");
    
    Ok(())
}