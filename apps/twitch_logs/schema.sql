CREATE SCHEMA logger;

CREATE TABLE logger.logged_channels (
    channel_id      text PRIMARY KEY,
    username        text NOT NULL,
    logging_since   timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE logger.known_bots (
    user_id         text PRIMARY KEY REFERENCES logger.users(channel_id),
);

CREATE TABLE logger.users (
    user_id         text PRIMARY KEY,
    username        text NOT NULL,
    display_name    text NOT NULL,
)

CREATE TABLE logger.username_history (
    id serial       PRIMARY KEY,
    user_id         text NOT NULL REFERENCES twitch.users(user_id),
    username        text NOT NULL,
    display_name    text NOT NULL,
    timestamp       timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP NOT NULL,
);

CREATE FUNCTION logger.update_username_history()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO logger.username_history (user_id, username, display_name)
    VALUES (NEW.user_id, NEW.username, NEW.display_name);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER logger.trigger_update_username_history
AFTER INSERT OR UPDATE OF username, display_name ON twitch.users
FOR EACH ROW
EXECUTE FUNCTION logger.update_username_history();

CREATE TABLE logger.messages (
    msg_id          uuid NOT NULL PRIMARY KEY,
    channel_id      text NOT NULL REFERENCES logger.logged_channels(channel_id),
    sender_id       text NOT NULL REFERENCES logger.users(user_id),
    message         text NOT NULL,
    sent_at         timestamptz NOT NULL,
);

CREATE TYPE logger.emote_type AS ENUM('TWITCH', 'BTTV', 'FFZ', '7TV');

CREATE TABLE logger.emote_data (
    id              serial PRIMARY KEY,
    emote_type      logger.emote_type NOT NULL,
    emote_id        text NOT NULL UNIQUE,
    emote_name      text NOT NULL
);

CREATE TABLE logger.message_emotes (
    msg_id          uuid NOT NULL REFERENCES logger.messages(msg_id),
    emote_id        integer NOT NULL REFERENCES logger.emote_data(id),
    count           integer NOT NULL DEFAULT 1,
    PRIMARY KEY (msg_id, emote_id)
);
