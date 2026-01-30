use sqlx::sqlite::SqlitePool;
use sqlx::Row;
use anyhow::Result;

#[derive(Debug)]
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

pub fn supprimer_user(){}

pub async fn lister_users(pool: &SqlitePool) -> Result<Vec<User>> {
    let rows = sqlx::query("SELECT name, password, grade FROM users")
        .fetch_all(pool)
        .await?;

    let users = rows.into_iter().map(|r| User {
        name: r.get("name"),
        password: r.get("password"),
        grade: r.get("grade")
    }).collect();
    println!("data base :\n{:#?}", users);

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

    println!("{:?}", row.clone().map(|(v,)| v));
    
    Ok(row.map(|(v,)| v))
}