
rustup default nightly
cargo clean
cargo build --release --features="c-interface"
ls -lht target/release/libsysinfo.so
strip -g target/release/libsysinfo.so
ls -lht target/release/libsysinfo.so
nm -g target/release/libsysinfo.so|less
