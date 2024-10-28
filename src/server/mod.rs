use std::sync::Arc;
use tracing::{error, info};

use crate::{models::database::AppState, routes};

pub async fn run() -> Result<(), axum::Error> {
    let database_url = std::env::var("DATABASE_URL").expect("Failed to load DATABASE_URL");

    let pool = match sqlx::PgPool::connect(&database_url).await {
        Ok(pool) => {
            info!("✅ Connected to the database");
            pool
        }
        Err(e) => {
            error!("❌ Error connecting to the database: {e}");
            std::process::exit(1);
        }
    };

    let app = routes::create_routes(Arc::new(AppState { db: pool.clone() }));

    let addr = std::env::var("HOST").expect("Failed to load HOST");
    let listener = match tokio::net::TcpListener::bind(&addr).await {
        Ok(listener) => {
            info!("✅ Server started at: {}", &addr);
            listener
        }
        Err(e) => {
            error!("❌ Error starting the server: {e}");
            std::process::exit(1)
        }
    };

    axum::serve(listener, app)
        .await
        .expect("Error starting the server");
    Ok(())
}
