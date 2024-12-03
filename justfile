#!/usr/bin/env just --justfile

dev:
    cargo watch -x 'run'

run:
    cargo run

build:
    cargo build

test:
    cargo test

check:
    cargo check

doc:
    cargo doc

lint:
    cargo clippy
