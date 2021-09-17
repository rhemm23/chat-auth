use chrono::NaiveDateTime;
use super::schema::users;

#[derive(Queryable)]
pub struct User {
  pub id: i32,
  pub username: String,
  pub pass_hash: String,
  pub pass_salt: String,
  pub created_at: NaiveDateTime
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
  pub username: &'a str,
  pub pass_hash: &'a str,
  pub pass_salt: &'a str
}
