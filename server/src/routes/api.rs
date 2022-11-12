use actix_web::{Responder, web};

use crate::AppState;
use crate::utils::generate_uid;
use rusty_poker_common::{Session, NewSessionParams};
use crate::routes::ApiResponse;
use crate::storage;
use crate::storage::Storage;

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

pub async fn create_session(params: web::Json<NewSessionParams>, data: web::Data<AppState>) -> impl Responder {
    let session = Session {
        title: params.title.clone(),
        id: generate_uid(),
        cards: get_cards()
    };
    let mut storage = data.storage.lock().unwrap();
    // TODO: check the error and return HTTP 5xx response
    storage.save(session.clone()).expect("Failed to save session");
    return ApiResponse::Ok(session);
}

pub async fn get_session(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    let storage = data.storage.lock().unwrap();
    match storage.find(&id) {
        Ok(session) => {
            let copy = session.clone();
            ApiResponse::Ok(copy)
        },
        Err(storage_err) => match storage_err {
            storage::Error::NotFound => ApiResponse::NotFound
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;
    use actix_web::{test, App, web};
    use serde_json::json;
    use crate::AppState;
    use crate::storage;
    use crate::routes::{api::{Session}, app_http_config};
    
    fn app_test_config(cfg: &mut web::ServiceConfig) {
        cfg
            .app_data(web::Data::new(AppState {
                storage: Mutex::new(storage::memory::MemoryStorage::new())
            }))
            .configure(app_http_config);
    }    

    #[actix_web::test]
    async fn test_post_api_sesson_ok() {
        let srv = test::init_service(App::new()
            .configure(app_test_config)
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
                .configure(app_test_config)
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

    #[actix_web::test]
    async fn test_get_api_session_return_not_found() {
        let srv = test::init_service(
            App::new()
                .configure(app_test_config)
        )
        .await;

        let res = test::TestRequest::get().uri("/api/session/not-existing")
            .send_request(&srv)
            .await;
        assert_eq!(res.status(), 404);
    }
}
