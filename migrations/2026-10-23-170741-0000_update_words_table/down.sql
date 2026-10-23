-- This file should undo anything in `up.sql`
-- Colonnes phonétiques
ALTER TABLE words DROP COLUMN IF EXISTS ipa;
ALTER TABLE words DROP COLUMN IF EXISTS ipa_reversed;

-- Colonnes linguistiques
ALTER TABLE words DROP COLUMN IF EXISTS element;
ALTER TABLE words DROP COLUMN IF EXISTS part_of_speech;
ALTER TABLE words DROP COLUMN IF EXISTS polarity;

-- Colonnes optionnelles (si tu veux les préparer)
ALTER TABLE words DROP COLUMN IF EXISTS frequency;
ALTER TABLE words DROP COLUMN IF EXISTS grapheme;
