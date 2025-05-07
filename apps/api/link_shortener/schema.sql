CREATE SCHEMA link_shortener;

CREATE TABLE link_shortener.links (
    id SERIAL PRIMARY KEY,
    original_url TEXT NOT NULL,
    slug TEXT NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMPTZ DEFAULT NULL
);

CREATE TABLE link_shortener.clicks (
    id SERIAL PRIMARY KEY,
    link_id INTEGER NOT NULL,
    clicked_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (link_id) REFERENCES link_shortener.links (id)
);