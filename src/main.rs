use std::{path::PathBuf, sync::Arc};
use config::parse_config;
use tokio::task;
use webserver::build_rocket;
use yandex_music::YandexMusicClient;

mod config;
mod webserver;

#[rocket::main]
async fn main() {
    let config_path = PathBuf::from("config.toml");
    let config = task::spawn( async {
        parse_config(config_path).await});
    
    let app_config = config.await.expect("Config error");
    let client = Arc::new(YandexMusicClient::new(&app_config.keys.ya_music));
    
    let rocket_instance = task::spawn(build_rocket(client));
    
    let rocket_instance = rocket_instance.await.unwrap();
    
    let handle = task::spawn(async move {
        let _ = rocket_instance.launch().await;
    });
    
    handle.await;
    
}