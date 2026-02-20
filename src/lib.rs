pub mod auth;
pub mod db;
pub mod errors;
pub mod handlers;
pub mod models;

use axum::{middleware, routing::{get, post}, Router};
use sqlx::postgres::PgPoolOptions;
use tower_http::services::ServeDir;

pub async fn build_app_for_test() -> Router {
    use std::env;
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://Tornadoes:Seblent2568.@localhost:5432/handball_db".to_string());
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL");
    let public_api = Router::new()
        .route("/api/announcements", get(handlers::announcements::list_announcements))
        .route("/api/matches", get(handlers::matches::list_matches))
        .route("/api/seasons", get(handlers::seasons::list_seasons))
        .route("/api/tournaments", get(handlers::seasons::list_tournaments));
    let auth_routes = Router::new()
        .route("/api/register", post(auth::register_handler))
        .route("/api/login", post(auth::login_handler));
    let protected_api = Router::new()
        .route("/api/announcements", post(handlers::announcements::create_announcement))
        .route("/api/announcements/approve", post(handlers::announcements::approve_announcement))
        .route("/api/announcements/reject", post(handlers::announcements::reject_announcement))
        .route("/api/announcements/pending", get(handlers::announcements::list_pending_announcements))
        .route("/api/matches", post(handlers::matches::create_match))
        .route("/api/matches/update", post(handlers::matches::update_match))
        .route("/api/matches/:id", axum::routing::delete(handlers::matches::delete_match))
        .route("/api/attendance", post(handlers::attendance::mark_attendance))
        .route("/api/attendance/bulk", post(handlers::attendance::mark_attendance_bulk))
        .route("/api/attendance/list", get(handlers::attendance::list_attendance))
        .route("/api/seasons", post(handlers::seasons::create_season))
        .route("/api/seasons/:id", axum::routing::delete(handlers::seasons::delete_season))
        .route("/api/seasons/:id", axum::routing::patch(handlers::seasons::update_season))
        .route("/api/tournaments", post(handlers::seasons::create_tournament))
        .route("/api/tournaments/:id", axum::routing::delete(handlers::seasons::delete_tournament))
        .route("/api/tournaments/:id", axum::routing::patch(handlers::seasons::update_tournament))
        .route("/api/users", get(handlers::seasons::list_users))
        .route("/api/users/role", post(handlers::seasons::update_user_role))
        .route("/api/users/:id", axum::routing::delete(handlers::seasons::delete_user))
        .route("/api/users/:id", axum::routing::patch(handlers::seasons::update_user))
        .route("/api/admin/protected", get(handlers::admin::protected_admin_route))
        .layer(middleware::from_fn(auth::auth_middleware));
    Router::new()
        .merge(public_api)
        .merge(auth_routes)
        .merge(protected_api)
        .fallback_service(ServeDir::new("static"))
        .with_state(pool)
}
