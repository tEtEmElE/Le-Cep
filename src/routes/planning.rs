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
#[template(path = "routes/planning.html")]
struct Planning;

pub async fn planning() -> impl IntoResponse {
    Html(Planning{}.render().unwrap())
}