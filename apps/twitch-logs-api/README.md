# Message logger

An api that connects to twitch chats through IRC anonymously and logs messages. From the messages, emote counts are calculated for twitch, bttv, ffz, and 7tv emotes.

## Endpoints

- [ ] GET /channels
  - Gets all the channels this api is logging

- [ ] POST /channels
  - Adds a new channel to be logged
  - Authentication

- [ ] POST /channels/delete
  - Removes a channel from being logged
  - Authentication

- [ ] GET /emotes/:channel
  - Gets the emote usage counts in the channel by the emote provider
  - Channel id or name
  - Query params:
    - provider: twitch, bttv/betterttv, ffz/frankerfacez, 7tv/seventv

- [ ] GET /logs
  - Gets message logs
  - Query params:
    - channel: User id or name
    - user: Channel id or name
    - before: timestamp
    - after: timestamp
    - limit
    - order: newest, oldest (desc, asc)
    - contains
