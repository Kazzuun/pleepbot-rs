# Link shortener

A basic implementation of a link shortener.

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

## Possibilities

- Authentication and user accounts
  - GET /links
  - DELETE /:slug
