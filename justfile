alias d := dev
alias b := build

dev:
    cargo check
    cargo clippy
    cargo fmt
build: dev
    cargo build
release: dev
    cargo build --release
