bash
  export RUST_BACKTRACE=1
  export RUSTFLAGS='--deny warnings'
  export PKG_CONFIG_PATH="/usr/local/opt/zlib/lib/pkgconfig:$PKG_CONFIG_PATH"
  export RUSTC_WRAPPER=sccache
  cd webrender_api/
  cargo test --verbose --features "ipc"
