-- 2026-02-20T10:30:00Z
-- Migration: seed_roles_admin

INSERT INTO roles (name) VALUES ('admin') ON CONFLICT (name) DO NOTHING;
INSERT INTO roles (name) VALUES ('coach') ON CONFLICT (name) DO NOTHING;
INSERT INTO roles (name) VALUES ('player') ON CONFLICT (name) DO NOTHING;
