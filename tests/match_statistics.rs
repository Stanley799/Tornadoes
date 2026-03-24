//! Integration tests for match statistics service and endpoint.

use axum::http::{Request, StatusCode};
use axum::body::Body;
use handball_team_app::auth::create_token;
use handball_team_app::build_app_for_test;
use tower::util::ServiceExt;

#[tokio::test]
async fn test_statistics_admin_access() {
    let app = build_app_for_test().await;
    let token = create_token(1, "admin@example.com", "admin", "Admin").unwrap();
    let req = Request::builder()
        .uri("/api/matches/1/statistics")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert!(resp.status() == StatusCode::OK || resp.status() == StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_statistics_coach_access() {
    let app = build_app_for_test().await;
    let token = create_token(2, "coach@example.com", "coach", "Coach").unwrap();
    let req = Request::builder()
        .uri("/api/matches/1/statistics")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert!(resp.status() == StatusCode::OK || resp.status() == StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_statistics_player_forbidden() {
    let app = build_app_for_test().await;
    let token = create_token(3, "player@example.com", "player", "Player").unwrap();
    let req = Request::builder()
        .uri("/api/matches/1/statistics")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_statistics_not_found() {
    let app = build_app_for_test().await;
    let token = create_token(1, "admin@example.com", "admin", "Admin").unwrap();
    let req = Request::builder()
        .uri("/api/matches/99999/statistics")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
