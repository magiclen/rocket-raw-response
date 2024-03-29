#[macro_use]
extern crate rocket;

use std::{io::ErrorKind, path::Path};

use rocket::http::Status;
use rocket_raw_response::RawResponse;

#[get("/")]
async fn view() -> Result<RawResponse, Status> {
    let path = Path::join(Path::new("examples"), Path::join(Path::new("images"), "image(貓).jpg"));

    RawResponse::from_file(path, None::<String>, None).await.map_err(|err| {
        if err.kind() == ErrorKind::NotFound {
            Status::NotFound
        } else {
            Status::InternalServerError
        }
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![view])
}
