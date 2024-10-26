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
