use actix_web::{web::{ServiceConfig, scope}, post, Responder, HttpResponse, http::header::ContentType};

#[post("/session")]
async fn create_session() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(r#"{ "name": "test", "id": "qwe123" }"#)
}

pub fn config_service(cfg: &mut ServiceConfig) {
    cfg.service(scope("/api")
        .service(create_session)
    );
}
