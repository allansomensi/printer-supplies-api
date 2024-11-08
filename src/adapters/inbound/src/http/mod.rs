use anyhow::Context;
use std::sync::Arc;
use tokio::net;

use domain::stock::ports::StockService;

mod handlers;
mod responses;
mod routes;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpServerConfig<'a> {
    pub host: &'a str,
    pub port: &'a str,
}

#[derive(Debug, Clone)]
struct AppState<BS: StockService> {
    toner_service: Arc<BS>,
}

pub struct HttpServer {
    router: axum::Router,
    listener: net::TcpListener,
}

impl HttpServer {
    pub async fn new(
        stock_service: impl StockService,
        config: HttpServerConfig<'_>,
    ) -> anyhow::Result<Self> {
        let state = AppState {
            toner_service: Arc::new(stock_service),
        };

        let router = routes::api_routes(state);

        let server_addr = format!("{}:{}", config.host, config.port);

        let listener = match tokio::net::TcpListener::bind(&server_addr).await {
            Ok(listener) => listener,
            Err(e) => {
                tracing::error!("❌ Error starting the server: {e}");
                std::process::exit(1)
            }
        };

        Ok(Self { router, listener })
    }

    pub async fn run(self) -> anyhow::Result<()> {
        tracing::info!(
            "✅ Server started at: {}",
            self.listener.local_addr().unwrap()
        );
        axum::serve(self.listener, self.router)
            .await
            .context("Error starting the server")?;
        Ok(())
    }
}
