use axum::Router;
use axum::body::{Body, to_bytes};
use axum::http::Request;
use axum::http::StatusCode;
use serde_json::json;
use sqlx::sqlite::SqlitePoolOptions;
use tower::ServiceExt;

use crate::{User, create_app, init_db};

async fn test_app() -> Router {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();

    init_db(&pool).await;
    create_app(pool)
}

#[tokio::test]
async fn post_user_returns_created() {
    let app = test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/user")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "name": "Alice",
                        "age": 30,
                        "email": "alice@example.com"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn get_user_returns_user_data() {
    let app = test_app().await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/user")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "name": "Bob",
                        "age": 41,
                        "email": "bob@example.com"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/user/1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    let user: User = serde_json::from_slice(&body).unwrap();

    assert_eq!(user.id, 1);
    assert_eq!(user.name, "Bob");
    assert_eq!(user.age, 41);
    assert_eq!(user.email, "bob@example.com");
}

#[tokio::test]
async fn get_user_returns_not_found_for_missing_id() {
    let app = test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/user/999")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
