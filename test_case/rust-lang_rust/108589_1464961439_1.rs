
git checkout 7f01ae877dfd26277a52630c9e40724db23ff8fc
env RUSTFLAGS="-C target-cpu=native" cargo install --path crates/rust-analyzer --locked --force --features force-always-assert --features jemalloc
