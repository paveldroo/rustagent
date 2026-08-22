set dotenv-load := true

export PROJECT_NAME := "rustagent"

run:
    RUST_BACKTRACE=1 cargo run

run-cat:
    cat tests/fixtures/prompt.txt | cargo run

lint:
    cargo fmt
    cargo clippy --all-targets -- -D warnings

test:
    cargo test

release:
    cargo build --bin {{PROJECT_NAME}} --release
    ./target/release/{{PROJECT_NAME}}
