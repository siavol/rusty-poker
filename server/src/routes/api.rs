use actix_web::{Responder, HttpResponse, http::header::ContentType};

pub async fn create_session() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(r#"{ "name": "test", "id": "qwe123" }"#)
}

#[cfg(test)]
mod tests {
    use actix_web::{test, Responder, http, body::to_bytes};

    use super::create_session;

    #[actix_web::test]
    async fn test_post_session_ok() {
        let req = test::TestRequest::default()
            .to_http_request();
        let res = create_session().await.respond_to(&req);
        assert_eq!(res.status(), http::StatusCode::OK);
        
        let body = match to_bytes(res.into_body()).await {
            Ok(body) => body,
            Err(_) => panic!("Can not get response body.")
        };
        assert_eq!(std::str::from_utf8(&body).unwrap(), r#"{ "name": "test", "id": "qwe123" }"#);
    }
}