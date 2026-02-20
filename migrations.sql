-- Roles table (must be created before users)
CREATE TABLE IF NOT EXISTS roles (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE
);

-- Users table
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(100) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    name VARCHAR(100) NOT NULL,
    role_id INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    FOREIGN KEY (role_id) REFERENCES roles(id)
);

-- Announcements table
CREATE TABLE IF NOT EXISTS announcements (
    id SERIAL PRIMARY KEY,
    title VARCHAR(200) NOT NULL,
    content TEXT NOT NULL,
    external_link VARCHAR(255),
    author_id INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    FOREIGN KEY (author_id) REFERENCES users(id)
);

-- Seasons table (must be before matches)
CREATE TABLE IF NOT EXISTS seasons (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL
);

-- Tournaments table
CREATE TABLE IF NOT EXISTS tournaments (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    season_id INTEGER NOT NULL,
    FOREIGN KEY (season_id) REFERENCES seasons(id)
);

-- Matches table
CREATE TABLE IF NOT EXISTS matches (
    id SERIAL PRIMARY KEY,
    date DATE NOT NULL,
    opponent VARCHAR(100) NOT NULL,
    venue VARCHAR(100) NOT NULL,
    result VARCHAR(50),
    score VARCHAR(20),
    match_link VARCHAR(255),
    season_id INTEGER NOT NULL,
    tournament_id INTEGER,
    FOREIGN KEY (season_id) REFERENCES seasons(id),
    FOREIGN KEY (tournament_id) REFERENCES tournaments(id)
);

-- Attendance table with unique constraint for upsert support
CREATE TABLE IF NOT EXISTS attendance (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    match_id INTEGER NOT NULL,
    present BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (match_id) REFERENCES matches(id),
    UNIQUE(user_id, match_id)
);
