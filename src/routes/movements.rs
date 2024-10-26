use crate::handlers::movement;
use crate::models::database::AppState;
use axum::{routing::get, Router};
use std::sync::Arc;

pub fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/count", get(movement::count_movements))
        .route("/:id", get(movement::search_movement))
        .route(
            "/",
            get(movement::show_movements)
                .post(movement::create_movement)
                .put(movement::update_movement)
                .delete(movement::delete_movement),
        )
        .with_state(state)
}
