mod config;
mod errors;
mod handlers;
mod models;
mod openapi;
mod routes;
mod server;

#[tokio::main]
async fn main() {
    println!("🌟 Printer Supplies API 🌟");

    match config::Config::init() {
        Ok(_) => {
            tracing::info!("✅ Configurations loaded!");
        }
        Err(e) => {
            tracing::error!("❌ Error loading configurations: {:?}", e);
            std::process::exit(1);
        }
    }
    server::run().await.unwrap();
}
