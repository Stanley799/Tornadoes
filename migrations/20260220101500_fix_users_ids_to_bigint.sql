-- 2026-02-20T10:15:00Z
-- Migration: fix_users_ids_to_bigint

ALTER TABLE users ALTER COLUMN id TYPE BIGINT;
ALTER TABLE users ALTER COLUMN role_id TYPE BIGINT;
