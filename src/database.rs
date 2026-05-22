use std::fmt::Debug;

use sqlx::sqlite::SqlitePool;
use sqlx::Row;
use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct User{
    pub name : String,
    pub password : String,
    pub grade : String
}

pub async fn ajouter_user(pool: &SqlitePool, user: User) -> Result<()>{
    let _ = sqlx::query(
        "INSERT INTO users ( name, password, grade) VALUES (?, ?, ?)"
    )
        .bind(&user.name)
        .bind(&user.password)
        .bind(&user.grade)
        .execute(pool)
        .await?;
    
    Ok(())
}

pub async fn supprimer_user(pool: &SqlitePool, name: String) -> Result<()>{
    let nb_admin: Vec<sqlx::sqlite::SqliteRow> = sqlx::query(
        "SELECT COUNT(*) AS nb FROM users WHERE grade = 'ADMIN'"
    )
        .fetch_all(pool)
        .await?;

    let nom_admin = sqlx::query(
        "SELECT name FROM users WHERE grade = 'ADMIN'"
    )
        .fetch_all(pool)
        .await?
        .iter()
        .map(
            |r| 
            r.get::<String, _>("name")
        )
            .collect::<Vec<String>>();


    if !(nb_admin[0].get::<i64, _>("nb") > 1) && nom_admin.contains(&name) {
        return Ok(());
    }
    let _ = sqlx::query(
        "DELETE FROM users WHERE name = ?"
    )
        .bind(name)
        .execute(pool)
        .await?;

    Ok(())

}

pub async fn lister_users(pool: &SqlitePool) -> Result<Vec<User>> {
    let rows = sqlx::query("SELECT name, password, grade FROM users")
        .fetch_all(pool)
        .await?;

    let users = rows.into_iter().map(|r| User {
        name: r.get("name"),
        password: r.get("password"),
        grade: r.get("grade")
    }).collect();

    Ok(users)
}

pub async fn exist(pool: &SqlitePool, name: &String) -> Result<bool> {
    let row: Option<(String,)> = sqlx::query_as("SELECT name FROM users WHERE name = ?")
        .bind(name)
        .fetch_optional(pool)
        .await?;

    Ok(row.is_some())
}

pub async fn password_match(pool: &SqlitePool, name: &String, password: &String) -> Result<bool> {
    let row: Option<(String,)> = sqlx::query_as("SELECT password FROM users WHERE name = ?")
        .bind(name)
        .fetch_optional(pool)
        .await?;

    match row {
        Some((password_from_db,)) => Ok(&password_from_db == password),
        None => Ok(false), 
    }
}

pub async fn get_info(
    pool: &SqlitePool,
    name: &str,
    key: &str,
) -> Result<Option<String>> {

    let query = match key {
        "grade" => "SELECT grade FROM users WHERE name = ?",
        "password" => "SELECT password FROM users WHERE name = ?",
        _ => return Ok(None),
    };
    
    let row: Option<(String,)> = sqlx::query_as(query)
        .bind(name)
        .fetch_optional(pool)
        .await?;    
    Ok(row.map(|(v,)| v))
}

pub async fn ajouter_event(pool: &SqlitePool, title: &String, date_debut: &String, date_fin: &String, description: &String) -> Result<()> {
    let _ = sqlx::query(
        "INSERT INTO events (title, date_debut, date_fin, description) VALUES (?, ?, ?, ?)"
    )
        .bind(title)
        .bind(format!("{}:00+00:00", date_debut))
        .bind(format!("{}:00+00:00", date_fin))
        .bind(description)
        .execute(pool)
        .await?;
    
    Ok(())
}

pub async fn clean_events(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "DELETE FROM events WHERE datetime(date_fin) < datetime('now')"
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn lister_events(pool: &SqlitePool) -> Result<Vec<(String, String, String, String)>> {
    let _ = clean_events(pool).await?;
    let rows = sqlx::query("SELECT title, date_debut, date_fin, description FROM events")
        .fetch_all(pool)
        .await?;

    let events: Vec<(String, String, String, String)> = rows.into_iter().map(|r| (
        r.get("title"),
        r.get("date_debut"),
        r.get("date_fin"),
        r.get("description")
    )).collect();

    Ok(events)
}