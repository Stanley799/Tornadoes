/// PATCH /api/users/:id — Admin updates a user's name/email/role
use crate::models::UserUpdateRequest;
pub async fn update_user(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
    Json(payload): Json<UserUpdateRequest>,
) -> Result<impl IntoResponse, AppError> {
    if claims.role != "admin" {
        return Err(AppError::Unauthorized("Admin role required".into()));
    }
    // Find role id
    let role: Option<(i64,)> = sqlx::query_as("SELECT id FROM roles WHERE name = ?")
        .bind(&payload.role)
        .fetch_optional(&pool)
        .await?;
    let role_id = role.ok_or_else(|| AppError::BadRequest(format!("Role '{}' does not exist", payload.role)))?.0;
    let result = sqlx::query("UPDATE users SET name = ?, email = ?, role_id = ? WHERE id = ?")
        .bind(&payload.name)
        .bind(&payload.email)
        .bind(role_id)
        .bind(id)
        .execute(&pool)
        .await?;
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("User not found".into()));
    }
    Ok(Json(ApiResponse {
        success: true,
        message: "User updated.".into(),
    }))
}
/// DELETE /api/users/:id — Admin deletes a user
pub async fn delete_user(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    if claims.role != "admin" {
        return Err(AppError::Unauthorized("Admin role required".into()));
    }
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await?;
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("User not found".into()));
    }
    Ok(Json(ApiResponse {
        success: true,
        message: "User deleted.".into(),
    }))
}
/// PATCH /api/seasons/:id — Admin updates a season
use crate::models::SeasonUpdateRequest;
pub async fn update_season(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
    Json(payload): Json<SeasonUpdateRequest>,
) -> Result<impl IntoResponse, AppError> {
    require_coach_or_admin(&claims)?;
    let result = sqlx::query("UPDATE seasons SET name = $1, start_date = $2, end_date = $3 WHERE id = $4")
        .bind(&payload.name)
        .bind(&payload.start_date)
        .bind(&payload.end_date)
        .bind(id)
        .execute(&pool)
        .await?;
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Season not found".into()));
    }
    Ok(Json(ApiResponse {
        success: true,
        message: "Season updated.".into(),
    }))
}

/// PATCH /api/tournaments/:id — Admin updates a tournament
use crate::models::TournamentUpdateRequest;
pub async fn update_tournament(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
    Json(payload): Json<TournamentUpdateRequest>,
) -> Result<impl IntoResponse, AppError> {
    require_coach_or_admin(&claims)?;
    let result = sqlx::query("UPDATE tournaments SET name = $1, season_id = $2 WHERE id = $3")
        .bind(&payload.name)
        .bind(payload.season_id)
        .bind(id)
        .execute(&pool)
        .await?;
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Tournament not found".into()));
    }
    Ok(Json(ApiResponse {
        success: true,
        message: "Tournament updated.".into(),
    }))
}
/// DELETE /api/seasons/:id — Admin deletes a season
use axum::extract::Path;
pub async fn delete_season(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    require_coach_or_admin(&claims)?;
    let result = sqlx::query("DELETE FROM seasons WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await?;
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Season not found".into()));
    }
    Ok(Json(ApiResponse {
        success: true,
        message: "Season deleted.".into(),
    }))
}

/// DELETE /api/tournaments/:id — Admin deletes a tournament
pub async fn delete_tournament(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    require_coach_or_admin(&claims)?;
    let result = sqlx::query("DELETE FROM tournaments WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await?;
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Tournament not found".into()));
    }
    Ok(Json(ApiResponse {
        success: true,
        message: "Tournament deleted.".into(),
    }))
}
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
