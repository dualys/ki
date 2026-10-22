-- Your SQL goes here
-- Table principale des mots
CREATE TABLE IF NOT EXISTS words (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    word TEXT NOT NULL UNIQUE,              -- ex: Tāḥem
    ipa TEXT,                               -- ex: taː.ħɛm
    root TEXT NOT NULL,                     -- ex: T-Ḥ-M
    chakra TEXT,                            -- ex: Racine
    element TEXT,                           -- ex: Terre
    frequency INTEGER,                      -- ex: 396
    grapheme TEXT,                          -- ex: ◯
    polarity TEXT,                          -- Actif / Complémentaire
    part_of_speech TEXT,                    -- n., v., etc.
    created_at TEXT DEFAULT CURRENT_TIMESTAMP
);

-- Polarités détaillées de la racine (matrice générative)
CREATE TABLE IF NOT EXISTS root_polarities (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    word_id INTEGER NOT NULL,
    position INTEGER NOT NULL,              -- 1, 2 ou 3 (C1, C2, C3)
    consonant TEXT NOT NULL,
    agency TEXT,                            -- Actif / Passif
    realm TEXT,                             -- Céleste / Terrestre
    element TEXT,                           -- Terre, Eau, Feu, Air...
    quality TEXT,                           -- Force / Grâce
    FOREIGN KEY (word_id) REFERENCES words(id) ON DELETE CASCADE
);

-- Contenus textuels (mythe, essence, définition...)
CREATE TABLE IF NOT EXISTS word_contents (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    word_id INTEGER NOT NULL,
    section TEXT NOT NULL,                  -- Origine, Essence, Signification, Invocation...
    content TEXT NOT NULL,
    language TEXT DEFAULT 'fr',             -- fr / en
    FOREIGN KEY (word_id) REFERENCES words(id) ON DELETE CASCADE
);

-- Complémentaires
CREATE TABLE IF NOT EXISTS complementaries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    word_id INTEGER NOT NULL,
    complementary_word TEXT NOT NULL,
    complementary_ipa TEXT,
    note TEXT,
    FOREIGN KEY (word_id) REFERENCES words(id) ON DELETE CASCADE
);

-- Exemples de phrases
CREATE TABLE IF NOT EXISTS examples (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    word_id INTEGER NOT NULL,
    register TEXT NOT NULL,                 -- Descriptif / Rituel
    meikyo_sentence TEXT NOT NULL,
    translation TEXT,
    FOREIGN KEY (word_id) REFERENCES words(id) ON DELETE CASCADE
);

-- Correspondances cosmiques (optionnel)
CREATE TABLE IF NOT EXISTS correspondences (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    word_id INTEGER NOT NULL,
    type TEXT NOT NULL,                     -- plante / mineral / constellation
    name TEXT NOT NULL,
    rationale TEXT,
    FOREIGN KEY (word_id) REFERENCES words(id) ON DELETE CASCADE
);

-- Index utiles
CREATE INDEX IF NOT EXISTS idx_words_root ON words(root);
CREATE INDEX IF NOT EXISTS idx_words_chakra ON words(chakra);
CREATE INDEX IF NOT EXISTS idx_polarities_word ON root_polarities(word_id);
