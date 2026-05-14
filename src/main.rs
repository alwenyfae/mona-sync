mod handlers;
mod models;

use axum::{routing::get, Router};
use axum_server::tls_rustls::RustlsConfig;
use sqlx::sqlite::SqlitePoolOptions;
use std::env;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let server_ip = env::var("SERVER_IP").unwrap_or_else(|_| "0.0.0.0".to_string());
    let server_port = env::var("SERVER_PORT").unwrap_or_else(|_| "3000".to_string());
    let server_address = format!("{}:{}", server_ip, server_port);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    let app = Router::new()
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

    // Generate self-signed certificate
    let subject_alt_names = vec!["localhost".to_string(), server_ip.clone()];
    let cert = rcgen::generate_simple_self_signed(subject_alt_names)
        .expect("Failed to generate self-signed certificate");

    let cert_pem = cert.cert.pem();
    let key_pem = cert.key_pair.serialize_pem();

    let config = RustlsConfig::from_pem(cert_pem.into_bytes(), key_pem.into_bytes())
        .await
        .expect("Failed to create RustlsConfig");

    let addr: SocketAddr = server_address.parse().expect("Invalid server address");

    println!("🚀 Server starting on https://{}", server_address);

    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
