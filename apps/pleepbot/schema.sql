CREATE SCHEMA twitch;

CREATE SCHEMA timeseries;

CREATE TABLE twitch.users (
    user_id text PRIMARY KEY,
    username text NOT NULL,
);

-- This role permission thing is probably too complicated for this bot
CREATE TABLE twitch.roles (
    roles text PRIMARY KEY,
);


CREATE TABLE twitch.permissions (
    permission text PRIMARY KEY,
);


CREATE TABLE twitch.user_roles (
    user_id text PRIMARY KEY,
    role text PRIMARY KEY,
);


CREATE TABLE twitch.role_permissions (
    role text PRIMARY KEY,
    permission text PRIMARY KEY,
);


CREATE TABLE twitch.joined_channels (
    channel_id text PRIMARY KEY REFERENCES twitch.users(user_id),
    currently_online boolean DEFAULT false NOT NULL,
    active boolean NOT NULL DEFAULT TRUE,
    timestamp timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
);


CREATE TABLE twitch.channel_config (
    channel_id text PRIMARY KEY KEY REFERENCES twitch.joined_channels(channel_id),
    logging boolean DEFAULT TRUE NOT NULL,
    emote_streaks boolean DEFAULT FALSE NOT NULL,
    commands_online boolean DEFAULT TRUE NOT NULL,
    reminds_online boolean DEFAULT TRUE NOT NULL,
    notifications_online boolean DEFAULT FALSE NOT NULL,
    isolated boolean DEFAULT FALSE NOT NULL,
    disabled_commands text[] DEFAULT array[]::text[] NOT NULL,
    prefixes text[] DEFAULT array[]::text[] NOT NULL
);



CREATE TABLE twitch.channel_banned_users (
    channel_id text PRIMARY KEY REFERENCES twitch.joined_channels(channel_id),
    user_id text PRIMARY KEY REFERENCES twitch.users(user_id),
    reason text,
    timestamp timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
);


CREATE FUNCTION twitch.create_channel_config()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO twitch.channel_config (channel_id)
    VALUES (NEW.channel_id);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_create_channel_config
AFTER INSERT ON twitch.joined_channels
FOR EACH ROW
EXECUTE FUNCTION twitch.create_channel_config();


-- TODO: make a different table for non user messages?
CREATE TABLE twitch.messages (
    id serial PRIMARY KEY,
    sender_id text NOT NULL REFERENCES twitch.users(user_id),
    channel_id text NOT NULL REFERENCES twitch.users(user_id),
    message text NOT NULL,
    timestamp timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
    online boolean, -- TODO: NOT NULL?
)


CREATE TYPE twitch.system_message_type AS ENUM (
    'SUB',
    'BITS',
    'BAN',
    'TIMEOUT',
    'CHATMODE',
    -- ?????
);

CREATE TABLE twitch.system_messages (
    id serial PRIMARY KEY,
    sender_id text REFERENCES twitch.users(user_id),
    channel_id text NOT NULL REFERENCES twitch.users(user_id),
    message text NOT NULL,
    timestamp timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
    online boolean, -- TODO: NOT NULL?
    type twitch.system_message_type NOT NULL,
)


CREATE TABLE twitch.authorized_seventv_editor (
    channel_id text PRIMARY KEY REFERENCES twitch.users(user_id),
    user_id text PRIMARY KEY REFERENCES twitch.users(user_id),
    timestamp timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
);


CREATE TABLE twitch.custom_commands (
    id serial PRIMARY KEY,
    owner_id text REFERENCES twitch.users(user_id)
    name text NOT NULL,
    content text NOT NULL,
    updated_at timestamptz,
    timestamp timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
);


CREATE TABLE twitch.custom_command_user_links (
    custom_command_id integer PRIMARY KEY REFERENCES twitch.custom_commands(id),
    user_id text PRIMARY KEY REFERENCES twitch.users(user_id)
    alias text,
    timestamp timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
);


-- TODO: add permissions?
CREATE TABLE twitch.custom_command_channel_links (
    custom_command_id integer PRIMARY KEY REFERENCES twitch.custom_commands(id),
    channel_id text PRIMARY KEY REFERENCES twitch.users(user_id)
    alias text,
    timestamp timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
);


CREATE TABLE timeseries.username_history (
    id serial PRIMARY KEY,
    user_id text REFERENCES twitch.users(user_id),
    username text,
    timestamp timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
);


CREATE FUNCTION timeseries.update_username_history()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO timeseries.username_history (user_id, username)
    VALUES (NEW.user_id, NEW.username);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_username_history
AFTER INSERT OR UPDATE OF username ON twitch.users
FOR EACH ROW
EXECUTE FUNCTION timeseries.update_username_history();



CREATE TYPE timeseries.seventv_emote_action AS ENUM (
    'ADD',
    'REMOVE',
    'RENAME',
);

-- TODO: editor additions / emote set creations?
CREATE TABLE timeseries.seventv_emote_actions (
    id serial PRIMARY KEY,
    actor_seventv_id text,
    actor_twitch_id text REFERENCES twitch.users(user_id),
    emote_id text NOT NULL,
    emote_name text NOT NULL,
    emote_cdn_url text NOT NULL,
    timestamp timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
    action timeseries.seventv_emote_action NOT NULL,
)

CREATE TYPE timeseries.stream_event_type AS ENUM (
    'ONLINE',
    'OFFLINE',
    'RESTART',
);

CREATE TABLE timeseries.stream_events (
    id serial PRIMARY KEY,
    channel_id text NOT NULL REFERENCES twitch.users(user_id),
    title text NOT NULL,
    category text NOT NULL,
    timestamp timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
    event_type timeseries.stream_event_type NOT NULL,
)

CREATE TYPE timeseries.bot_join_event_type AS ENUM (
    'JOIN',
    'PART',
    -- TODO: reconnect?
);

CREATE TABLE timeseries.bot_join_events (
    id serial PRIMARY KEY,
    channel_id text NOT NULL REFERENCES twitch.users(user_id),
    actor_id text NOT NULL REFERENCES twitch.users(user_id),
    timestamp timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
    event_type timeseries.bot_join_event_type NOT NULL,
)
