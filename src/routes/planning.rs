use axum::{
    response::{
        IntoResponse,
        Html
    },
    Extension
};
use askama::{
    Template
};
use tower_sessions::Session;
use crate::database::lister_events;
use std::sync::Arc;
use sqlx::sqlite::SqlitePool;
use serde::Deserialize;


#[derive(Default, Template)]
#[template(path = "routes/planning.html")]
struct Planning{
    connected: bool,
    weekly_meetings: Vec<WeeklyMeeting>,
    special_events: Vec<SpecialEvent>
}

#[derive(Debug, Deserialize)]
pub struct SpecialEvent {
    pub title: String,
    pub date: String,
    pub description: String
}

struct WeeklyMeeting {
    title: String,
    day: String,
    description: String,
    heure: String
}

pub async fn planning(session: Session, Extension(pool): Extension<Arc<SqlitePool>>) -> impl IntoResponse {
    Html(
        Planning{
            connected: session.get::<bool>("connected").await.unwrap_or(None) == Some(true), 
            weekly_meetings: vec![
                WeeklyMeeting {
                    title: "Culte".into(), 
                    day: "Dimanche".into(), 
                    description: "Réunion de l'église tout les dimanches".into(),
                    heure: "10h-12h".into()
                },
                WeeklyMeeting {
                    title: "Réunion de prière".into(), 
                    day: "Jeudi".into(), 
                    description: "Prière ensemble suivi d'une courte reflexion".into(),
                    heure: "19h-20h".into()
                }
            ],
            special_events: lister_events(&pool).await.unwrap_or(vec![]).into_iter().map(|(title, date, description)| 
                SpecialEvent {
                    title,
                    date,
                    description
                }
            ).collect()
        }.render().unwrap()
    )
}