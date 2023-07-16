
$ RUSTFLAGS="-C linker_plugin_lto -C linker=clang -Clink-arg=-fuse-ld=lld" cargo +rust-stage1 build --release
