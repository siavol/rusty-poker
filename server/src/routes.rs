pub mod api;

use actix_web::{web, Responder, body::BoxBody, HttpResponse, http::header::ContentType};
use serde_json::json;

use crate::schema::Session;

pub fn app_http_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::resource("/session").route(web::post().to(api::create_session)))
            .service(web::resource("/session/{id}").route(web::get().to(api::get_session)))
    );
}

// HTTP Error responses

pub enum ApiResponse {
    Ok(Session),
    NotFound,
}

impl Responder for ApiResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        match &self {
            ApiResponse::Ok(value) => {
                let body = serde_json::to_string(value).unwrap();
                HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .body(body)        
            },
            ApiResponse::NotFound => HttpResponse::NotFound()
                .content_type(ContentType::json())
                .body(json!({
                    "errorCode": "Not Found"
                }).to_string())
        }
    }
}