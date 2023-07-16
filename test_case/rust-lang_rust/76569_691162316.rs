console
$ cargo new --bin empty
$ cd empty
$ env \
  RUSTFLAGS=-Zmir-opt-level=2\
  RUSTC_LOG=rustc_mir::transform::copy_prop=info\
  cargo +stage1 -Zbuild-std build --target x86_64-unknown-linux-gnu
