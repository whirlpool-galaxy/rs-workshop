# rs-workshop

<div align=center>

![GitHub](https://img.shields.io/github/license/whirlpool-galaxy/rs-workshop)
![GitHub release (with filter)](https://img.shields.io/github/v/release/whirlpool-galaxy/rs-workshop?sort=semver)
![GitHub issues](https://img.shields.io/github/issues/whirlpool-galaxy/rs-workshop)
[![Discord](https://img.shields.io/discord/1016731291267387544)](https://discord.gg/T2fDVrmGnF)

</div>

Workshop participant and room scheduler. Designed for the Ophase at the Department of Computer Science of TU Darmstadt.

## Development

- Install [Docker](https://docs.docker.com/engine/install/)
- Install [Rust](https://www.rust-lang.org/tools/install)
- Install sqlx `cargo install sqlx-cli`
- Start DB server `docker compose -f docker-compose.test.yml up -d`
- Initialise DB `sqlx database setup`