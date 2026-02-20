use axum::{extract::State, response::IntoResponse, Extension, Json};
use sqlx::PgPool;

use crate::auth::{require_coach_or_admin, Claims};
use crate::errors::AppError;
use crate::models::{
    ApiResponse, SeasonCreateRequest, SeasonResponse, TournamentCreateRequest, TournamentResponse,
    RoleUpdateRequest, UserResponse,
};

// ─── Seasons ────────────────────────────────────────────────────────

/// POST /api/seasons — Admin creates a season
pub async fn create_season(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<SeasonCreateRequest>,
) -> Result<impl IntoResponse, AppError> {
    require_coach_or_admin(&claims)?;

    if payload.name.is_empty() {
        return Err(AppError::BadRequest("Season name is required".into()));
    }

    sqlx::query("INSERT INTO seasons (name, start_date, end_date) VALUES ($1, $2, $3)")
        .bind(&payload.name)
        .bind(&payload.start_date)
        .bind(&payload.end_date)
        .execute(&pool)
        .await?;

    Ok(Json(ApiResponse {
        success: true,
        message: "Season created.".into(),
    }))
}

/// GET /api/seasons — Public: list all seasons
pub async fn list_seasons(
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, AppError> {
    let rows = sqlx::query_as::<_, (i64, String, String, String)>(
        "SELECT id, name, start_date, end_date FROM seasons ORDER BY start_date DESC",
    )
    .fetch_all(&pool)
    .await?;

    let seasons: Vec<SeasonResponse> = rows
        .into_iter()
        .map(|(id, name, start_date, end_date)| SeasonResponse {
            id,
            name,
            start_date,
            end_date,
        })
        .collect();

    Ok(Json(seasons))
}

// ─── Tournaments ────────────────────────────────────────────────────

/// POST /api/tournaments — Admin creates a tournament
pub async fn create_tournament(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<TournamentCreateRequest>,
) -> Result<impl IntoResponse, AppError> {
    require_coach_or_admin(&claims)?;

    if payload.name.is_empty() {
        return Err(AppError::BadRequest("Tournament name is required".into()));
    }

    sqlx::query("INSERT INTO tournaments (name, season_id) VALUES (?, ?)")
        .bind(&payload.name)
        .bind(payload.season_id)
        .execute(&pool)
        .await?;

    Ok(Json(ApiResponse {
        success: true,
        message: "Tournament created.".into(),
    }))
}

/// GET /api/tournaments — Public: list all tournaments
pub async fn list_tournaments(
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, AppError> {
    let rows = sqlx::query_as::<_, (i64, String, i64)>(
        "SELECT id, name, season_id FROM tournaments ORDER BY id DESC",
    )
    .fetch_all(&pool)
    .await?;

    let tournaments: Vec<TournamentResponse> = rows
        .into_iter()
        .map(|(id, name, season_id)| TournamentResponse {
            id,
            name,
            season_id,
        })
        .collect();

    Ok(Json(tournaments))
}

// ─── User Management (Admin) ────────────────────────────────────────

/// GET /api/users — Admin: list all users
pub async fn list_users(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse, AppError> {
    require_coach_or_admin(&claims)?;

    let rows = sqlx::query_as::<_, (i64, String, String, String)>(
        "SELECT u.id, u.email, u.name, r.name FROM users u JOIN roles r ON u.role_id = r.id ORDER BY u.name",
    )
    .fetch_all(&pool)
    .await?;

    let users: Vec<UserResponse> = rows
        .into_iter()
        .map(|(id, email, name, role)| UserResponse {
            id,
            email,
            name,
            role,
        })
        .collect();

    Ok(Json(users))
}

/// POST /api/users/role — Admin: change a user's role
pub async fn update_user_role(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<RoleUpdateRequest>,
) -> Result<impl IntoResponse, AppError> {
    if claims.role != "admin" {
        return Err(AppError::Unauthorized("Admin role required".into()));
    }

    // Find role id
    let role: Option<(i64,)> = sqlx::query_as("SELECT id FROM roles WHERE name = ?")
        .bind(&payload.role_name)
        .fetch_optional(&pool)
        .await?;

    let role_id = role
        .ok_or_else(|| AppError::BadRequest(format!("Role '{}' does not exist", payload.role_name)))?
        .0;

    let result = sqlx::query("UPDATE users SET role_id = ? WHERE id = ?")
        .bind(role_id)
        .bind(payload.user_id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("User not found".into()));
    }

    Ok(Json(ApiResponse {
        success: true,
        message: format!("User role updated to '{}'.", payload.role_name),
    }))
}
