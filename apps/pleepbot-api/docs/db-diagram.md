# Database diagram

```mermaid
erDiagram

  USERS {
    int id PK
    text user_id UK "twitch id"
    text username UK "current twitch username"
  }

  USER_PERMISSIONS 0+--1 USERS : has
  USER_PERMISSIONS {
    int id PK
    int user_id FK
    PERMISSION permission
    bool negated
  }

  JOINED_CHANNELS 1--1 USERS : is
  JOINED_CHANNELS {
    int id PK
    int channel_id FK
    bool currently_live
    bool active
    timestamptz joined_at
  }

  CHANNEL_BANNED_USERS 0+--1 JOINED_CHANNELS : "is banned in"
  CHANNEL_BANNED_USERS 0+--1 USERS : "is banned"
  CHANNEL_BANNED_USERS {
    int id PK
    int channel_id FK
    int user_id FK
    text reason "optional"
    timestamptz banned_at
  }

  AUTHORIZED_SEVENTV_EDITORS 0+--1 USERS : "can be"
  AUTHORIZED_SEVENTV_EDITORS 0+--1 JOINED_CHANNELS : has
  AUTHORIZED_SEVENTV_EDITORS {
    int id PK
    int user_id FK
    int channel_id FK
    int permissions
    timestamptz granted_at
  }

  REMINDERS 0+--1 USERS : "has sent"
  REMINDERS 0+--1 USERS : "is target of"
  REMINDERS 0+--1 JOINED_CHANNELS : "was set in"
  REMINDERS {
    int id PK
    text message
    text channel_id FK "channel created in"
    text sender_id FK
    text target_id FK
    bool cancelled
    bool isolated "only sent in the same channel"
    timestamptz created_at
    timestamptz processed_at
  }

  CHANNEL_CONFIG 1--1 JOINED_CHANNELS : has
  CHANNEL_CONFIG {
    int id PK
    text channel_id FK
    bool emote_streaks
    bool commands_online
    bool reminds_online
    bool notifications_online
    bool isolated
    text[] prefixes
  }

  CHANNEL_EMOTE_RULES 0+--1 JOINED_CHANNELS : has
  CHANNEL_EMOTE_RULES 0+--1 EMOTE_RULES : has
  CHANNEL_EMOTE_RULES {
    int id PK
    int emote_set_id FK "emote set where action happens"
    int emote_rule_id FK
  }

  EMOTE_RULES {
    int id PK
    text emote_set_id FK "emote set for events"
    CONDITION_CONNECTIVE condition_connective "ANY, ALL"
  }

  EMOTE_RULE_CONDITIONS 1+--1 EMOTE_RULES : "depends on"
  EMOTE_RULE_CONDITIONS {
    int id PK
    int emote_rule_id FK
    CONDITION condition "emote ADD, RENAME, REMOVE"
    bool negated
    text name_match "optional"
    text id_match "optional"
  }

  EMOTE_RULE_ACTIONS 1+--1 EMOTE_RULES : executes
  EMOTE_RULE_ACTIONS {
    int id PK
    int emote_rule_id FK
    ACTION_TYPE action_type "ADD, RENAME, REMOVE"
  }
  EMOTE_RULE_RENAME_ACTIONS |o--1 EMOTE_RULE_ACTIONS : executes
  EMOTE_RULE_RENAME_ACTIONS {
    int id PK
    int emote_rule_action_id FK
    text selection_rule
    text replacement_rule
  }

```

## Emote rules

### Actions

- Add emote: by id
- Rename emotes: emotes matching [regex] renamed to [regex]
- Remove emote: by id or matching [regex]

### Examples

#### 1. Emote set synchronization
Condition: a certain user adds an emote in any channel the bot has eventsub on
Action: add the added emote to a certain emote set(s) by emote set id

#### 2. Emote name standardation
Condition: an emote is added to a specific channel
Action: rename it based on regex rule

#### 3. Emote blocking
Condition: an emote is added to a specific channel
Action: remove it


- channel has a rule
- rule can be subscribed to events for a certain channel
-
