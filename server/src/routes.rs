pub mod api;

use actix_web::{web};

pub fn app_http_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::resource("/session").route(web::post().to(api::create_session)))
    );
}
