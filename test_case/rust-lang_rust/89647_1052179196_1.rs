
$ RUSTFLAGS="-Z new-llvm-pass-manager=no" \
    cargo +nightly build --release --verbose -j1 --target=wasm32-unknown-unknown
