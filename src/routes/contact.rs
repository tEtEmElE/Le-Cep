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
#[template(path = "routes/contact.html")]
struct Contact{
    connected: bool
}

pub async fn contact(session: Session) -> impl IntoResponse {
    Html(Contact{connected: session.get::<bool>("connected").await.unwrap_or(None) == Some(true)}.render().unwrap())
}