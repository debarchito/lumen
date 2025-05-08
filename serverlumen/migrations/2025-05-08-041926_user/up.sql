-- Your SQL goes here
CREATE TABLE "user" (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  ehash BYTEA NOT NULL UNIQUE CHECK (octet_length(ehash) = 32),
  primary_public_key BYTEA NOT NULL CHECK (octet_length(primary_public_key) = 32),
  additional_public_keys BYTEA[] NOT NULL DEFAULT array[]::BYTEA[],
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE OR REPLACE FUNCTION check_additional_public_keys()
RETURNS trigger AS $$
BEGIN
  IF EXISTS (
    SELECT 1 FROM unnest(NEW.additional_public_keys) AS p
    WHERE octet_length(p) != 32
  ) THEN
    RAISE EXCEPTION 'each additional public key must be exactly 32 bytes';
  END IF;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER validate_additional_public_keys
BEFORE INSERT OR UPDATE ON "user"
FOR EACH ROW EXECUTE FUNCTION check_additional_public_keys();
