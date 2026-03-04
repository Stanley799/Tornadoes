-- 2026-02-20T13:20:00Z
-- Migration: rename_match_date_to_date_in_matches

DO $$
BEGIN
	IF EXISTS (
		SELECT 1 FROM information_schema.columns 
		WHERE table_name='matches' AND column_name='match_date'
	) THEN
		EXECUTE 'ALTER TABLE matches RENAME COLUMN match_date TO date';
	END IF;
END$$;
