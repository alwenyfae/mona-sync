mod handlers;
mod models;

use axum::{
    Router,
    response::Html,
    routing::get,
};
use sqlx::sqlite::SqlitePoolOptions;

#[tokio::main]
async fn main() {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://database.db")
        .await
        .unwrap();

    let app = Router::new()
        .route("/", get(|| async { Html(include_str!("../index.html")) }))
        .route("/health", get(|| async { "Sync API is alive!" }))
        .route(
            "/api/sync/supply_items",
            get(handlers::pull_supply_items).post(handlers::push_supply_items),
        )
        .route(
            "/api/sync/medication_schedules",
            get(handlers::pull_medication_schedules).post(handlers::push_medication_schedules),
        )
        .route(
            "/api/sync/medication_intakes",
            get(handlers::pull_medication_intakes).post(handlers::push_medication_intakes),
        )
        .route(
            "/api/sync/blood_tests",
            get(handlers::pull_blood_tests).post(handlers::push_blood_tests),
        )
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Server starting on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
