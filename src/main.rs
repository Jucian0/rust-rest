#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;


use crate::api_info::index;
use actix_redis::RedisSession;
use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use std::env;

mod api_error;
mod auth;
mod db;
mod email;
mod email_verification_token;
mod schema;
mod user;
mod api_info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    info!("Starting server");

    let host = env::var("HOST").expect("Host not set");
    let port = env::var("PORT").expect("Port not set");

    let redis_port = env::var("REDIS_PORT").expect("Redis port not set");
    let redis_host = env::var("REDIS_HOST").expect("Redis host not set");

    HttpServer::new(move || {
        App::new()
            .wrap(RedisSession::new(
                format!("{}:{}", redis_host, redis_port),
                &[0; 32],
            ))
            .route("/", web::get().to(index))
            .configure(user::init_routes)
            .configure(auth::init_routes)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
