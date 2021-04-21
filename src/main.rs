#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;

mod api_error;
mod db;
mod schema;
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
