-- 2026-02-20T13:20:00Z
-- Migration: rename_match_date_to_date_in_matches

ALTER TABLE matches RENAME COLUMN match_date TO date;
