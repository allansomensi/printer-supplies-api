use axum::{routing::post, Router};
use domain::stock::ports::StockService;

use crate::http::{handlers::create_toner::create_toner, AppState};

pub fn create_routes<BS: StockService>() -> Router<AppState<BS>> {
    Router::new().route("/", post(create_toner::<BS>))
}
