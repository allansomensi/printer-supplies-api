use crate::models::database::AppState;
use axum::Router;
use std::sync::Arc;

pub mod drums;
pub mod toners;

pub fn create_routes(state: Arc<AppState>) -> Router {
    Router::new().nest(
        "/",
        Router::new()
            .nest("/toners", toners::create_routes(state.clone()))
            .nest("/drums", drums::create_routes(state)),
    )
}