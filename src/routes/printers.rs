use crate::handlers::printer;
use crate::models::database::AppState;
use axum::{routing::get, Router};
use std::sync::Arc;

pub fn create_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/count", get(printer::count_printers))
        .route("/:id", get(printer::search_printer))
        .route(
            "/",
            get(printer::show_printers)
                .post(printer::create_printer)
                .put(printer::update_printer)
                .delete(printer::delete_printer),
        )
        .with_state(state)
}
