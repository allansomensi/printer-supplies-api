mod routes;

pub async fn run() -> Result<(), axum::Error> {
    dotenvy::dotenv().unwrap();

    let app = routes::create_routes();

    let addr = std::env::var("HOST").expect("Failed to load HOST");
    let listener = match tokio::net::TcpListener::bind(&addr).await {
        Ok(listener) => {
            tracing::info!("✅ Server started at: {}", &addr);
            listener
        }
        Err(e) => {
            tracing::error!("❌ Error starting the server: {e}");
            std::process::exit(1)
        }
    };

    axum::serve(listener, app)
        .await
        .expect("Error starting the server");
    Ok(())
}
