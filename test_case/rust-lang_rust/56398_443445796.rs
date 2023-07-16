
cargo +beta new foo --lib
cd foo
cargo +beta new ignore --lib
echo 'ignore = {path = "ignore"}' >> Cargo.toml
echo 'use ignore;' >> src/lib.rs
cargo +beta check
