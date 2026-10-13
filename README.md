# Rust GraphQL Service



## Getting started!

- Run the migrations: `cargo run --manifest-path migrations/Cargo.toml -- up`
- Start the service: `cargo run`
- Navigate to: `http://localhost:8301/playground`

## Commands

- Build: `cargo build`
- Run: `cargo run`
- Format: `cargo fmt --all`

## Config

The service can be configured using environment parameters or the .env file.

| Parameter              | Description                           | Defaults to                |
| ---------------------- | ------------------------------------- | -------------------------- |
| URL                    | HTTP URL where the service is exposed | `http://localhost:8301`    |
| GRAPHQL_ENDPOINT       | GraphQL endpoint                      | `/graphql`                 |
| PLAYGROUND_ENDPOINT    | GraphQL UI                            | `/playground`              |
| DATABASE_URL           | Database connection                   | `sqlite://app.db?mode=rwc` |
| GRAPH_DEPTH_LIMIT      | Maximum depth of a query              | 16                          |
| GRAPH_COMPLEXITY_LIMIT | Maximum complexity of a query         | 4096                       |
| WWWROOT_PATH           | Filesystem location for serving files | `./wwwroot`                |

## Migrations

- Up: `cargo run --manifest-path migrations/Cargo.toml -- up`
- Down: `cargo run --manifest-path migrations/Cargo.toml -- down`
- Check: `cargo check --manifest-path migrations/Cargo.toml`

> 📝 for a custom database connection set the env param `DATABASE_URL`

## Tests

- Service: `cargo test --manifest-path ./Cargo.toml`
- Migrations: `cargo test --manifest-path ./migrations/Cargo.toml`
- All: `cargo test --workspace` 

## References

- https://github.com/SeaQL/seaography
- https://github.com/SeaQL/sea-orm
- https://www.sea-ql.org/SeaORM/
