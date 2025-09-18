# Media api

Combines pastebin, file host and link shortener apis into one.

## Endpoints

- [ ] POST /shorten
  - Shortens the link
  - Request body:
    - url
    - expiration

- [ ] GET /:slug
  - Get redirected to the shortened link
  - Collect data like click count

- [ ] GET /:slug/info
  - Returns metadata of the link
  - Returns original link, created at, expires at, clicks

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
  - GET /links
  - DELETE /:slug
