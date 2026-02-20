-- 2026-02-20T00:02:00Z
-- Migration: create_attendance_table

CREATE TABLE attendance (
    id SERIAL PRIMARY KEY,
    player_id INTEGER NOT NULL REFERENCES players(id) ON DELETE CASCADE,
    match_id INTEGER NOT NULL,
    attended BOOLEAN NOT NULL DEFAULT FALSE,
    timestamp TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(player_id, match_id)
);
