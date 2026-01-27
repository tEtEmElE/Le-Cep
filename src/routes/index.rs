use axum::{
    response::{
        IntoResponse,
        Html
    }
};
use askama::{
    Template
};
use tower_sessions::Session;


#[derive(Default, Template)]
#[template(path = "routes/index.html")]
struct Index{
    connected: bool
}

pub async fn index(session: Session) -> impl IntoResponse {
    Html(Index{connected: session.get::<bool>("connected").await.unwrap_or(None) == Some(true)}.render().unwrap())
}