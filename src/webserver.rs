use std::sync::Arc;
use rocket:: State;

use rocket::{fs::FileServer, get, Build, Rocket};
use serde_json::json;
use yandex_music::YandexMusicClient;

pub async fn build_rocket(client: Arc<YandexMusicClient>) -> Rocket<Build> {
    rocket::build()
        .manage(client)
        .mount("/", FileServer::from("static"))
}

#[get("/song/<track_id>")]
async fn get_song_by_id(track_id: i32, client: &State<Arc<YandexMusicClient>>) {
    let download_link = &client.clone().get_track_download_info(track_id).await.unwrap();
    let download_link = &download_link[0];
    let download_link = &download_link.download_info_url;
    
    let track = &client.get_track(track_id).await.unwrap();
    let track = &track[0];
    
    let track_dur_mins = &track.duration_ms.unwrap() / 1000 / 60;
    let track_dur_secs = &(track.duration_ms.unwrap() / 1000) - (&track_dur_mins * 60);
    
    let track_info = json!({
        "track_id": track_id,
        "title": &track.title.unwrap(),
        "artist": artist,
        "img": &track.cover_uri.unwrap(),
        "duration": &track.duration_ms.unwrap() / 1000,
        "minutes": track_dur_mins,
        "seconds": track_dur_secs,
        "album": album,
        "download_link": download_link
    });
}