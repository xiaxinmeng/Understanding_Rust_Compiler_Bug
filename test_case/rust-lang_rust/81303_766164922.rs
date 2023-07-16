bash
cargo new test2
cd test2
echo 'serde_derive = "*"' >> Cargo.toml
RUSTFLAGS='-Z polonius=yes' cargo +nightly run
