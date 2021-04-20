#[macro_use]
extern crate log;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;

mod user;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    info!("Starting server");

    let host = env::var("HOST").expect("Host not set");
    let port = env::var("PORT").expect("Port not set");

    HttpServer::new(|| App::new().configure(user::init_routes))
        .bind(format!("{}:{}", host, port))?
        .run()
        .await
}
