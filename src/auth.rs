use sqlx::Row;
use axum::{
    extract::{Request, State},
    http,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::errors::AppError;
use crate::models::{AuthResponse, LoginRequest, RegisterRequest};

// ─── JWT Configuration ──────────────────────────────────────────────

/// Secret key for JWT signing. In production, use an env variable.
fn jwt_secret() -> String {
    std::env::var("JWT_SECRET").unwrap_or_else(|_| "tornadoes-handball-secret-key-2026".to_string())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: i64,       // user id
    pub email: String,
    pub role: String,
    pub name: String,
    pub exp: usize,     // expiry timestamp
}

/// Generate a JWT token for a user.
pub fn create_token(user_id: i64, email: &str, role: &str, name: &str) -> Result<String, AppError> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .ok_or_else(|| AppError::Internal("Failed to compute token expiry".into()))?
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        email: email.to_string(),
        role: role.to_string(),
        name: name.to_string(),
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret().as_bytes()),
    )?;

    Ok(token)
}

/// Decode and validate a JWT token.
pub fn decode_token(token: &str) -> Result<Claims, AppError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret().as_bytes()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}

// ─── Auth Middleware ─────────────────────────────────────────────────

/// Middleware that extracts JWT from Authorization header and injects Claims.
pub async fn auth_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let token = auth_header
        .and_then(|h| h.strip_prefix("Bearer ").map(|s| s.to_string()))
        .ok_or_else(|| AppError::Unauthorized("Missing or invalid Authorization header".into()))?;

    let claims = decode_token(&token)?;
    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}

// (require_role is not used and has been removed to resolve dead code warning)

/// Check if user is coach or admin.
pub fn require_coach_or_admin(claims: &Claims) -> Result<(), AppError> {
    if claims.role == "coach" || claims.role == "admin" {
        Ok(())
    } else {
        Err(AppError::Unauthorized(
            "Coach or Admin role required".into(),
        ))
    }
}

// ─── Auth Handlers ──────────────────────────────────────────────────

/// POST /api/register — Create a new user account
pub async fn register_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate input
    if payload.email.is_empty() || payload.password.is_empty() || payload.name.is_empty() {
        return Err(AppError::BadRequest("All fields are required".into()));
    }
    if payload.password.len() < 6 {
        return Err(AppError::BadRequest("Password must be at least 6 characters".into()));
    }

    // Check if email already exists
    let exists: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE email = $1")
        .bind(&payload.email)
        .fetch_one(&pool)
        .await?;

    if exists.0 > 0 {
        return Err(AppError::Conflict("Email already registered".into()));
    }

    // Hash password with argon2
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(format!("Password hashing failed: {}", e)))?
        .to_string();

    // Default role: player (id=1)
    let role_id: i64 = 1;
    let result = sqlx::query(
        "INSERT INTO users (email, password_hash, name, role_id, created_at) VALUES ($1, $2, $3, $4, NOW()) RETURNING id",
    )
    .bind(&payload.email)
    .bind(&password_hash)
    .bind(&payload.name)
    .bind(role_id)
    .fetch_one(&pool)
    .await?;

    let user_id: i64 = result.get(0);

    // Issue JWT
    let token = create_token(user_id, &payload.email, "player", &payload.name)?;

    Ok(Json(AuthResponse {
        success: true,
        message: "Registration successful".into(),
        token: Some(token),
        role: Some("player".into()),
        user_id: Some(user_id),
        name: Some(payload.name),
    }))
}

/// POST /api/login — Authenticate and receive JWT
pub async fn login_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Find user by email
    let user = sqlx::query_as::<_, (i64, String, String, i64)>(
        "SELECT id, password_hash, name, role_id FROM users WHERE email = $1",
    )
    .bind(&payload.email)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| AppError::Unauthorized("Invalid email or password".into()))?;

    let (user_id, stored_hash, name, role_id) = user;

    // Verify password
    let parsed_hash = PasswordHash::new(&stored_hash)
        .map_err(|e| AppError::Internal(format!("Hash parse error: {}", e)))?;

    let argon2 = Argon2::default();
    argon2
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::Unauthorized("Invalid email or password".into()))?;

    // Get role name
    let role_name: (String,) = sqlx::query_as("SELECT name FROM roles WHERE id = $1")
        .bind(role_id)
        .fetch_one(&pool)
        .await?;

    // Issue JWT
    let token = create_token(user_id, &payload.email, &role_name.0, &name)?;

    Ok(Json(AuthResponse {
        success: true,
        message: "Login successful".into(),
        token: Some(token),
        role: Some(role_name.0),
        user_id: Some(user_id),
        name: Some(name),
    }))
}
