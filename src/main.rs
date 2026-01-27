use axum::{
    routing::{get, post},
    Router
};
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
use tower_http::services::ServeDir;
use time::Duration;

mod functions;
mod routes {
    pub mod index;
    pub mod contact;
    pub mod planning;
    pub mod panel;
    pub mod api {
        pub mod login;
        pub mod logout;
        pub mod func;
    }
}

#[tokio::main]
async fn main() {
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(true)
        .with_expiry(Expiry::OnInactivity(Duration::minutes(5)));

    let api = Router::new()
        .route("/login", post(routes::api::login::login))
        .route("/logout", post(routes::api::logout::logout))
        .route("/func", post(routes::api::func::func));

    let app = Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .nest("/api", api)
        .route("/", get(routes::index::index))
        .route("/contact", get(routes::contact::contact))
        .route("/planning", get(routes::planning::planning))
        .route("/panel", get(routes::panel::panel))
        .layer(session_layer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on {:?}", &listener);
    axum::serve(listener, app).await.unwrap();
}