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
    require_coach_or_admin(&claims)?;

    if payload.opponent.is_empty() || payload.venue.is_empty() || payload.date.is_empty() {
        return Err(AppError::BadRequest("Date, opponent, and venue are required".into()));
    }

    sqlx::query(
        "INSERT INTO matches (date, opponent, venue, result, score, match_link, season_id, tournament_id) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
    )
    .bind(&payload.date)
    .bind(&payload.opponent)
    .bind(&payload.venue)
    .bind(&payload.result)
    .bind(&payload.score)
    .bind(&payload.match_link)
    .bind(payload.season_id)
    .bind(payload.tournament_id)
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
    require_coach_or_admin(&claims)?;

    let result = sqlx::query(
        "UPDATE matches SET result = $1, score = $2, match_link = COALESCE($3, match_link) WHERE id = $4"
    )
    .bind(&payload.result)
    .bind(&payload.score)
    .bind(&payload.match_link)
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
    let rows = sqlx::query_as::<_, (i64, String, String, String, Option<String>, Option<String>, Option<String>, i64, Option<i64>)>(
        "SELECT id, date, opponent, venue, result, score, match_link, season_id, tournament_id \
         FROM matches ORDER BY date DESC",
    )
    .fetch_all(&pool)
    .await?;

    let matches: Vec<MatchResponse> = rows
        .into_iter()
        .map(|(id, date, opponent, venue, result, score, match_link, season_id, tournament_id)| {
            MatchResponse {
                id,
                date,
                opponent,
                venue,
                result,
                score,
                match_link,
                season_id,
                tournament_id,
            }
        })
        .collect();

    Ok(Json(matches))
}
