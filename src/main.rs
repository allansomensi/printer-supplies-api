use axum::{routing::get, Router};
use handlers::{drum, status, toner};
use sqlx::postgres::PgPoolOptions;
use std::env;

mod handlers;
mod models;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").unwrap();

    println!("🌟 Printer Supplies API 🌟");

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Conectado ao banco de dados");
            pool
        }
        Err(e) => {
            eprintln!("❌ Erro ao se conectar ao banco de dados: {e}");
            std::process::exit(1);
        }
    };

    let app = Router::new()
        // Status
        .route("/api/v1/status", get(status::show_status))
        // Toner
        .route(
            "/api/v1/toners",
            get(toner::show_toners)
                .post(toner::create_toner)
                .delete(toner::delete_toner),
        )
        .route("/api/v1/toner-count", get(toner::count_toners))
        // Drums
        .route(
            "/api/v1/drums",
            get(drum::show_drums)
                .post(drum::create_drum)
                .delete(drum::delete_drum),
        )
        .route("/api/v1/drum-count", get(drum::count_drums))
        .with_state(pool);

    let addr = env::var("HOST").expect("Erro ao carregar env HOST");
    let listener = match tokio::net::TcpListener::bind(&addr).await {
        Ok(listener) => {
            println!("✅ Servidor iniciado em {}", &addr);
            listener
        }
        Err(e) => {
            eprintln!("❌ Erro ao iniciar o servidor: {e}");
            std::process::exit(1)
        }
    };

    axum::serve(listener, app).await.unwrap();
}
