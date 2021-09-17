#[macro_use] extern crate rocket;

extern crate chat_auth;
extern crate diesel;

use self::chat_auth::*;

#[get("/")]
fn index() -> &'static str {
  "Hello world!"
}

#[launch]
fn rocket() -> _ {
  rocket::build().mount("/", routes![index])
}
