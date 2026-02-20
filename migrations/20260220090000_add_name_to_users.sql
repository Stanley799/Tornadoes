-- 2026-02-20T09:00:00Z
-- Migration: add_name_to_users

ALTER TABLE users ADD COLUMN name VARCHAR(100) NOT NULL DEFAULT '';
