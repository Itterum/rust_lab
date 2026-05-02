use axum::{
    Json, Router,
    extract::{Path, State},
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

#[derive(Serialize, sqlx::FromRow)]
struct User {
    id: i64,
    name: String,
    age: i32,
    email: String,
}

async fn get_user(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<User>, (axum::http::StatusCode, String)> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM user WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|e| (axum::http::StatusCode::NOT_FOUND, e.to_string()))?;

    Ok(Json(user))
}

async fn post_user(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateUser>,
) -> Result<axum::http::StatusCode, String> {
    sqlx::query("INSERT INTO user (name, age, email) VALUES (?, ?, ?)")
        .bind(payload.name)
        .bind(payload.age)
        .bind(payload.email)
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(axum::http::StatusCode::CREATED)
}

#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect(
        "sqlite:C:\\Users\\ivanl\\DataGripProjects\\database_app\\identifier.sqlite?mode=rwc",
    )
    .await
    .unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS user (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            age INTEGER,
            email TEXT
        )",
    )
    .execute(&pool)
    .await
    .unwrap();

    let app = Router::new()
        .route("/user/{id}", get(get_user))
        .route("/user", post(post_user))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
