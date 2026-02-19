mod auth;
mod db;
mod errors;
mod handlers;
mod models;

use axum::{middleware, routing::{get, post}, Router};
use dotenvy::dotenv;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use std::env;
use std::net::SocketAddr;
use std::str::FromStr;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // Load environment
        // Debug: print current working directory
        println!("Current dir: {:?}", std::env::current_dir().unwrap());
        match dotenvy::from_filename(".env") {
            Ok(_) => println!("Loaded .env successfully"),
            Err(e) => println!("Failed to load .env: {:?}", e),
        }
        // Debug: print DATABASE_URL env var
        println!("DATABASE_URL: {:?}", std::env::var("DATABASE_URL"));
    tracing_subscriber::fmt::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    // Connect to SQLite — create database file if it doesn't exist
    let connect_options = SqliteConnectOptions::from_str(&database_url)
        .expect("Invalid DATABASE_URL")
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connect_options)
        .await
        .expect("Failed to connect to SQLite");

    // Initialize database
    db::run_migrations(&pool).await;
    db::seed_roles(&pool).await;

    // ── Public API routes (no auth required) ────────────────────────
    let public_api = Router::new()
        .route("/api/announcements", get(handlers::announcements::list_announcements))
        .route("/api/matches", get(handlers::matches::list_matches))
        .route("/api/seasons", get(handlers::seasons::list_seasons))
        .route("/api/tournaments", get(handlers::seasons::list_tournaments));

    // ── Auth routes (no auth required) ──────────────────────────────
    let auth_routes = Router::new()
        .route("/api/register", post(auth::register_handler))
        .route("/api/login", post(auth::login_handler));

    // ── Protected API routes (JWT auth required) ────────────────────
    let protected_api = Router::new()
        // Announcements
        .route("/api/announcements", post(handlers::announcements::create_announcement))
        .route("/api/announcements/approve", post(handlers::announcements::approve_announcement))
        .route("/api/announcements/pending", get(handlers::announcements::list_pending_announcements))
        // Matches
        .route("/api/matches", post(handlers::matches::create_match))
        .route("/api/matches/update", post(handlers::matches::update_match))
        // Attendance
        .route("/api/attendance", post(handlers::attendance::mark_attendance))
        .route("/api/attendance/bulk", post(handlers::attendance::mark_attendance_bulk))
        .route("/api/attendance/list", get(handlers::attendance::list_attendance))
        // Seasons & Tournaments
        .route("/api/seasons", post(handlers::seasons::create_season))
        .route("/api/tournaments", post(handlers::seasons::create_tournament))
        // User management
        .route("/api/users", get(handlers::seasons::list_users))
        .route("/api/users/role", post(handlers::seasons::update_user_role))
        // Apply auth middleware to all protected routes
        .layer(middleware::from_fn(auth::auth_middleware));

    // ── Combine all routes ──────────────────────────────────────────
    let app = Router::new()
        .merge(public_api)
        .merge(auth_routes)
        .merge(protected_api)
        .fallback_service(ServeDir::new("static"))
        .with_state(pool);

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("🏐 Tornadoes Team Management running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}
