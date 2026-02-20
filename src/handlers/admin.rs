use axum::{response::IntoResponse, Extension, Json};
use crate::auth::Claims;
use crate::errors::AppError;

/// GET /api/admin/protected — Only accessible by admin
pub async fn protected_admin_route(
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse, AppError> {
    if claims.role == "admin" {
        Ok(Json("Admin access granted"))
    } else {
        Err(AppError::Forbidden("Forbidden: Admins only".into()))
    }
}
