/// POST /api/announcements/reject — Coach/Admin rejects a pending announcement
pub async fn reject_announcement(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<ApproveRequest>,
) -> Result<impl IntoResponse, AppError> {
    require_coach_or_admin(&claims)?;
    let result = sqlx::query("UPDATE announcements SET status = 'rejected' WHERE id = ? AND status = 'pending'")
        .bind(payload.id)
        .execute(&pool)
        .await?;
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Announcement not found or already processed".into()));
    }
    Ok(Json(ApiResponse {
        success: true,
        message: "Announcement rejected.".into(),
    }))
}
use axum::{extract::State, response::IntoResponse, Extension, Json};
use sqlx::PgPool;

use crate::auth::{require_coach_or_admin, Claims};
use crate::errors::AppError;
use crate::models::{
    AnnouncementCreateRequest, AnnouncementResponse, ApiResponse, ApproveRequest,
};

/// POST /api/announcements — Any authenticated user can submit (status = pending)
pub async fn create_announcement(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<AnnouncementCreateRequest>,
) -> Result<impl IntoResponse, AppError> {
    if payload.title.is_empty() || payload.content.is_empty() {
        return Err(AppError::BadRequest("Title and content are required".into()));
    }
    // Only users can post announcements
    if claims.role != "user" {
        return Err(AppError::Unauthorized("Only users can post announcements.".into()));
    }
    sqlx::query(
        "INSERT INTO announcements (title, content, external_link, author_id, created_at, status) \
         VALUES ($1, $2, $3, $4, NOW(), 'pending')"
    )
    .bind(&payload.title)
    .bind(&payload.content)
    .bind(&payload.external_link)
    .bind(claims.sub)
    .execute(&pool)
    .await?;

    Ok(Json(ApiResponse {
        success: true,
        message: "Announcement submitted for approval.".into(),
    }))
}

/// POST /api/announcements/approve — Coach/Admin approves a pending announcement
pub async fn approve_announcement(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<ApproveRequest>,
) -> Result<impl IntoResponse, AppError> {
    require_coach_or_admin(&claims)?;

    let result = sqlx::query("UPDATE announcements SET status = 'approved' WHERE id = ? AND status = 'pending'")
        .bind(payload.id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Announcement not found or already approved".into()));
    }

    Ok(Json(ApiResponse {
        success: true,
        message: "Announcement approved.".into(),
    }))
}

/// GET /api/announcements — Public: list approved announcements
pub async fn list_announcements(
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, AppError> {
    let rows = sqlx::query_as::<_, (i64, String, String, Option<String>, i64, String, String)>(
        "SELECT a.id, a.title, a.content, a.external_link, a.author_id, a.created_at, a.status \
         FROM announcements a WHERE a.status = 'approved' ORDER BY a.created_at DESC",
    )
    .fetch_all(&pool)
    .await?;

    let announcements: Vec<AnnouncementResponse> = rows
        .into_iter()
        .map(|(id, title, content, external_link, author_id, created_at, status)| {
            AnnouncementResponse {
                id,
                title,
                content,
                external_link,
                author_id,
                author_name: None,
                created_at,
                status,
            }
        })
        .collect();

    Ok(Json(announcements))
}

/// GET /api/announcements/pending — Coach/Admin: list pending announcements
pub async fn list_pending_announcements(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse, AppError> {
    require_coach_or_admin(&claims)?;

    let rows = sqlx::query_as::<_, (i64, String, String, Option<String>, i64, String, String)>(
        "SELECT a.id, a.title, a.content, a.external_link, a.author_id, a.created_at, a.status \
         FROM announcements a WHERE a.status = 'pending' ORDER BY a.created_at DESC",
    )
    .fetch_all(&pool)
    .await?;

    let announcements: Vec<AnnouncementResponse> = rows
        .into_iter()
        .map(|(id, title, content, external_link, author_id, created_at, status)| {
            AnnouncementResponse {
                id,
                title,
                content,
                external_link,
                author_id,
                author_name: None,
                created_at,
                status,
            }
        })
        .collect();

    Ok(Json(announcements))
}
