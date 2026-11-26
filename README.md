# wow-api

[![CI](https://github.com/apollopower/wow-api/actions/workflows/ci.yml/badge.svg)](https://github.com/apollopower/wow-api/actions/workflows/ci.yml)

A small Rust HTTP API built on Dropshot. Its intended purpose is to serve a player's World of Warcraft game data from the Battle.net API.

## Prerequisites

A [Battle.net API Developer account](https://community.developer.battle.net/) is required for setting up your own client.

## Endpoints

| Method | Path |
|--------|------|
| GET | `/health` |
| GET | `/api/character/{realm}/{name}` |
| GET | `/api/item/{id}` |
| GET | `/api/realms` |

`GET /api/realms` is paginated. Pass `limit` to choose a page size and send the
previous response's `next_page` value as `page_token`. Tokens are opaque; do not
parse them. When `next_page` is `null`, the scan is complete.

## Run

Copy `.env.example` to `.env` and fill in your Battle.net client ID and secret:

```bash
cp .env.example .env
cargo run
```

The server listens on `127.0.0.1:8080` by default.

## Test

```bash
cargo test
```

Live Battle.net tests are ignored by default. Run them with:

```bash
cargo test --test live -- --ignored
```

## Configuration

| Variable | Default |
|----------|---------|
| `WOW_API_BIND` | `127.0.0.1:8080` |
| `BNET_CLIENT_ID` | — |
| `BNET_CLIENT_SECRET` | — |
| `BNET_REGION` | `us` |
| `BNET_LOCALE` | `en_US` |


