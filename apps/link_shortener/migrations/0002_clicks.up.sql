-- Add up migration script here
CREATE TABLE clicks (
    id SERIAL PRIMARY KEY,
    link_id INTEGER NOT NULL,
    clicked_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (link_id) REFERENCES links (id)
);