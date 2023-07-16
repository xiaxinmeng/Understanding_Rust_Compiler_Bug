
#!/bin/sh

export RUSTFLAGS='-Z share-generics -C linker-plugin-lto  -C link-arg=-Wl,--icf=safe'

#--by-commit --start '7aa413d59'\

exec cargo bisect-rustc \
    --start '7aa413d59'\
    --target armv7-unknown-linux-gnueabihf --with-src --verbose -- \
    zigbuild --release --target armv7-unknown-linux-gnueabihf.2.17 \
    -Z build-std=std,panic_abort \
    -Z build-std-features=panic_immediate_abort \
    --config='profile.release.split-debuginfo="packed"' \
    --config=profile.release.debug=2 \
    --features static,rustls,trust-dns,fancy-no-backtrace,zstd-thin,log_release_max_level_debug,cross-lang-fat-lto
