use actix_session::Session;
use crate::api_error::ApiError;
use crate::user::model::UserMessage;
use crate::user::Params;
use crate::user::User;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;
use uuid::Uuid;

#[get("/users")]
async fn find_all(params: web::Query<Params>, session:Session) -> Result<HttpResponse, ApiError> {
   let id: Option<Uuid> = session.get("user_id")?;

   if let Some(id) = id {      
      let users = User::find_all(params.into_inner())?;
      Ok(HttpResponse::Ok().json(users))
   } else {
      Err(ApiError::new(401, "Unauthorized".to_string()))
   }
}


#[get("/users/{id}")]
async fn find(id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
   let user = User::find(id.into_inner())?;
   Ok(HttpResponse::Ok().json(user))
}

#[post("/users")]
async fn create(user: web::Json<UserMessage>) -> Result<HttpResponse, ApiError> {
   let user = User::create(user.into_inner())?;
   Ok(HttpResponse::Ok().json(user))
}

#[put("/users/{id}")]
async fn update(
   id: web::Path<Uuid>,
   user: web::Json<UserMessage>,
) -> Result<HttpResponse, ApiError> {
   let user = User::update(id.into_inner(), user.into_inner())?;
   Ok(HttpResponse::Ok().json(user))
}

#[delete("/users/{id}")]
async fn delete(id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
   let num_deleted = User::delete(id.into_inner())?;
   Ok(HttpResponse::Ok().json(json!({ "deleted": num_deleted })))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
   cfg.service(find_all)
      .service(find)
      .service(create)
      .service(update)
      .service(delete);
}
