use axum::{routing::get, Router};
use sqlx::postgres::PgPoolOptions;

#[derive(Clone)]
struct AppState {
    db: sqlx::PgPool,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").expect("Falta DATABASE_URL en el .env");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error al conectar a la base de datos");

    let state = AppState { db: pool };

    let app = Router::new()
        .route("/api/health", get(health_check))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();

    println!("Servidor corriendo en Supabase");
    axum::serve(listener, app).await.unwrap();   
}

async fn health_check() -> &'static str {
    "OK"
}