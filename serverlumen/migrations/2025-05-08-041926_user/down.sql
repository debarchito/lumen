-- This file should undo anything in `up.sql`
DROP TRIGGER IF EXISTS validate_users_additional_public_keys ON users;

DROP FUNCTION IF EXISTS check_users_additional_public_keys ();

DROP TABLE IF EXISTS "user";
