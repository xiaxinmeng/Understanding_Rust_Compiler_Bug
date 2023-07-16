
rustup toolchain install nightly-2022-11-22
rustup default nightly-2022-11-22-x86_64-unknown-linux-gnu
cargo xtask build-ebpf --release
