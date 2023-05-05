use std::env;
use actix_web::{App, HttpServer};
use actix_web::middleware;
use actix_web::middleware::Logger;
use dotenv::dotenv;
use env_logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "full");

    env_logger::init_from_env(env_logger::Env::new());

    let address = env::var("ADDRESS")
        .expect("ADDRESS must be defined");

    let port = env::var("PORT")
        .map(|port| port.parse::<u16>())
        .unwrap()
        .expect("PORT must be defined");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(middleware::Compress::default())
    })
        .bind((address, port))?
        .run()
        .await
}
