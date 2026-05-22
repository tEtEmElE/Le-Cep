use axum::{
    extract::Form,
    response::{
        IntoResponse,
        Redirect
    },
    Extension
};
use tower_sessions::Session;
use crate::database::{User, ajouter_user, supprimer_user};
use std::sync::Arc;
use sqlx::sqlite::SqlitePool;
use serde::Deserialize;

pub async fn api_ajouter_user(session: Session, Extension(pool): Extension<Arc<SqlitePool>>, Form(payload): Form<User>) -> impl IntoResponse {
    if session.get::<bool>("connected").await.unwrap() == Some(true) && session.get::<String>("grade").await.unwrap() == Some("ADMIN".to_string()) {
        ajouter_user(&pool, payload).await.unwrap();
    }
    Redirect::to("/")
}

#[derive(Debug, Deserialize)]
pub struct DeleteUser {
    pub name: String
}

pub async fn api_supprimer_user(session: Session, Extension(pool): Extension<Arc<SqlitePool>>, Form(payload): Form<DeleteUser>) -> impl IntoResponse {
    if session.get::<bool>("connected").await.unwrap() == Some(true) && session.get::<String>("grade").await.unwrap() == Some("ADMIN".to_string()) {
        let _ = supprimer_user(&pool, payload.name).await.unwrap();
    }
    Redirect::to("/")
}