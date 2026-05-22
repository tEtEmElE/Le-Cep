use axum::{
    extract::Form,
    response::{
        IntoResponse,
        Redirect
    },
    Extension
};
use tower_sessions::Session;
use crate::routes::planning::SpecialEvent;
use crate::database::ajouter_event;
use sqlx::sqlite::SqlitePool;
use std::sync::Arc;
use serde::Deserialize;


pub async fn event(session: Session, Extension(pool): Extension<Arc<SqlitePool>>, Form(event): Form<SpecialEvent>) -> impl IntoResponse {
    if session.get::<bool>("connected").await.unwrap() == Some(true) {
        ajouter_event(
            &pool,
            &event.title.to_string(),
            &&event.date_debut.to_string(),
            &&event.date_fin.to_string(),
            &event.description.to_string()
        ).await.unwrap();
        return Redirect::to("/planning");
    }
    Redirect::to("/")
}

#[derive(Debug, Deserialize)]
pub struct DeleteEvent {
    pub title: String
}

pub async fn supprimer_event(session: Session, Extension(pool): Extension<Arc<SqlitePool>>, Form(event): Form<DeleteEvent>) -> impl IntoResponse {
    if session.get::<bool>("connected").await.unwrap() == Some(true) {
        let _ = sqlx::query("DELETE FROM events WHERE title = ?")
            .bind(event.title)
            .execute(&*pool)
            .await;
        return Redirect::to("/planning");
    }
    Redirect::to("/")
}