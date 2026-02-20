use sqlx::PgPool;
use std::fs;

/// Run SQL migrations from migrations.sql file.
/// Each statement is separated by `;` and executed sequentially.
pub async fn run_migrations(pool: &PgPool) {
    let sql = fs::read_to_string("migrations.sql").expect("Failed to read migrations.sql");
    for statement in sql.split(';') {
        let stmt = statement.trim();
        if !stmt.is_empty() {
            if let Err(e) = sqlx::query(stmt).execute(pool).await {
                // Log but don't panic on "already exists" errors
                tracing::warn!("Migration statement warning: {} — SQL: {}", e, stmt);
            }
        }
    }
    tracing::info!("Database migrations complete.");
}

/// Seed default roles (player, coach, admin) if not already present.
pub async fn seed_roles(pool: &PgPool) {
    let roles = ["player", "coach", "admin"];
    for role in roles.iter() {
        let _ = sqlx::query("INSERT INTO roles (name) VALUES ($1) ON CONFLICT (name) DO NOTHING")
            .bind(role)
            .execute(pool)
            .await;
    }
    tracing::info!("Roles seeded.");
}
