-- 20260222160000_fix_all_id_types_to_bigint.sql
-- This migration fixes all type mismatches by converting all relevant IDs and foreign keys to BIGINT.
-- It also drops and recreates all affected foreign key constraints.

-- 1. Drop foreign key constraints
ALTER TABLE IF EXISTS announcements DROP CONSTRAINT IF EXISTS announcements_author_id_fkey;
ALTER TABLE IF EXISTS attendance DROP CONSTRAINT IF EXISTS attendance_player_id_fkey;
ALTER TABLE IF EXISTS players DROP CONSTRAINT IF EXISTS players_user_id_fkey;
ALTER TABLE IF EXISTS matches DROP CONSTRAINT IF EXISTS matches_tournament_id_fkey;
ALTER TABLE IF EXISTS tournaments DROP CONSTRAINT IF EXISTS tournaments_season_id_fkey;

-- 2. Change column types to bigint
ALTER TABLE announcements ALTER COLUMN id TYPE bigint;
ALTER TABLE announcements ALTER COLUMN author_id TYPE bigint;

ALTER TABLE attendance ALTER COLUMN id TYPE bigint;
ALTER TABLE attendance ALTER COLUMN player_id TYPE bigint;
ALTER TABLE attendance ALTER COLUMN match_id TYPE bigint;

ALTER TABLE players ALTER COLUMN id TYPE bigint;
ALTER TABLE players ALTER COLUMN user_id TYPE bigint;

ALTER TABLE matches ALTER COLUMN id TYPE bigint;
ALTER TABLE matches ALTER COLUMN tournament_id TYPE bigint;
ALTER TABLE matches ALTER COLUMN season_id TYPE bigint;

ALTER TABLE tournaments ALTER COLUMN id TYPE bigint;
ALTER TABLE tournaments ALTER COLUMN season_id TYPE bigint;

ALTER TABLE roles ALTER COLUMN id TYPE bigint;

-- 3. Re-add foreign key constraints
ALTER TABLE announcements ADD CONSTRAINT announcements_author_id_fkey FOREIGN KEY (author_id) REFERENCES users(id);
ALTER TABLE attendance ADD CONSTRAINT attendance_player_id_fkey FOREIGN KEY (player_id) REFERENCES players(id) ON DELETE CASCADE;
ALTER TABLE players ADD CONSTRAINT players_user_id_fkey FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE;
ALTER TABLE matches ADD CONSTRAINT matches_tournament_id_fkey FOREIGN KEY (tournament_id) REFERENCES tournaments(id) ON DELETE SET NULL;
ALTER TABLE tournaments ADD CONSTRAINT tournaments_season_id_fkey FOREIGN KEY (season_id) REFERENCES seasons(id);

-- 4. (Optional) Set sequences for new bigserial columns if needed
-- This is only needed if you encounter sequence errors after migration.
-- Example:
-- SELECT setval(pg_get_serial_sequence('announcements', 'id'), COALESCE(MAX(id),1), false) FROM announcements;
-- Repeat for each table as needed.
