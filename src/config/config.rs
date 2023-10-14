use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::{env, fmt};
use std::fmt::Formatter;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SETTING: Setting = Setting::new().expect("Failed to load config");
}

#[derive(Deserialize, Debug, Clone)]
pub struct Setting {
    pub environment: String,
    pub server: Server,
    pub logger: Logger,
    pub database: Database,
    pub auth: Auth,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    pub port: u32
}

#[derive(Debug, Clone, Deserialize)]
pub struct Logger {
    pub level: String
}

#[derive(Debug, Clone, Deserialize)]
pub struct Database {
    pub uri: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Auth {
    pub secret: String
}

impl Setting {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let mut builder = Config::builder()
            .add_source(File::with_name("storage/configs/development"))
            .add_source(File::with_name(&format!("storage/configs/{run_mode}")).required(false))
            .add_source(File::with_name("storage/configs/local").required(false))
            .add_source(Environment::default().separator("__"));

        if let Ok(port) = env::var("PORT") {
            builder = builder.set_override("server.port", port)?;
        }

        builder
            .build()?
            // Deserialize and thus freeze the entire config
            .try_deserialize()
    }
}

impl fmt::Display for Server {
    fn fmt(
        &self,
        f: &mut Formatter
    ) -> fmt::Result {
        write!(f, "http://127.0.0.1:{}", &self.port)
    }
}
