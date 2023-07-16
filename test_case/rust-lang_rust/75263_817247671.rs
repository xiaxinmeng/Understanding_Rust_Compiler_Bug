
cargo new hello-world
cd hello-world
echo '[profile.release]
panic = "abort"' >> Cargo.toml
RUSTFLAGS="--remap-path-prefix=/home/joshua/test-rustdoc/hello-world=src --remap-path-prefix=/home/joshua/.local/lib/cargo=cargo --remap-path-prefix=/home/joshua/.local/lib/rustup=rustup" cargo run --release
strings /home/joshua/.local/lib/cargo/target/release/hello-world | grep home
