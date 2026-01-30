use axum::{
    extract::Form,
    response::{
        IntoResponse,
        Redirect
    },
    Extension
};
use tower_sessions::Session;
use serde::Deserialize;
use sqlx::sqlite::SqlitePool;
use std::sync::Arc;
use crate::database::{exist, password_match, get_info};

#[derive(Deserialize)]
pub struct Connection {
    username: String,
    password: String
}

pub async fn login(session: Session, Extension(pool): Extension<Arc<SqlitePool>>, Form(payload): Form<Connection>) -> impl IntoResponse {
    if exist(&pool, &payload.username).await.unwrap() && password_match(&pool, &payload.username, &payload.password).await.unwrap(){
        session.insert("connected", true).await.unwrap();
        session.insert("grade", 
            get_info(
                &pool, 
                payload.username.as_str(), 
                "grade")
                    .await
                    .unwrap()
                    .unwrap()
            ).await.unwrap();
        return Redirect::to("/panel");
    }
    Redirect::to("/panel")
}