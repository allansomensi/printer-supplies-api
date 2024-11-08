use axum::Router;
use config::Config;
use domain::stock::ports::StockService;

mod toner;

use super::AppState;

pub fn api_routes<SS: StockService>(state: AppState<SS>) -> Router {
    Router::new()
        .nest(
            "/api/v1",
            Router::new().nest("/toners", toner::create_routes()),
        )
        .layer(Config::cors())
        .with_state(state)
}
