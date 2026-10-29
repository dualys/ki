-- This file should undo anything in `up.sql`
ALTER TABLE words DROP COLUMN IF EXISTS definition_fr;
