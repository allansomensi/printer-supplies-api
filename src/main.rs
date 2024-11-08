use domain::stock::service::Service;
use inbound::http::{HttpServer, HttpServerConfig};
use outbound::sqlite::Sqlite;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("\n---🌟 Printer Supplies API 🌟---\n");

    let config = match config::Config::init() {
        Ok(config) => {
            tracing::info!(environment = %config.environment);
            tracing::info!(file_logger = %config.rust_log_file);
            tracing::info!(console_logger = %config.rust_log_console);
            config
        }
        Err(e) => {
            tracing::error!("❌ Error loading configurations: {:?}", e);
            std::process::exit(1);
        }
    };

    let sqlite = Sqlite::new(&config.database_url).await?;
    let stock_service = Service::new(sqlite);

    let server_config = HttpServerConfig {
        host: &config.server_host,
        port: &config.server_port,
    };

    let http_server = HttpServer::new(stock_service, server_config).await?;
    http_server.run().await
}
