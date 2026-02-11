default:
    just --list

check: cargo-check test clippy fmt update outdated

cargo-check:
    cargo check

test:
    cargo test

clippy:
    cargo clippy -- -D warnings

fmt:
    cargo fmt

update:
    cargo update

outdated:
    cargo outdated -R

dump:
    cargo run -- --dump
