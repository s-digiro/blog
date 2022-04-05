#[macro_use] extern crate rocket;

use rocket::fs::{FileServer, relative};
use std::path::Path;
use rocket::fs::NamedFile;

#[get("/dist/bundle.js")]
async fn bundle() -> Option<NamedFile> {
    let path = Path::new(relative!("frontend/dist/bundle.js"));
    NamedFile::open(path).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", rocket::routes![bundle])
        .mount("/", FileServer::from(relative!("frontend/public")))
}
