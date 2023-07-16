
rustup default nightly-2017-06-30
cargo clean
cargo build --release --features="c-interface"
ls -lht target/release/libsysinfo.dylib
nm -g target/release/libsysinfo.dylib|less
