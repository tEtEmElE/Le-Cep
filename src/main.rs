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
use tower_http::services::ServeDir;

mod routes {
    pub mod index;
    pub mod contact;
    pub mod planning;
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .route("/", get(routes::index::index))
        .route("/contact", get(routes::contact::contact))
        .route("/planning", get(routes::planning::planning));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on {:?}", &listener);
    axum::serve(listener, app).await.unwrap();
    
}