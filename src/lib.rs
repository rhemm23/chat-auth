#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use self::models::{ User, NewUser };

pub fn establish_connection() -> PgConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL")
    .expect("DATABASE_URL must be set");

  PgConnection::establish(&database_url)
    .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_user<'a>(conn: &PgConnection, phone_number: &'a str) -> User {
  let new_user = NewUser {
    phone_number: phone_number
  };

  diesel::insert_into(schema::users::table)
    .values(&new_user)
    .get_result(conn)
    .expect("Error saving new user")
}
