# wow-api

A small Rust HTTP API built on Dropshot. Its intended purpose is to serve a player's World of Warcraft game data from the Battle.net API.

## Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/health` | TODO |
| GET | `/api/character/{realm}/{name}` | TODO |

## Run

```bash
cargo run
```

The server listens on `127.0.0.1:8080` by default.

## Test

```bash
cargo test
```

## Configuration

| Variable | Default | Description |
|----------|---------|-------------|
| `WOW_API_BIND` | `127.0.0.1:8080` | TODO |
| `BNET_CLIENT_ID` | — | TODO (planned) |
| `BNET_CLIENT_SECRET` | — | TODO (planned) |


