use crate::{database::AppState, handlers::status};
use axum::{routing::get, Router};
use std::sync::Arc;

pub fn create_routes(state: Arc<AppState>) -> Router {
    Router::new().route("/", get(status::show_status).with_state(state))
}
