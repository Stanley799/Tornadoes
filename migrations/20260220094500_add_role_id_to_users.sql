-- 2026-02-20T09:45:00Z
-- Migration: add_role_id_to_users

ALTER TABLE users ADD COLUMN role_id INTEGER NOT NULL DEFAULT 1;
