ARCH := aarch64-unknown-linux-gnu
LEPTOS_OUTPUT_NAME := dora

.PHONY: all
all: build test

.PHONY: setup
setup:
ifeq (,$(shell which rustc))
	$(error "Could not found Rust compiler, please install it")
endif
ifeq (,$(shell which cargo))
	$(error "Could not found Cargo, please install it")
endif
	cargo install cargo-leptos@0.2.20 --locked
	cargo install leptosfmt@0.1.30 --locked
	rustup target add wasm32-unknown-unknown
	cargo install cargo-lambda@1.4.0 --locked

.PHONY: test
test:
	cargo test --lib --bins
	cargo test --features ssr --lib --bins

.PHONY: build_dev
build_dev: setup
	npm install
	cargo leptos build
	LEPTOS_OUTPUT_NAME=$(LEPTOS_OUTPUT_NAME) cargo lambda build --no-default-features --features=ssr --target $(ARCH)

.PHONY: build_release
build_release:
	npm install
	cargo leptos build --release
	LEPTOS_OUTPUT_NAME=$(LEPTOS_OUTPUT_NAME) cargo lambda build --no-default-features --features=ssr --release --target $(ARCH)

.PHONY: build
build: build_dev

.PHONY: format
format:
	leptosfmt 'src/**/*.rs'
	cargo fmt
	cargo clippy --all-features

.PHONY: run
run:
	cargo leptos watch