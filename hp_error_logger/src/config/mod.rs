use dotenv::dotenv;
use serde::Deserialize;
use color_eyre::Result;
use eyre::WrapErr;
use config::Config;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: i32,
}

impl Config {

    pub fn from_env() -> Result<Config> {
        dotenv().ok();

        let mut c = config::Config::new();

        c.merge(config::Environment::default())?;

        c.try_into()
        .context("Loading configration from e")
    }
}
