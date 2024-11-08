use std::{env, path::Path};
use tracing::{error, info};

use crate::Config;

impl Config {
    pub fn load_environment() {
        dotenvy::dotenv().expect("Error loading .env");

        let environment = env::var("ENVIRONMENT").unwrap_or_else(|_| String::from("development"));

        match environment.as_str() {
            "development" => {
                info!("Running in Development mode");
                dotenvy::from_path(Path::new("environments/.env.development"))
                    .expect("Error loading .env.development")
            }
            "production" => {
                info!("Running in Production mode");
                dotenvy::from_path(Path::new("environments/.env.production"))
                    .expect("Error loading .env.production")
            }
            "test" => {
                info!("Running in Test mode");
                dotenvy::from_path(Path::new("environments/.env.test"))
                    .expect("Error loading .env.tests")
            }
            _ => {
                error!("Unknown environment: {}", environment);
            }
        }
    }
}
