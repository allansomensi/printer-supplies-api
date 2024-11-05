use crate::handlers::brand;
use axum::{routing::get, Router};
use infra::database::AppState;
use std::sync::Arc;

pub fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/count", get(brand::count_brands))
        .route("/:id", get(brand::search_brand))
        .route(
            "/",
            get(brand::show_brands)
                .post(brand::create_brand)
                .put(brand::update_brand)
                .delete(brand::delete_brand),
        )
        .with_state(state)
}
