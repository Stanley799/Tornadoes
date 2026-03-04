use axum::{extract::{Path, State}, response::IntoResponse, Extension, Json};
use sqlx::PgPool;
use crate::auth::{require_coach_or_admin, Claims};
use crate::errors::AppError;
use crate::models::{MatchEventCreateRequest, MatchEventResponse, validate_event_type, validate_period};

/// POST /api/matches/{match_id}/events — Insert match event
pub async fn create_match_event(
    Path(match_id): Path<i64>,
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<MatchEventCreateRequest>,
) -> Result<impl IntoResponse, AppError> {
    require_coach_or_admin(&claims)?;

    // Validate event_type
    if !validate_event_type(&payload.event_type) {
        return Err(AppError::BadRequest("Invalid event_type".into()));
    }
    // Validate period
    if !validate_period(&payload.period) {
        return Err(AppError::BadRequest("Invalid period".into()));
    }
    // Validate minute
    if payload.minute < 0 {
        return Err(AppError::BadRequest("Minute must be >= 0".into()));
    }

    // Confirm match exists
    let match_exists = sqlx::query_scalar::<_, i64>("SELECT id FROM matches WHERE id = $1")
        .bind(match_id)
        .fetch_optional(&pool)
        .await?;
    if match_exists.is_none() {
        return Err(AppError::NotFound("Match not found".into()));
    }

    // Confirm player exists
    let player_exists = sqlx::query_scalar::<_, i64>("SELECT id FROM players WHERE id = $1")
        .bind(payload.player_id)
        .fetch_optional(&pool)
        .await?;
    if player_exists.is_none() {
        return Err(AppError::NotFound("Player not found".into()));
    }

    // Insert event
    let rec = sqlx::query_as!(
        MatchEventResponse,
        r#"
        INSERT INTO match_events
            (match_id, player_id, event_type, minute, period, is_fast_break, is_penalty, created_by)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id, match_id, player_id, event_type, minute, period, is_fast_break, is_penalty, created_by, to_char(created_at, 'YYYY-MM-DD"T"HH24:MI:SS') as created_at
        "#,
        match_id,
        payload.player_id,
        payload.event_type,
        payload.minute,
        payload.period,
        payload.is_fast_break,
        payload.is_penalty,
        claims.sub
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(rec))
}
/// DELETE /api/matches/:id — Coach/Admin deletes a match
use axum::extract::Path;
pub async fn delete_match(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    require_coach_or_admin(&claims)?;
    let result = sqlx::query("DELETE FROM matches WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await?;
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Match not found".into()));
    }
    Ok(Json(ApiResponse {
        success: true,
        message: "Match deleted.".into(),
    }))
}
use axum::{extract::State, response::IntoResponse, Extension, Json};
use sqlx::PgPool;

use crate::auth::{require_coach_or_admin, Claims};
use crate::errors::AppError;
use crate::models::{ApiResponse, MatchCreateRequest, MatchResponse, MatchUpdateRequest};

/// POST /api/matches — Coach/Admin creates a new match
pub async fn create_match(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<MatchCreateRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validation for new fields
    if payload.match_date.is_empty() || payload.home_team.is_empty() || payload.away_team.is_empty() || payload.location.as_deref().unwrap_or("").is_empty() {
        return Err(AppError::BadRequest("Date, home team, away team, and location are required".into()));
    }
        let date = chrono::NaiveDate::parse_from_str(&payload.match_date, "%Y-%m-%d")
        .map_err(|_| AppError::BadRequest("Invalid date format. Use YYYY-MM-DD.".into()))?;
    sqlx::query(
        "INSERT INTO matches (date, home_team, away_team, location, tournament_id, home_score, away_score) \
         VALUES ($1, $2, $3, $4, $5, $6, $7)",
    )
        .bind(date)
    .bind(&payload.home_team)
    .bind(&payload.away_team)
    .bind(&payload.location)
    .bind(payload.tournament_id)
    .bind(payload.home_score)
    .bind(payload.away_score)
    .execute(&pool)
    .await?;
    Ok(Json(ApiResponse {
        success: true,
        message: "Match created.".into(),
    }))
}

/// POST /api/matches/update — Coach/Admin updates match result/score
pub async fn update_match(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<MatchUpdateRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Only update home_score and away_score if present in struct
    let result = sqlx::query(
        "UPDATE matches SET home_score = $1, away_score = $2 WHERE id = $3",
    )
    .bind(payload.home_score)
    .bind(payload.away_score)
    .bind(payload.id)
    .execute(&pool)
    .await?;
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Match not found".into()));
    }
    Ok(Json(ApiResponse {
        success: true,
        message: "Match updated.".into(),
    }))
}

/// GET /api/matches — Public: list all matches (newest first)
pub async fn list_matches(
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, AppError> {
    let rows = sqlx::query_as::<_, (i64, chrono::NaiveDate, String, String, Option<String>, Option<i64>, Option<i32>, Option<i32>)>(
        "SELECT id, date, home_team, away_team, location, tournament_id, home_score, away_score FROM matches ORDER BY date DESC",
    )
    .fetch_all(&pool)
    .await?;

    let matches: Vec<MatchResponse> = rows
        .into_iter()
            .map(|(id, date, home_team, away_team, location, tournament_id, home_score, away_score)| {
            MatchResponse {
                id,
                    match_date: date.to_string(),
                home_team,
                away_team,
                location,
                tournament_id,
                home_score,
                away_score,
            }
        })
        .collect();

    Ok(Json(matches))
}
