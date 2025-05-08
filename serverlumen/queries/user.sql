--: User(additional_public_keys?)
--- Represents a user record with their encryption keys and metadata

--! add_user (ehash, primary_public_key, additional_public_keys?) : User
--- Creates a new user with the given encryption hash and public keys
--- Parameters:
---   ehash: 32-byte hash
---   primary_public_key: 32-byte primary public key
---   additional_public_keys: Optional array of additional 32-bit public keys
INSERT INTO "user" (
    ehash,
    primary_public_key,
    additional_public_keys
)
VALUES (
    :ehash,
    :primary_public_key,
    COALESCE(:additional_public_keys, array[]::BYTEA[])
)
RETURNING
    id,
    ehash,
    primary_public_key,
    additional_public_keys,
    created_at;

--! get_user (id) : User
--- Retrieves a user by their UUID
--- Parameters:
---   id: The UUID of the user to retrieve
SELECT
    id,
    ehash,
    primary_public_key,
    additional_public_keys,
    created_at
FROM "user"
WHERE id = :id;
