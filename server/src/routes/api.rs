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
    id: String,
    cards: Vec<String>
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

fn get_cards() -> Vec<String> {
    vec![
        "0.5".to_string(),
        "1".to_string(),
        "2".to_string(),
        "3".to_string(),
        "5".to_string(),
        "8".to_string(),
        "13".to_string(),
        "21".to_string(),
        "?".to_string()
    ]
}

pub async fn create_session(params: web::Json<NewSessionParams>) -> impl Responder {
    Session {
        title: params.title.clone(),
        id: generate_uid(),
        cards: get_cards()
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
        assert!(session.cards.len() > 0);
    }

    #[actix_web::test]
    async fn test_post_api_session_return_unique_id() {
        let srv = test::init_service(
            App::new()
                .configure(app_http_config)
        )
        .await;

        let res1 = test::TestRequest::post().uri("/api/session")
            .set_json(json!({
                "title": "session 1"
            }))
            .send_request(&srv)
            .await;
        assert!(res1.status().is_success());
        let session1: Session = test::read_body_json(res1).await;

        let res2 = test::TestRequest::post().uri("/api/session")
            .set_json(json!({
                "title": "session 2"
            }))
            .send_request(&srv)
            .await;
        assert!(res2.status().is_success());
        let session2: Session = test::read_body_json(res2).await;

        assert!(session1.id.len() > 0);
        assert!(session2.id.len() > 0);
        assert_ne!(session1.id, session2.id);
    }
}
