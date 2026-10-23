-- This file should undo anything in `up.sql`
-- Supprimer d'abord les tables dépendantes
DROP TABLE IF EXISTS correspondences;
DROP TABLE IF EXISTS examples;
DROP TABLE IF EXISTS complementaries;
DROP TABLE IF EXISTS word_contents;
DROP TABLE IF EXISTS root_polarities;

-- Puis la table principale
DROP TABLE IF EXISTS words;

-- Index (optionnel, ils sont déjà droppés avec les tables)
DROP INDEX IF EXISTS idx_words_root;
DROP INDEX IF EXISTS idx_words_chakra;
DROP INDEX IF EXISTS idx_polarities_word;
