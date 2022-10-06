use actix_web::{Responder, HttpResponse, http::header::ContentType, body::BoxBody};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Session {
    name: &'static str,
    id: &'static str
}

impl Responder for Session {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

pub async fn create_session() -> impl Responder {
    Session {
        name: "test session",
        id: "qwe123"
    }
}

#[cfg(test)]
mod tests {
    use actix_web::{test, Responder, http, body::to_bytes};

    use super::create_session;

    #[actix_web::test]
    async fn test_create_session_ok() {
        let req = test::TestRequest::default()
            .to_http_request();
        let res = create_session().await.respond_to(&req);
        assert_eq!(res.status(), http::StatusCode::OK);
        
        let body = match to_bytes(res.into_body()).await {
            Ok(body) => body,
            Err(_) => panic!("Can not get response body.")
        };
        let body = std::str::from_utf8(&body).unwrap();
        assert_eq!(body, r#"{"name":"test session","id":"qwe123"}"#);

        // let session: Session = serde_json::from_str(&body).expect("Failed to parse body as Session object");
        // assert_eq!(session.name, "test session");

    }
}