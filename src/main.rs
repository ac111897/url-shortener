mod redirect;
mod shortener_routes;

use rocket::fs::{FileServer, relative};

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![redirect::get])
    .mount("/s", routes![shortener_routes::create_url])
    .mount("/", FileServer::from(relative!("static")))
}
