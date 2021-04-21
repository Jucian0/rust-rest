use crate::api_error::ApiError;
use crate::db;
use crate::schema::user;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
   pub password: String,
   pub created_at: NaiveDateTime,
   pub updated_at: Option<NaiveDateTime>,
}
