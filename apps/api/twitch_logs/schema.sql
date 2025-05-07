CREATE TABLE logged_channels (
    channel_id      text PRIMARY KEY,
    username        text NOT NULL,
    logging_since   timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE known_bots (
    user_id         text PRIMARY KEY,
    username        text NOT NULL
);

CREATE TABLE messages (
    id              serial PRIMARY KEY,
    msg_id          uuid NOT NULL,
    channel_id      text NOT NULL,
    sender_id       text NOT NULL,
    sender_name     text NOT NULL,
    message         text NOT NULL,
    sent_at         timestamptz NOT NULL,
    emote_data      jsonb -- emote data like name and the image url as an array of key value pairs
);

CREATE TYPE emote_type AS ENUM('TWITCH', 'BTTV', 'FFZ', '7TV');

CREATE TABLE emote_counts (
    emote_type      emote_type NOT NULL,
    channel_id      text,
    emote_id        text,
    emote_name      text NOT NULL,
    image_url       text NOT NULL,
    count           int NOT NULL DEFAULT 0,
    PRIMARY KEY (channel_id, emote_id)
);
