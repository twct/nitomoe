CREATE TYPE user_role AS ENUM ('moderator', 'admin');

CREATE TABLE IF NOT EXISTS users (
    "id" BIGSERIAL NOT NULL PRIMARY KEY,
    "username" TEXT NOT NULL UNIQUE,
    "password_hash" TEXT NOT NULL,
    "role" user_role NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT NOW(),
    "updated_at" TIMESTAMP NOT NULL DEFAULT NOW()
);