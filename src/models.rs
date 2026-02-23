#[derive(Deserialize)]
pub struct UserUpdateRequest {
    pub name: String,
    pub email: String,
    pub role: String,
}
#[derive(Deserialize)]
pub struct SeasonUpdateRequest {
    pub name: String,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Deserialize)]
pub struct TournamentUpdateRequest {
    pub name: String,
    pub season_id: i64,
}
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

// ─── Generic API Response ────────────────────────────────────────────

#[derive(Serialize)]
pub struct ApiResponse {
    pub success: bool,
    pub message: String,
}

// ─── Auth ────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub role: String, // "coach" or "player"
    pub player_details: Option<PlayerRegisterDetails>, // Only for player
}

#[derive(Deserialize)]
pub struct PlayerRegisterDetails {
    pub date_of_birth: NaiveDate,
    pub position: String,
    pub jersey_number: i32,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub success: bool,
    pub message: String,
    pub token: Option<String>,
    pub role: Option<String>,
    pub user_id: Option<i64>,
    pub name: Option<String>,
}

// ─── Announcements ──────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct AnnouncementCreateRequest {
    pub title: String,
    pub content: String,
    pub external_link: Option<String>,
}

#[derive(Deserialize)]
pub struct ApproveRequest {
    pub id: i64,
}

#[derive(Serialize)]
pub struct AnnouncementResponse {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub external_link: Option<String>,
    pub author_id: i64,
    pub author_name: Option<String>,
    pub created_at: String,
    pub status: String,
}

// ─── Matches ────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct MatchCreateRequest {
    pub date: String,
    pub opponent: String,
    pub venue: String,
    pub result: Option<String>,
    pub score: Option<String>,
    pub match_link: Option<String>,
    pub season_id: i64,
    pub tournament_id: Option<i64>,
}

#[derive(Deserialize)]
pub struct MatchUpdateRequest {
    pub id: i64,
    pub result: Option<String>,
    pub score: Option<String>,
    pub match_link: Option<String>,
}

#[derive(Serialize)]
pub struct MatchResponse {
    pub id: i64,
    pub date: String,
    pub opponent: String,
    pub venue: String,
    pub result: Option<String>,
    pub score: Option<String>,
    pub match_link: Option<String>,
    pub season_id: i64,
    pub tournament_id: Option<i64>,
}

// ─── Attendance ─────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct AttendanceMarkRequest {
    pub user_id: i64,
    pub match_id: i64,
    pub present: bool,
    pub date: Option<String>,
}

#[derive(Deserialize)]
pub struct AttendanceBulkRequest {
    pub match_id: i64,
    pub records: Vec<AttendanceRecord>,
    pub date: Option<String>,
}

#[derive(Deserialize)]
pub struct AttendanceRecord {
    pub user_id: i64,
    pub present: bool,
    pub date: Option<String>,
}

#[derive(Serialize)]
pub struct AttendanceResponse {
    pub id: i64,
    pub user_id: i64,
    pub user_name: Option<String>,
    pub match_id: i64,
    pub present: bool,
    pub date: Option<String>,
}

// ─── Seasons & Tournaments ──────────────────────────────────────────

#[derive(Deserialize)]
pub struct SeasonCreateRequest {
    pub name: String,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Serialize)]
pub struct SeasonResponse {
    pub id: i64,
    pub name: String,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Deserialize)]
pub struct TournamentCreateRequest {
    pub name: String,
    pub season_id: i64,
}

#[derive(Serialize)]
pub struct TournamentResponse {
    pub id: i64,
    pub name: String,
    pub season_id: i64,
}

// ─── User Management ────────────────────────────────────────────────

#[derive(Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub email: String,
    pub name: String,
    pub role: String,
}

#[derive(Deserialize)]
pub struct RoleUpdateRequest {
    pub user_id: i64,
    pub role_name: String,
}
