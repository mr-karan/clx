default: build

build:
    cargo build

release:
    cargo build --release

run *args:
    cargo run -- {{args}}

check:
    cargo check

test:
    cargo test

clean:
    cargo clean

install:
    cargo install --path .

fmt:
    cargo fmt

lint:
    cargo clippy -- -D warnings
