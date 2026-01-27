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
#[template(path = "routes/planning.html")]
struct Planning{
    connected: bool
}

pub async fn planning(session: Session) -> impl IntoResponse {
    Html(Planning{connected: session.get::<bool>("connected").await.unwrap_or(None) == Some(true)}.render().unwrap())
}