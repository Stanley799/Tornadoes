-- Fix migration syntax for previously applied migrations
-- Only run if columns or constraints are not already correct

-- Coaches table: ensure it exists and columns are correct
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'coaches') THEN
        CREATE TABLE coaches (
            id BIGSERIAL PRIMARY KEY,
            user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            first_name VARCHAR(50) NOT NULL,
            last_name VARCHAR(50) NOT NULL,
            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP NOT NULL DEFAULT NOW()
        );
    END IF;
END$$;

-- Remove unsupported IF EXISTS/ALTER COLUMN IF EXISTS from previous migrations
-- Only alter columns if needed
DO $$
DECLARE
    col_exists BOOLEAN;
BEGIN
    -- Example for coaches.id
    SELECT EXISTS (
        SELECT 1 FROM information_schema.columns WHERE table_name='coaches' AND column_name='id' AND data_type='bigint'
    ) INTO col_exists;
    IF NOT col_exists THEN
        ALTER TABLE coaches ALTER COLUMN id TYPE BIGINT;
    END IF;
END$$;

-- Repeat similar blocks for other tables/columns if needed
