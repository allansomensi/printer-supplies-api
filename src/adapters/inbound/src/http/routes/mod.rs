use axum::Router;
use config::Config;
use domain::stock::ports::StockService;

mod toner;

use super::AppState;

pub fn api_routes<BS: StockService>(state: AppState<BS>) -> Router {
    Router::new()
        .nest(
            "/api/v1",
            Router::new().nest("/toners", toner::create_routes()),
        )
        .layer(Config::cors())
        .with_state(state)
}
