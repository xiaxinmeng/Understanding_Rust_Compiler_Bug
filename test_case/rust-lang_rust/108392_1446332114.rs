
cargo-zigbuild build \
    --release --target armv7-unknown-linux-gnueabihf \
    --config='profile.release.split-debuginfo="packed"' \
    --config=profile.release.debug=2 \
    --features static,rustls,trust-dns,fancy-no-backtrace,zstd-thin,log_release_max_level_debug
