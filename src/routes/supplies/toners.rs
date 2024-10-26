use crate::handlers::supplies::toner;
use crate::models::database::AppState;
use axum::{routing::get, Router};
use std::sync::Arc;

pub fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/count", get(toner::count_toners))
        .route("/:id", get(toner::search_toner))
        .route(
            "/",
            get(toner::show_toners)
                .post(toner::create_toner)
                .put(toner::update_toner)
                .delete(toner::delete_toner),
        )
        .with_state(state)
}
