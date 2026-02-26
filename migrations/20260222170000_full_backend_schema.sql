-- handball_team_app backend schema: all tables, correct order, all IDs and FKs as BIGINT

-- Drop all tables if they exist
DROP TABLE IF EXISTS attendance CASCADE;
DROP TABLE IF EXISTS announcements CASCADE;
DROP TABLE IF EXISTS players CASCADE;
DROP TABLE IF EXISTS coaches CASCADE;
DROP TABLE IF EXISTS matches CASCADE;
DROP TABLE IF EXISTS tournaments CASCADE;
DROP TABLE IF EXISTS seasons CASCADE;
DROP TABLE IF EXISTS users CASCADE;
DROP TABLE IF EXISTS roles CASCADE;

-- 1. roles
CREATE TABLE roles (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE
);

-- 2. users (references roles)
CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    email VARCHAR(100) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT now() NOT NULL,
    updated_at TIMESTAMP DEFAULT now() NOT NULL,
    name VARCHAR(100) DEFAULT '' NOT NULL,
    role_id BIGINT DEFAULT 1 NOT NULL REFERENCES roles(id)
);

-- 3. seasons
CREATE TABLE seasons (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL
);

-- 4. tournaments (references seasons)
CREATE TABLE tournaments (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE,
    location VARCHAR(100),
    created_at TIMESTAMP DEFAULT now() NOT NULL,
    updated_at TIMESTAMP DEFAULT now() NOT NULL,
    season_id BIGINT REFERENCES seasons(id)
);

-- 5. matches (references tournaments, seasons)
CREATE TABLE matches (
    id BIGSERIAL PRIMARY KEY,
    tournament_id BIGINT REFERENCES tournaments(id) ON DELETE SET NULL,
    date DATE NOT NULL,
    location VARCHAR(100),
    home_team VARCHAR(100) NOT NULL,
    away_team VARCHAR(100) NOT NULL,
    home_score INT,
    away_score INT,
    created_at TIMESTAMP DEFAULT now() NOT NULL,
    updated_at TIMESTAMP DEFAULT now() NOT NULL,
    opponent VARCHAR(100),
    venue VARCHAR(100),
    result VARCHAR(100),
    score VARCHAR(50),
    match_link VARCHAR(255),
    season_id BIGINT REFERENCES seasons(id)
);

-- 6. players (references users)
CREATE TABLE players (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    first_name VARCHAR(50) NOT NULL,
    last_name VARCHAR(50) NOT NULL,
    date_of_birth DATE NOT NULL,
    position VARCHAR(30),
    jersey_number INT,
    created_at TIMESTAMP DEFAULT now() NOT NULL,
    updated_at TIMESTAMP DEFAULT now() NOT NULL
);

-- 7. coaches (references users)
CREATE TABLE coaches (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    first_name VARCHAR(50) NOT NULL,
    last_name VARCHAR(50) NOT NULL,
    created_at TIMESTAMP DEFAULT now() NOT NULL,
    updated_at TIMESTAMP DEFAULT now() NOT NULL
);

-- 8. attendance (references players, matches)
CREATE TABLE attendance (
    id BIGSERIAL PRIMARY KEY,
    player_id BIGINT NOT NULL REFERENCES players(id) ON DELETE CASCADE,
    match_id BIGINT REFERENCES matches(id),
    attended BOOL DEFAULT false NOT NULL,
    timestamp TIMESTAMP DEFAULT now() NOT NULL,
    date DATE,
    UNIQUE (player_id, match_id)
);

-- 9. announcements (references users)
CREATE TABLE announcements (
    id BIGSERIAL PRIMARY KEY,
    title VARCHAR(200) NOT NULL,
    content TEXT NOT NULL,
    external_link VARCHAR(255),
    author_id BIGINT NOT NULL REFERENCES users(id),
    created_at TIMESTAMP DEFAULT now() NOT NULL,
    status VARCHAR(20) DEFAULT 'pending' NOT NULL
);
