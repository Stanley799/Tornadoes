-- 2026-02-20T13:30:00Z
-- Migration: add_result_and_score_to_matches

ALTER TABLE matches ADD COLUMN result VARCHAR(100);
ALTER TABLE matches ADD COLUMN score VARCHAR(50);
