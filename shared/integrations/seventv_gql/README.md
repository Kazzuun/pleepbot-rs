# 7tv API wrapper

Asynchronous API wrapper for 7tv GraphQL API.

Uses the new ULID format for all the ids instead of the old MongoDB Object ID.

Schema is fetched using `cynic-cli`.
```sh
cynic introspect https://api.7tv.app/v4/gql -o schemas/seventv.graphql
```

Correctness of the implementation of the schema is checked at build time by cynic.

## Endpoints

7tv GQL V4 endpoint

Only includes endpoints that can be used unauthenticated or with a token from a regular user. Endpoints for moderators/admins are not implemented.

Docs: <https://7tv.io/v4/gql/playground>

The following endpoints are implemented:

- Query
  - badges
    - badges
  - emotes
    - emote
      - channels
      - events
      - inEmoteSets
    - search
  - emoteSets
    - emoteSet
    - emoteSets
  - paints
    - paints
  - products
    - subscriptionProducts
    - subscriptionProduct
  - roles
    - roles
  - store
    - monthlyPaints
  - users
    - me
    - user
    - userByConnection
    - search

- Mutation
  - emotes
    - emote
      - name
      - flags
      - tags
      - delete
    - emotes
      - name
      - flags
      - tags
      - delete
  - emoteSets
    - emoteSet
      - name
      - tags
      - capacity
      - addEmote
      - removeEmote
      - updateEmoteAlias
      - updateEmoteFlags
      - delete
    - create
  - users
    - user
      - mainConnection
      - activateEmoteSet
      - activateBadge
      - removeProfilePicture
      - removeConnection
  - userEditors
    - editor
      - delete
      - updateState
      - updatePermissions
    - create
  - billing (?)
    - subscribe
    - cancelSubscription
    - reactivateSubscription
    - redeemCode
