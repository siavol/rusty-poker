use actix_web::{Responder, HttpResponse, http::header::ContentType, body::BoxBody, web};
use serde::{Serialize, Deserialize};

use crate::utils::generate_uid;

#[derive(Deserialize)]
pub struct NewSessionParams {
    title: String
}

#[derive(Serialize, Deserialize)]
struct Session {
    title: String,
    id: String
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

pub async fn create_session(params: web::Json<NewSessionParams>) -> impl Responder {
    Session {
        title: params.title.clone(),
        id: generate_uid()
    }
}

#[cfg(test)]
mod tests {
    use actix_web::{test, App};
    use serde_json::json;

    use crate::routes::{api::{Session}, app_http_config};

    #[actix_web::test]
    async fn test_post_api_sesson_ok() {
        let srv = test::init_service(
            App::new()
                .configure(app_http_config)
        )
        .await;

        let res = test::TestRequest::post().uri("/api/session")
            .set_json(json!({
                "title": "My new session"
            }))
            .send_request(&srv)
            .await;
        assert!(res.status().is_success());
        
        let session: Session = test::read_body_json(res).await;
        assert_eq!(session.title, "My new session");
        assert!(session.id.len() > 0);
    }
}
