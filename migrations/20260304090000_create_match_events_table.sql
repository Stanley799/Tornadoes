-- Create match_events table for event tracking

CREATE TABLE IF NOT EXISTS match_events (
    id BIGSERIAL PRIMARY KEY,
    match_id BIGINT NOT NULL,
    player_id BIGINT NOT NULL,
    event_type VARCHAR NOT NULL,
    minute INTEGER CHECK (minute >= 0),
    period VARCHAR NOT NULL CHECK (period IN ('first_half', 'second_half', 'extra_time')),
    is_fast_break BOOLEAN DEFAULT FALSE NOT NULL,
    is_penalty BOOLEAN DEFAULT FALSE NOT NULL,
    created_by BIGINT NOT NULL,
    created_at TIMESTAMP DEFAULT now() NOT NULL,

    CONSTRAINT fk_match_events_match
        FOREIGN KEY (match_id) REFERENCES matches(id) ON DELETE CASCADE,
    CONSTRAINT fk_match_events_player
        FOREIGN KEY (player_id) REFERENCES players(id) ON DELETE CASCADE,
    CONSTRAINT fk_match_events_created_by
        FOREIGN KEY (created_by) REFERENCES users(id)
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_match_events_match_id ON match_events(match_id);
CREATE INDEX IF NOT EXISTS idx_match_events_player_id ON match_events(player_id);
CREATE INDEX IF NOT EXISTS idx_match_events_event_type ON match_events(event_type);
