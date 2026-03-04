-- 2026-02-20T13:25:00Z
-- Migration: add_opponent_and_venue_to_matches

ALTER TABLE matches ADD COLUMN IF NOT EXISTS opponent VARCHAR(100);
ALTER TABLE matches ADD COLUMN IF NOT EXISTS venue VARCHAR(100);
