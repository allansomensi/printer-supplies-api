use dotenvy::Error as DotenvError;

pub mod cors;
pub mod logger;

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
        Self::logger_init();
        dotenvy::dotenv()?;
        Ok(())
    }
}