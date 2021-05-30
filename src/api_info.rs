use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::HttpRequest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ApiDescription {
    pub apiName: String,
    pub version: String
}

pub async fn index(req: HttpRequest)->impl Responder{
    HttpResponse::Ok().json(ApiDescription{apiName:"Api Rust Rest".to_string(), version:"1.0".to_string()})
}
