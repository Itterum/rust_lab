use std::env;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    age: i32,
    email: String,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
struct User {
    id: i64,
    name: String,
    age: i32,
    email: String,
}

async fn get_user(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<User>, (StatusCode, String)> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM user WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, e.to_string()))?;

    Ok(Json(user))
}

async fn post_user(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateUser>,
) -> Result<StatusCode, String> {
    sqlx::query("INSERT INTO user (name, age, email) VALUES (?, ?, ?)")
        .bind(payload.name)
        .bind(payload.age)
        .bind(payload.email)
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(StatusCode::CREATED)
}

async fn init_db(pool: &SqlitePool) {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS user (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            age INTEGER,
            email TEXT
        )",
    )
    .execute(pool)
    .await
    .unwrap();
}

fn create_app(pool: SqlitePool) -> Router {
    Router::new()
        .route("/user/{id}", get(get_user))
        .route("/user", post(post_user))
        .with_state(pool)
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect(
        "DATABASE_URL is not set. Copy apps/server_app/.env.example to .env and configure it.",
    );
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    let bind_addr = format!("{}:{}", host, port);

    let pool = SqlitePool::connect(&database_url).await.unwrap();

    init_db(&pool).await;
    let app = create_app(pool);

    let listener = tokio::net::TcpListener::bind(&bind_addr).await.unwrap();
    println!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests;
