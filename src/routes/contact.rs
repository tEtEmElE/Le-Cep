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
#[template(path = "routes/contact.html")]
struct Contact;

pub async fn contact() -> impl IntoResponse {
    Html(Contact{}.render().unwrap())
}