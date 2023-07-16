sh
#!/bin/sh
set -ex
rm -rf a b c
cargo new --lib a
cargo new --lib b
cargo new --bin c
echo 'pub trait A { fn a(&self) {} } impl A for () {}' > a/src/lib.rs
echo 'a = { path = "../a" }' >> b/Cargo.toml
echo 'pub extern crate a;' > b/src/lib.rs
echo 'b = { path = "../b" }' >> c/Cargo.toml
echo 'extern crate b; fn main() { ().a(); }' > c/src/main.rs
cargo build --manifest-path c/Cargo.toml
