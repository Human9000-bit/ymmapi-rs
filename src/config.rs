use std::path::PathBuf;

use serde::Deserialize;
use tokio::fs;

pub async fn parse_config(filename: PathBuf) -> AppConfig {
    let contents = fs::read_to_string(filename).await.expect("failed to read config (config.toml) file");
    let contents: &str = &contents;
    let config: AppConfig = toml::from_str(contents).unwrap();
    config
}

#[derive(Deserialize)]
pub struct AppConfig {
    pub keys: Keys
}

#[derive(Deserialize)]
pub struct Keys {
    pub ya_music: String,
    pub lastfm: Option<String>,
    pub lastfm_secret: Option<String>
}