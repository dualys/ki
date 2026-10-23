-- Your SQL goes here
-- Colonnes phonétiques
ALTER TABLE words ADD COLUMN IF NOT EXISTS ipa TEXT;
ALTER TABLE words ADD COLUMN IF NOT EXISTS ipa_reversed TEXT;

-- Colonnes linguistiques
ALTER TABLE words ADD COLUMN IF NOT EXISTS element TEXT;
ALTER TABLE words ADD COLUMN IF NOT EXISTS part_of_speech TEXT;
ALTER TABLE words ADD COLUMN IF NOT EXISTS polarity TEXT;

-- Colonnes optionnelles (si tu veux les préparer)
ALTER TABLE words ADD COLUMN IF NOT EXISTS frequency INTEGER;
ALTER TABLE words ADD COLUMN IF NOT EXISTS grapheme TEXT;
