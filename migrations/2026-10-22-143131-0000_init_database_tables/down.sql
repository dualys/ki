-- This file should undo anything in `up.sql`
-- Table principale des mots
DROP TABLE IF EXISTS words;
-- Polarités détaillées de la racine (matrice générative)
DROP TABLE IF EXISTS root_polarities;
-- Contenus textuels (mythe, essence, définition...)
DROP TABLE IF EXISTS word_contents;
-- Complémentaires
DROP TABLE IF EXISTS complementaries;
-- Exemples de phrases
DROP TABLE IF EXISTS examples;
-- Correspondances cosmiques (optionnel)
DROP TABLE IF EXISTS correspondences;
-- Index utiles
DROP INDEX IF EXISTS idx_words_root;
DROP INDEX IF EXISTS idx_words_chakra;
DROP INDEX IF EXISTS idx_polarities_word;
