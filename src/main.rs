use axum::{
    routing::{get, post},
    Router,
    Extension
};
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
use tower_http::services::ServeDir;
use time::Duration;
use sqlx::sqlite::{SqlitePoolOptions};
use std::sync::Arc;

mod functions;
mod database;
mod routes {
    pub mod index;
    pub mod contact;
    pub mod planning;
    pub mod panel;
    pub mod api {
        pub mod login;
        pub mod logout;
        pub mod func;
        pub mod events;
        pub mod users;
    }
}

#[tokio::main]
async fn main() {
    println!("{:?}", std::env::current_dir());
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:db.db")
        .await
        .unwrap();

    let _ = sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            name TEXT PRIMARY KEY,
            password TEXT NOT NULL,
            grade TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS events (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            date_debut TEXT NOT NULL,
            date_fin TEXT NOT NULL,
            description TEXT NOT NULL
        );",
    )
    .execute(&pool)
    .await;
    
    if database::lister_users(&pool).await.unwrap().is_empty() {
        let default_user: serde_json::Value = serde_json::from_str(&std::fs::read_to_string("default_user.json").unwrap()).unwrap();

        let _ = database::ajouter_user(&pool, database::User{
            name: default_user["name"].as_str().unwrap().to_string(),
            password: default_user["password"].as_str().unwrap().to_string(),
            grade: "ADMIN".to_string()
        }).await;
    }

    let pool = Arc::new(pool);

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(true)
        .with_expiry(Expiry::OnInactivity(Duration::minutes(5)));

    let api = Router::new()
        .route("/login", post(routes::api::login::login))
        .route("/logout", post(routes::api::logout::logout))
        .route("/func", post(routes::api::func::func))
        .route("/event", post(routes::api::events::event))
        .route("/user/add", post(routes::api::users::api_ajouter_user))
        .route("/user/remove", post(routes::api::users::api_supprimer_user))
        .route("/event/remove", post(routes::api::events::supprimer_event));

    let app = Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .nest("/api", api)
        .route("/", get(routes::index::index))
        .route("/contact", get(routes::contact::contact))
        .route("/planning", get(routes::planning::planning))
        .route("/panel", get(routes::panel::panel))
        .layer(session_layer)
        .layer(Extension(pool));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on {:?}", &listener);
    axum::serve(listener, app).await.unwrap();
}