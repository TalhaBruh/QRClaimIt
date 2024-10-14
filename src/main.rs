#![allow(dead_code)]

use std::sync::Arc;

mod error;
mod modules;
mod utils;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use modules::qr;
use utils::db::PostgresRepository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info,debug");
    env_logger::init();
    let qr_repository = Arc::new(PostgresRepository::new().await);
    let service = Arc::new(qr::Service::new(qr_repository));

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .configure(qr::api::route_config)
            .app_data(web::Data::new(service.clone()))
    })
    .bind("0.0.0.0:80")?
    .run()
    .await
}
