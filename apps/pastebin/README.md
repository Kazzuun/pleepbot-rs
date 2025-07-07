# Pastebin

A simple pastbin implementation.

## Endpoints

- [ ] POST /pastes
  - Creates a new paste entry
  - Request body:
    - content
    - language: (code, markdown, ...)
    - expires at

- [ ] GET /pastes/:slug
  - Fetches the content and formats it

- [ ] GET /pastes/:slug/raw
  - Fetches the raw content

## Possibilities

- Authentication and user accounts
