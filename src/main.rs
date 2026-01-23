use axum::{
    routing::get,
    Router,
    response::{
        IntoResponse,
        Html
    }
};
use askama::{
    Template
};
mod routes {
    pub mod index;
    pub mod contact;
}
use tower_http::services::ServeDir;


#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .route("/", get(routes::index::index))
        .route("/contact", get(routes::contact::contact));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}