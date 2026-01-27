use axum::{
    response::{
        IntoResponse,
        Redirect
    },
};
use tower_sessions::Session;

pub async fn logout(session: Session) -> impl IntoResponse {
    if session.get::<bool>("connected").await.unwrap() == Some(true) {
        session.remove::<bool>("connected").await.unwrap();
    }
    Redirect::to("/")
}