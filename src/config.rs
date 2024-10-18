use dotenvy::Error as DotenvError;
use tower_http::cors::{Any, CorsLayer};

#[derive(Debug)]
#[allow(dead_code)]
pub enum ConfigError {
    DotenvError(DotenvError),
}

impl From<DotenvError> for ConfigError {
    fn from(e: DotenvError) -> Self {
        ConfigError::DotenvError(e)
    }
}

pub struct Config {}

impl Config {
    pub fn init() -> Result<(), ConfigError> {
        dotenvy::dotenv()?;
        Ok(())
    }

    pub fn cors() -> CorsLayer {
        CorsLayer::new()
            .allow_origin(
                "http://localhost:3000"
                    .parse::<axum::http::HeaderValue>()
                    .unwrap(),
            )
            .allow_methods(Any)
            .allow_headers(Any)
    }
}
