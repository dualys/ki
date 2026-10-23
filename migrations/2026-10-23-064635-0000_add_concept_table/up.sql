-- Your SQL goes here
CREATE TABLE IF NOT EXISTS concepts (
    id SERIAL PRIMARY KEY,
    english_word TEXT NOT NULL UNIQUE,
    implemented BOOLEAN DEFAULT FALSE,
    meikyo_root TEXT
);
