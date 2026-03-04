-- 2026-02-20T14:00:00Z
-- Migration: add_season_id_to_matches_and_tournaments

ALTER TABLE matches ADD COLUMN IF NOT EXISTS season_id INTEGER;
ALTER TABLE tournaments ADD COLUMN IF NOT EXISTS season_id INTEGER;
