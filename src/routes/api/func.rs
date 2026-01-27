    use axum::{
        response::{
            IntoResponse,
            Json
        },
    };
    use tower_sessions::Session;
    use serde::Deserialize;
    use serde_json::json;
    use crate::functions::{shutdown, restart, upgrade};


    #[derive(Deserialize)]
    pub struct Func {
        func: String
    }

    pub async fn func(session: Session, Json(payload): Json<Func>) -> impl IntoResponse {
        if session.get::<bool>("connected").await.unwrap() == Some(true) {
            let res = match payload.func.as_str() {
                "shutdown" => {
                    shutdown();
                    json!({"status": "success"})
                },
                "restart" => {
                    restart();
                    json!({"status": "success"})
                },
                "upgrade" => {
                    upgrade();
                    json!({"status": "success"})
                },
                _ => {json!({"status": "error"})}
            };
            return Json(res).into_response();
        }
        Json(json!({"status": "Connection required"})).into_response()
    }