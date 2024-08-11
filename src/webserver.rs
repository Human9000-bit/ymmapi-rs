use rocket::{fs::FileServer, Build, Rocket};

pub async fn build_rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", FileServer::from("static"))
}