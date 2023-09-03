# Ticker Bot

## shuttle

run `cargo install cargo-shuttle` to install [shuttle](https://docs.shuttle.rs/).

## user config

modify line 33 in `src/main.rs` to follow smth else than "bitcoin".

## environment variables

rename `Secrets.example.toml` to `Secrets.toml` and fill your `DISCORD_TOKEN`

## run locally

`cargo shuttle run`

## free hosting

run `cargo shuttle deploy` : you'll be prompted to login using discord to create an account on shuttle.
