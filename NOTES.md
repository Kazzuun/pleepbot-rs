Bot framework example: <https://valk.awoo.nl/Twitch/bot>

Api design example: <https://blog.0xshadow.dev/posts/backend-engineering-with-axum/axum-reset-password/>
  - this design pattern can also be used for an api wrapper
  - https://github.com/tokio-rs/axum/tree/main/examples/sqlx-postgres
  - https://github.com/nakamuraos/axum-postgres-boilerplate
  - https://www.reddit.com/r/rust/comments/1o4145i/how_do_you_review_your_code/
    - https://www.howtocodeit.com/articles/master-hexagonal-architecture-rust

## New features

- add boolean flags to commands for optoutable, disableable etc
- better permission management?
- positional and keyword arguments
- track every change in commands and allow making graphs of them
- pastebin usage (make own)
- link shortener (make own)
- 7tv eventapi usage
- discord admin logs and dashboard?
- api for some commands
- tests
- modules like cogs?
  - module wide enable/disable
  - module wide settings

- command configurations
  - like what emote to use for it
  - this should include disabled and stuff
  - custom aliases to commands for users and channels

- isolation but a group of channels instead of just one
- custom rules
  - replace word with some other word

- custom commands
  - for channel or person
    - can be linked like supibot
      - original changes -> everyone else's changes
  - channel can disable peronal commands

- command chain
  - allow conditional branching
  - possibly has context from previous command(s)
  - good for setting up things like module settings or more complex commands
  - ?next *args
  - different commands or the same command waits for the input?
  - confirmation command (a special command chain)
    - ?yes
    - ask a yes/no question to confirm and ?yes to accept
    - only one at a time
      - not answering will time it out and getting another question will override

- new fishing
  - seasons
    - after a season, convert some? all? progress into bonuses (perma / only for next season)
  - contests
    - admin can start some contest in some chats or globally
  - prestige

- add and yoink like potatbotat
  - temporarily add an emote for some time
  - queue an emote action

- a command to queue a command for later
  - add a config value to commands if this is allowed
    - don't allow this for fish for example

- reminder when when they type next but after certain time
  - set valid channels a reminder can be sent in
  - reoccur: <times> interval: <time>

- new fight command
  - changable fight mode to gain advantage

- howlongtobeat command
  - <https://github.com/Supinic/supibot/blob/master/commands/howlongtobeat/index.js>

- steam api command for game prices and maybe link to history?
  - <https://docs.isthereanydeal.com>

- message logger has a websocket to get emote streak data

- emote naming rule
  - rules set by the broadcaster
  - example: replace "^(bugcat)(\w*)" with "Bugcat\2"
  - checked on every emote action through the bot
    - should it also be done in all events, not just bot commands?
    - option between strict and only bot?
    - a sync command

- commands for managing roles with the bot
  - some commands need mod or broadcaster permissions -> allow giving users the permissions without them being one

- documentation for everything
  - openapi for apis
  - mermaid or dbml for databases (https://dbdiagram.io/d)
  - mermaid or something else for architecture
  - git-cliff for changelogs

- twitch api gateway servers
  - "https://twitchapi.teklynk.com", "https://twitchapi.teklynk.dev", "https://twitchapi2.teklynk.dev"

- https://www.cncf.io/ cloud stuff
