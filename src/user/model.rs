use crate::api_error::ApiError;
use crate::db;
use crate::schema::user;
use argon2::Config;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::db::LoadPaginated;
use crate::{sort_by, filter};


#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "user"]
pub struct UserMessage {
   pub email: String,
   pub password: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "user"]
pub struct User {
   pub id: Uuid,
   pub email: String,
   #[serde(skip_serializing)]
   pub password: String,
   pub created_at: NaiveDateTime,
   pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct Params {
   pub email: Option<String>,
   pub sort_by: Option<String>,
   #[serde(rename = "created_at[gte]")]
   pub created_at_gte: Option<NaiveDateTime>,
   #[serde(rename = "created_at[gte]")]
   pub created_at_lte: Option<NaiveDateTime>,
   #[serde(rename = "updated_at[gte]")]
   pub updated_at_gte: Option<NaiveDateTime>,
   #[serde(rename = "updated_at[gte]")]
   pub updated_at_lte: Option<NaiveDateTime>,
   pub page:Option<i64>,
   pub page_size:Option<i64>
}

impl User {
   pub fn find_all(params: Params) -> Result<(Vec<Self>,i64,i64), ApiError> {
      let conn = db::connection()?;

      let mut query = user::table.into_boxed();

      query = filter!(query,
         (user::email, @like, params.email),
         (user::created_at, @ge, params.created_at_gte),
         (user::created_at, @le, params.created_at_lte),
         (user::updated_at, @ge, params.updated_at_gte),
         (user::updated_at, @le, params.updated_at_lte)
     );

      query = sort_by!(query, params.sort_by,
         ("id", user::id),
         ("email",user::email),
         ("created_at", user::created_at),
         ("updated_at", user::updated_at)
      );

      let (users, total_pages, total) = query
      .load_with_pagination(&conn, params.page, params.page_size)?;

      Ok((users, total_pages, total))
   }

   pub fn find(id: Uuid) -> Result<Self, ApiError> {
      let conn = db::connection()?;

      let user = user::table.filter(user::id.eq(id)).first(&conn)?;

      Ok(user)
   }

   pub fn create(user: UserMessage) -> Result<Self, ApiError> {
      let conn = db::connection()?;

      let mut user = User::from(user);
      user.hash_password()?;
      let user = diesel::insert_into(user::table)
         .values(user)
         .get_result(&conn)?;

      Ok(user)
   }

   pub fn update(id: Uuid, user: UserMessage) -> Result<Self, ApiError> {
      let conn = db::connection()?;

      let user = diesel::update(user::table)
         .filter(user::id.eq(id))
         .set(user)
         .get_result(&conn)?;

      Ok(user)
   }

   pub fn delete(id: Uuid) -> Result<usize, ApiError> {
      let conn = db::connection()?;

      let res = diesel::delete(user::table.filter(user::id.eq(id))).execute(&conn)?;

      Ok(res)
   }

   pub fn hash_password(&mut self) -> Result<(), ApiError> {
      let salt: [u8; 32] = rand::thread_rng().gen();
      let config = Config::default();

      self.password = argon2::hash_encoded(self.password.as_bytes(), &salt, &config)
         .map_err(|e| ApiError::new(500, format!("Failed to hash password: {}", e)))?;

      Ok(())
   }

   pub fn verify_password(&self, password: &[u8]) -> Result<bool, ApiError> {
      argon2::verify_encoded(&self.password, password)
         .map_err(|e| ApiError::new(500, format!("Failed to verify password: {}", e)))
   }

   pub fn find_by_email(email: String) -> Result<Self, ApiError> {
      let conn = db::connection()?;

      let user = user::table.filter(user::email.eq(email)).first(&conn)?;

      Ok(user)
   }
}

impl From<UserMessage> for User {
   fn from(user: UserMessage) -> Self {
      User {
         id: Uuid::new_v4(),
         email: user.email,
         password: user.password,
         created_at: Utc::now().naive_utc(),
         updated_at: None,
      }
   }
}
