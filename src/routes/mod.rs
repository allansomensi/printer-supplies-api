pub mod brands;
pub mod migrations;
pub mod movements;
pub mod printers;
pub mod status;
pub mod supplies;
pub mod swagger;

use crate::{config::Config, database::AppState};
use axum::Router;
use std::sync::Arc;

pub fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .nest(
            "/api/v1",
            Router::new()
                .nest("/status", status::create_routes(state.clone()))
                .nest("/migrations", migrations::create_routes(state.clone()))
                .nest("/printers", printers::create_routes(state.clone()))
                .nest("/supplies", supplies::create_routes(state.clone()))
                .nest("/movements", movements::create_routes(state.clone()))
                .nest("/brands", brands::create_routes(state)),
        )
        .merge(swagger::swagger_routes())
        .layer(Config::cors())
}
