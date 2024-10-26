use crate::handlers::supplies::drum;
use crate::models::database::AppState;
use axum::{routing::get, Router};
use std::sync::Arc;

pub fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/count", get(drum::count_drums))
        .route("/:id", get(drum::search_drum))
        .route(
            "/",
            get(drum::show_drums)
                .post(drum::create_drum)
                .put(drum::update_drum)
                .delete(drum::delete_drum),
        )
        .with_state(state)
}
