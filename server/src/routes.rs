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

// Define schema structures traits for HTTP communication support

impl Responder for Session {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

// HTTP Error responses

pub enum HttpError {
    NotFound
}

impl Responder for HttpError {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        match &self {
            HttpError::NotFound => HttpResponse::NotFound()
                .content_type(ContentType::json())
                .body(json!({
                    "errorCode": "Not Found"
                }).to_string())
        }
    }
}