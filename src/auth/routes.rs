use crate::api_error::ApiError;
use crate::email::{Contact, Email};
use crate::email_verification_token::{EmailVerificationToken, EmailVerificationTokenMessage};
use crate::user::{User, UserMessage};
use actix_session::Session;
use actix_web::{get, post, web, HttpResponse};
use chrono::Utc;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

#[derive(Deserialize)]
struct RegistrationMessage {
   token: String,
   email: String,
   password: String,
}

#[post("/sign-in")]
async fn sign_in(
   credentials: web::Json<UserMessage>,
   session: Session,
) -> Result<HttpResponse, ApiError> {
   let credentials = credentials.into_inner();

   let user = User::find_by_email(credentials.email).map_err(|e| match e.status_code {
      404 => ApiError::new(401, "Credentials not valid".to_string()),
      _ => e,
   })?;

   let is_valid = user.verify_password(credentials.password.as_bytes())?;

   if is_valid == true {
      session.set("user_id", user.id)?;
      session.renew();

      Ok(HttpResponse::Ok().json(user))
   } else {
      Err(ApiError::new(401, "Credentials not valid!".to_string()))
   }
}

#[post("/sign-out")]
async fn sign_out(session: Session) -> Result<HttpResponse, ApiError> {
   let id: Option<Uuid> = session.get("user_id")?;

   if let Some(_) = id {
      session.purge();
      Ok(HttpResponse::Ok().json(json!({"message":"Successfully signed out"})))
   } else {
      Err(ApiError::new(401, "Unauthorized".to_string()))
   }
}

#[get("/who-am-i")]
async fn who_am_i(session: Session) -> Result<HttpResponse, ApiError> {
   let id: Option<Uuid> = session.get("user_id")?;

   if let Some(id) = id {
      let user = User::find(id)?;
      Ok(HttpResponse::Ok().json(user))
   } else {
      Err(ApiError::new(401, "Unauthorized".to_string()))
   }
}

#[post("/invite")]
async fn invite(body: web::Json<EmailVerificationTokenMessage>) -> Result<HttpResponse, ApiError> {
   let body = body.into_inner();
   let token = EmailVerificationToken::create(body.clone())?;
   let token_string = hex::encode(token.id);

   Email::new(Contact::new("juciano@outlook.com.br", "Jucian0"))
      .add_recipient(body.email)
      .set_subject("Confirm your email")
      .set_html(format!("Your confirmation code is: {}", &token_string))
      .send()?;

   Ok(HttpResponse::Ok().json(json!({"message": "Verification email sent"})))
}

#[post("/register")]
async fn register(body: web::Json<RegistrationMessage>) -> Result<HttpResponse, ApiError> {
   let body = body.into_inner();
   let token_id =
      hex::decode(body.token).map_err(|e| ApiError::new(403, "Invalid token".to_string()))?;

   let token = EmailVerificationToken::find(&token_id).map_err(|e| match e.status_code {
      404 => ApiError::new(403, "Invalid token".to_string()),
      _ => e,
   })?;

   if token.email != body.email {
      return Err(ApiError::new(403, "Invalid token".to_string()));
   }

   if token.expires_at < Utc::now().naive_utc() {
      return Err(ApiError::new(403, "Token expired".to_string()));
   }

   let user = User::create(UserMessage {
      email: body.email,
      password: body.password,
   })?;

   Ok(HttpResponse::Ok().json(json!({"message": "Successfully registered", "user": user})))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
   cfg.service(sign_in)
      .service(sign_out)
      .service(register)
      .service(who_am_i)
      .service(invite);
}
