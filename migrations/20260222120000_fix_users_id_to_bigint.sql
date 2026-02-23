-- 2026-02-22T12:00:00Z
-- Migration: fix_users_id_to_bigint

-- Change users.id to BIGINT
ALTER TABLE users ALTER COLUMN id TYPE BIGINT;

-- Change all referencing foreign keys to BIGINT
ALTER TABLE players ALTER COLUMN user_id TYPE BIGINT;
ALTER TABLE announcements ALTER COLUMN author_id TYPE BIGINT;
ALTER TABLE attendance ALTER COLUMN user_id TYPE BIGINT;
ALTER TABLE matches ALTER COLUMN id TYPE BIGINT;
-- Add more ALTER TABLE statements if other tables reference users(id) or use id as INT
