use anyhow::Error;

pub mod cors;
pub mod environment;
pub mod logger;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub server_host: String,
    pub server_port: String,
    pub database_url: String,
    pub rust_log_file: String,
    pub rust_log_console: String,
    pub environment: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            environment: String::new(),
            server_host: String::new(),
            server_port: String::new(),
            database_url: String::new(),
            rust_log_console: String::new(),
            rust_log_file: String::new(),
        }
    }
}

impl Config {
    pub fn init() -> Result<Config, Error> {
        Self::load_environment();
        let mut config = Self::default();
        config.from_env()?;
        Self::logger_init();
        Ok(config)
    }

    fn from_env(&mut self) -> Result<(), Error> {
        self.environment = std::env::var("ENVIRONMENT")?;
        self.server_host = std::env::var("SERVER_HOST")?;
        self.server_port = std::env::var("SERVER_PORT")?;
        self.database_url = std::env::var("DATABASE_URL")?;
        self.rust_log_console = std::env::var("RUST_LOG_CONSOLE")?;
        self.rust_log_file = std::env::var("RUST_LOG_FILE")?;
        Ok(())
    }
}
