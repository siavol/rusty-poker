mod routes;
mod utils;
mod schema;

use std::net::IpAddr;
use actix_web::{HttpServer, App, Responder, get, HttpResponse, middleware::Logger};
use log;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello from Rusty Pocker Server!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(
        env_logger::Env::new().default_filter_or("info"));

    let ip_addr: IpAddr = std::env::var("IP_ADDR")
        .unwrap_or("127.0.0.1".to_string())
        .parse()
        .expect("Failed to parse IP_ADDR");
    let port: u16 = std::env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse()
        .expect("Failed to parse PORT");

    let server = HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(hello)
            .configure(routes::app_http_config)
    })
    .bind((ip_addr, port));

    match server {
        Ok(server) => {
            log::info!("Server started on {:?}:{port}", ip_addr);
            server
                .run()
                .await
        },
        Err(err) => {
            log::error!("Can not start server: {:?}", err);
            Err(err)
        }
    }
}
