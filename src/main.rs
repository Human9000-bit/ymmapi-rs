use std::{path::PathBuf, sync::Arc};
use config::parse_config;
use tokio::task;
use webserver::build_rocket;
use yandex_music::YandexMusicClient;

mod config;
mod webserver;
mod ya_api;

#[tokio::main]
async fn main() {
    let rocket_instance = task::spawn(build_rocket());
    
    let config_path = PathBuf::from("config.toml");
    let config = task::spawn( async {
        parse_config(config_path).await});
    
    let rocket_instance = rocket_instance.await.unwrap();
    
    let handle = task::spawn(async {
        let _ = rocket_instance.launch().await;
    });
    
    let app_config = config.await.expect("Config error");
    handle.await;
}