pub mod cors;
pub mod environment;
pub mod logger;

pub struct Config {}

impl Config {
    pub fn init() -> Result<(), dotenvy::Error> {
        Self::logger_init();
        environment::load_environment();
        Ok(())
    }
}
