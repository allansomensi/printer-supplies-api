mod config;
mod handlers;
mod models;
mod routes;
mod server;

#[tokio::main]
async fn main() {
    println!("🌟 Printer Supplies API 🌟");

    match config::Config::init() {
        Ok(_) => {
            tracing::info!("✅ Configurações carregadas!");
        }
        Err(e) => {
            tracing::error!("Não foi possível carregar as configurações: {:?}", e);
            std::process::exit(1);
        }
    }
    server::run().await.unwrap();
}
