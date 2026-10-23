-- Your SQL goes here
-- Table principale des mots
CREATE TABLE IF NOT EXISTS words (
    id SERIAL PRIMARY KEY,
    word TEXT NOT NULL UNIQUE,
    ipa TEXT,
    root TEXT NOT NULL,
    chakra TEXT,
    element TEXT,
    frequency INTEGER,
    grapheme TEXT,
    polarity TEXT,
    part_of_speech TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Polarités détaillées de la racine
CREATE TABLE IF NOT EXISTS root_polarities (
    id SERIAL PRIMARY KEY,
    word_id INTEGER NOT NULL REFERENCES words(id) ON DELETE CASCADE,
    position INTEGER NOT NULL,
    consonant TEXT NOT NULL,
    agency TEXT,
    realm TEXT,
    element TEXT,
    quality TEXT
);

-- Contenus textuels
CREATE TABLE IF NOT EXISTS word_contents (
    id SERIAL PRIMARY KEY,
    word_id INTEGER NOT NULL REFERENCES words(id) ON DELETE CASCADE,
    section TEXT NOT NULL,
    content TEXT NOT NULL,
    language TEXT DEFAULT 'fr'
);

-- Complémentaires
CREATE TABLE IF NOT EXISTS complementaries (
    id SERIAL PRIMARY KEY,
    word_id INTEGER NOT NULL REFERENCES words(id) ON DELETE CASCADE,
    complementary_word TEXT NOT NULL,
    complementary_ipa TEXT,
    note TEXT
);

-- Exemples de phrases
CREATE TABLE IF NOT EXISTS examples (
    id SERIAL PRIMARY KEY,
    word_id INTEGER NOT NULL REFERENCES words(id) ON DELETE CASCADE,
    register TEXT NOT NULL,
    meikyo_sentence TEXT NOT NULL,
    translation TEXT
);

-- Correspondances cosmiques
CREATE TABLE IF NOT EXISTS correspondences (
    id SERIAL PRIMARY KEY,
    word_id INTEGER NOT NULL REFERENCES words(id) ON DELETE CASCADE,
    type TEXT NOT NULL,
    name TEXT NOT NULL,
    rationale TEXT
);

-- Index
CREATE INDEX IF NOT EXISTS idx_words_root ON words(root);
CREATE INDEX IF NOT EXISTS idx_words_chakra ON words(chakra);
CREATE INDEX IF NOT EXISTS idx_polarities_word ON root_polarities(word_id);
