bash
cargo new --bin rust-foo && cd rust-foo
cargo build
touch src/main.rs && time cargo +stable build
touch src/main.rs && time cargo +nightly build
cd .. && rm -rf rust-foo
