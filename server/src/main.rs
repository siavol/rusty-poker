use actix_web::{HttpServer, App, Responder, get, HttpResponse};
use log;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init();

    const ADDRS: &str = "127.0.0.1";
    const PORT: u16 = 8080;
    let server = HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind((ADDRS, PORT));
    match server {
        Ok(server) => {
            log::info!("Server started on {ADDRS}:{PORT}");
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
