use axum::{
    response::{
        IntoResponse,
        Html
    }
};
use askama::{
    Template
};


#[derive(Default, Template)]
#[template(path = "routes/index.html")]
struct Index;

pub async fn index() -> impl IntoResponse {
    Html(Index{}.render().unwrap())
}