use chrono::{DateTime, FixedOffset, Utc};
use dotenvy::Error as DotenvError;
use std::fmt;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;

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

    pub fn logger_init() {
        struct UtcFormattedTime;

        impl FormatTime for UtcFormattedTime {
            fn format_time(&self, writer: &mut Writer<'_>) -> fmt::Result {
                let brasilia_offset = FixedOffset::west_opt(3 * 3600).unwrap();
                let now: DateTime<FixedOffset> = Utc::now().with_timezone(&brasilia_offset);
                let formatted_time = now.format("%d/%m/%Y %H:%M:%S").to_string();
                write!(writer, "{}", formatted_time)
            }
        }

        tracing_subscriber::fmt()
            .pretty()
            .with_timer(UtcFormattedTime)
            .with_file(false)
            .with_line_number(false)
            .with_target(false)
            .init();
    }
}
