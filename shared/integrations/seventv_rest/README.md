# /tv V3 REST API wrapper

## Endpoints

Docs: <https://api.7tv.app/v3/docs>

Using the base url <https://api.7tv.app/v3> the following endpoints are implemented:

- GET     /auth (?)
- POST    /auth/logout (?)
- GET     /auth/manual (?)

- GET     /chatterino/version/{os}/{branch} (?)

- GET     /emote-sets/{id}

- POST    /emotes
- GET     /emotes/{id}

- GET     /users/{id}
- DELETE  /users/{id} (?)
- PATCH   /users/{id}/connections/{connection_id}
- POST    /users/{id}/presences
- PUT     /users/{id}/profile-picture
- GET     /users/{platform}/{platform_id}
