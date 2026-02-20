-- 2026-02-20T13:35:00Z
-- Migration: add_match_link_to_matches

ALTER TABLE matches ADD COLUMN match_link VARCHAR(255);
