use tower_http::cors::{Any, CorsLayer};

use super::Config;

impl Config {
    pub fn cors() -> CorsLayer {
        let origins = [
            "http://127.0.0.1:3000".parse().unwrap(),
            "http://localhost:3000".parse().unwrap(),
        ];

        CorsLayer::new()
            .allow_origin(origins)
            .allow_methods(Any)
            .allow_headers(Any)
    }
}
