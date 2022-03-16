#[macro_use]
extern crate rocket;

use rocket::fs::{relative, NamedFile};
use std::path::PathBuf;

#[get("/")]
async fn index() -> Option<NamedFile> {
    let mut path = PathBuf::from(relative!(""));
    path.pop();
    path.push("public");
    if path.is_dir() {
        path.push("index.html");
    }

    NamedFile::open(path).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
