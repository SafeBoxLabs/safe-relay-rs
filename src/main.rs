use std::env;
use actix_web::{App, HttpServer};
use actix_web::middleware;
use actix_web::middleware::Logger;
use dotenv::dotenv;
use env_logger;

use crate::safe::{SafeInfo, SafeResponse};
use crate::safe_config::SafeConfig;
use crate::safe_handlers::*;
use crate::safe_service::SafeService;
use crate::safe_use_case::SafeUseCase;

pub(crate) mod safe_service;
pub(crate) mod safe_handlers;
pub(crate) mod safe;
pub(crate) mod safe_use_case;
pub(crate) mod safe_config;
pub(crate) mod ethers_ext;

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
