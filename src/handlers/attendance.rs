use axum::{extract::State, response::IntoResponse, Extension, Json};
use sqlx::PgPool;

use crate::auth::{require_coach_or_admin, Claims};
use crate::errors::AppError;
use crate::models::{ApiResponse, AttendanceBulkRequest, AttendanceMarkRequest, AttendanceResponse};

/// POST /api/attendance — Coach/Admin marks a single player's attendance
pub async fn mark_attendance(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<AttendanceMarkRequest>,
) -> Result<impl IntoResponse, AppError> {
    require_coach_or_admin(&claims)?;

    // Upsert: insert or update attendance, including date
    let date = match &payload.date {
        Some(d) => Some(chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d")
            .map_err(|_| AppError::BadRequest("Invalid date format".into()))?),
        None => None,
    };
    // Find player_id from user_id
    let rec = sqlx::query!("SELECT id FROM players WHERE user_id = $1", payload.user_id)
        .fetch_one(&pool)
        .await?;
    let player_id = rec.id;
    sqlx::query(
        "INSERT INTO attendance (player_id, match_id, attended, date) VALUES ($1, $2, $3, $4) \
         ON CONFLICT (player_id, match_id) DO UPDATE SET attended = EXCLUDED.attended, date = EXCLUDED.date"
    )
    .bind(player_id)
    .bind(payload.match_id)
    .bind(payload.present)
    .bind(date)
    .execute(&pool)
    .await?;

    Ok(Json(ApiResponse {
        success: true,
        message: "Attendance recorded.".into(),
    }))
}

/// POST /api/attendance/bulk — Coach/Admin marks attendance for multiple players
pub async fn mark_attendance_bulk(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<AttendanceBulkRequest>,
) -> Result<impl IntoResponse, AppError> {
    require_coach_or_admin(&claims)?;

    for record in &payload.records {
        let date = match record.date.clone().or_else(|| payload.date.clone()) {
            Some(d) => Some(chrono::NaiveDate::parse_from_str(&d, "%Y-%m-%d")
                .map_err(|_| AppError::BadRequest("Invalid date format".into()))?),
            None => None,
        };
        // Find player_id from user_id
        let rec = sqlx::query!("SELECT id FROM players WHERE user_id = $1", record.user_id)
            .fetch_optional(&pool)
            .await?;
        let player_id = match rec {
            Some(r) => r.id,
            None => {
                tracing::warn!("No player found for user_id {}", record.user_id);
                continue; // Skip this record
            }
        };
        let match_id = payload.match_id;
        sqlx::query(
            "INSERT INTO attendance (player_id, match_id, attended, date) VALUES ($1, $2, $3, $4) \
             ON CONFLICT (player_id, match_id) DO UPDATE SET attended = EXCLUDED.attended, date = EXCLUDED.date"
        )
        .bind(player_id)
        .bind(match_id)
        .bind(record.present)
        .bind(date)
        .execute(&pool)
        .await?;
    }

    Ok(Json(ApiResponse {
        success: true,
        message: format!("{} attendance records saved.", payload.records.len()),
    }))
}

/// GET /api/attendance — Returns attendance records.
/// Coach/Admin: all records. Player: only their own.
pub async fn list_attendance(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse, AppError> {
    let rows = if claims.role == "coach" || claims.role == "admin" {
        sqlx::query_as::<_, (i64, i64, Option<String>, i64, bool, Option<String>)>(
            "SELECT a.id, a.player_id, u.name, a.match_id, a.attended, a.date \
             FROM attendance a LEFT JOIN players p ON a.player_id = p.id LEFT JOIN users u ON p.user_id = u.id \
             ORDER BY a.match_id DESC",
        )
        .fetch_all(&pool)
        .await?
    } else {
        sqlx::query_as::<_, (i64, i64, Option<String>, i64, bool, Option<String>)>(
            "SELECT a.id, a.player_id, u.name, a.match_id, a.attended, a.date \
             FROM attendance a LEFT JOIN players p ON a.player_id = p.id LEFT JOIN users u ON p.user_id = u.id \
             WHERE a.player_id = $1 ORDER BY a.match_id DESC",
        )
        .bind(claims.sub)
        .fetch_all(&pool)
        .await?
    };

    let attendance: Vec<AttendanceResponse> = rows
        .into_iter()
        .map(|(id, user_id, user_name, match_id, present, date)| AttendanceResponse {
            id,
            user_id,
            user_name,
            match_id,
            present,
            date,
        })
        .collect();

    Ok(Json(attendance))
}
