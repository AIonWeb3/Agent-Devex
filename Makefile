.PHONY: all check test fmt lint build install clean

all: check test fmt lint build

check:
	cargo check --all-targets

test:
	cargo test

fmt:
	cargo fmt --all

lint:
	cargo clippy --all-targets -- -D warnings

build:
	cargo build --release

install:
	cargo install --path .

clean:
	cargo clean
