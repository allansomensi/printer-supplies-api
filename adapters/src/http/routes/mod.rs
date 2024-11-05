use axum::{routing::get, Router};

pub fn create_routes() -> Router {
    Router::new().nest("/api/v1", Router::new().nest("/test", test_route()))
}

pub fn test_route() -> Router {
    Router::new().route("/", get("test"))
}
