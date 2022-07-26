use rocket::response::Redirect;

#[get("/<code>")]
pub async fn get(code: String) -> Redirect {
    return Redirect::to(uri!("https://google.com")); // just for show rn
}
