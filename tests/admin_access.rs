
use axum::http::StatusCode;
use tower::util::ServiceExt; // for `oneshot` method
use axum::body::Body;
use axum::http::Request;


fn get_token_for_role(role: &str) -> String {
    // Use the same secret and Claims struct as the app
    use handball_team_app::auth::{create_token};
    create_token(1, "admin@example.com", role, "Admin").unwrap()
}

#[tokio::test]
async fn test_admin_access_protected_route() {
    let app = handball_team_app::build_app_for_test().await;
    let token = get_token_for_role("admin");
    let response = app
        .oneshot(Request::builder()
            .uri("/api/admin/protected")
            .header("Authorization", format!("Bearer {}", token))
            .body(Body::empty())
            .unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_player_cannot_access_admin_route() {
    let app = handball_team_app::build_app_for_test().await;
    let token = get_token_for_role("player");
    let response = app
        .oneshot(Request::builder()
            .uri("/api/admin/protected")
            .header("Authorization", format!("Bearer {}", token))
            .body(Body::empty())
            .unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}
