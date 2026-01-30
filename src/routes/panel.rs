use axum::{
    response::{
        IntoResponse,
        Html
    },
};
use askama::{
    Template
};
use tower_sessions::Session;


#[derive(Template)]
#[template(path = "routes/panel.html")]
struct Panel{ 
    connected: bool,
    grade: String
}

#[derive(Default, Template)]
#[template(path = "routes/login.html")]
struct Login{
    connected: bool
}

pub async fn panel(session: Session) -> impl IntoResponse {
    if session.get::<bool>("connected").await.unwrap() == Some(true) {
        Html(Panel{
            connected: session.get::<bool>("connected").await.unwrap_or(None) == Some(true),
            grade: session.get::<String>("grade").await.unwrap_or(None).unwrap()
        }.render().unwrap())
    }else {
        Html(Login{connected: session.get::<bool>("connected").await.unwrap_or(None) == Some(true)}.render().unwrap())
    }
}