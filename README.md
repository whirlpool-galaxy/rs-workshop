# rs-workshop

Workshop participant and room scheduler. Designed for the Ophase at the Department of Computer Science of TU Darmstadt.

## Development

- Install [Docker](https://docs.docker.com/engine/install/)
- Install [Rust](https://www.rust-lang.org/tools/install)
- Install sqlx `cargo install sqlx-cli`
- Start DB server `docker compose -f docker-compose.test.yml up -d`
- Initialise DB `sqlx database setup`