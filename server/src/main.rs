mod routes;
mod utils;
mod storage;

use std::net::IpAddr;
use std::sync::Mutex;
use actix_files;
use actix_web::{HttpServer, App, middleware::Logger, web};
use log;

// switch to generic app state with Storage
pub struct AppState {
    storage: Mutex<storage::memory::MemoryStorage>,
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
        .unwrap_or("3000".to_string())
        .parse()
        .expect("Failed to parse PORT");

    let data = web::Data::new(AppState {
        storage: Mutex::new(storage::memory::MemoryStorage::new())
    });
    let server = HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(Logger::default())
            .configure(routes::app_http_config)
            .service(actix_files::Files::new("/", "./static").index_file("index.html"))
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
