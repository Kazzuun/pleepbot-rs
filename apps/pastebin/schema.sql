CREATE SCHEMA pastbin;


CREATE TABLE pastbin.pastes (
    id SERIAL PRIMARY KEY,
    content TEXT NOT NULL,
    language TEXT DEFAULT NULL,
    expires_at TIMESTAMPTZ DEFAULT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
