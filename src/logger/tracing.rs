use std::env;
use crate::config::config::SETTING;

pub fn logger() {
    if env::var_os("RUST_LOG").is_none() {
        let level = SETTING.logger.level.as_str();
        let env = format!("redis-rust-rest={level}, tower-http={level}");
        env::set_var("RUST_LOG", env)
    }

    tracing_subscriber::fmt::init()
}