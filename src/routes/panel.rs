use axum::{
    response::{
        IntoResponse,
        Html
    },
    Extension
};
use askama::{
    Template
};
use tower_sessions::Session;
use crate::database::{User, lister_users, lister_events};
use std::sync::Arc;
use sqlx::SqlitePool;


#[derive(Template)]
#[template(path = "routes/panel.html")]
struct Panel{ 
    connected: bool,
    grade: String,
    list_user: Vec<User>,
    list_event: Vec<(String, String, String)>
}

#[derive(Default, Template)]
#[template(path = "routes/login.html")]
struct Login{
    connected: bool
}

pub async fn panel(session: Session, Extension(pool): Extension<Arc<SqlitePool>>) -> impl IntoResponse {
    if session.get::<bool>("connected").await.unwrap() == Some(true) {
        Html(Panel{
            connected: session.get::<bool>("connected").await.unwrap_or(None) == Some(true),
            grade: session.get::<String>("grade").await.unwrap_or(None).unwrap(),
            list_user: lister_users(&pool).await.unwrap(),
            list_event: lister_events(&pool).await.unwrap()
        }.render().unwrap())
    }else {
        Html(Login{connected: session.get::<bool>("connected").await.unwrap_or(None) == Some(true)}.render().unwrap())
    }
}