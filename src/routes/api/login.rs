use axum::{
    extract::Form,
    response::{
        IntoResponse,
        Redirect
    },
};
use tower_sessions::Session;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Connection {
    username: String,
    password: String
}

pub async fn login(session: Session, Form(payload): Form<Connection>) -> impl IntoResponse {
    if payload.username == "admin" && payload.password == "password"{
        session.insert("connected", true).await.unwrap();
        return Redirect::to("/panel");
    }
    Redirect::to("/panel")
}