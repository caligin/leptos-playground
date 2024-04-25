ARCH := aarch64-unknown-linux-gnu
LEPTOS_OUTPUT_NAME := entirely-ssr-fns

.PHONY: build test lint run

all: build test

build:
	cargo leptos build --release
	LEPTOS_OUTPUT_NAME=$(LEPTOS_OUTPUT_NAME) cargo lambda build --no-default-features --features=ssr --release --target $(ARCH)

test:
	cargo test --lib --bins

lint:
	leptosfmt **/*.rs
	cargo fmt
	cargo clippy --all-features

run:
	cargo leptos watch