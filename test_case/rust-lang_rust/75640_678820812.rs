shell
export \
  'CC=clang'\
  'CFLAGS=-fsanitize=memory -fsanitize-memory-track-origins'\
  'RUSTFLAGS=-Zsanitizer=memory -Zsanitizer-memory-track-origins'\
  'RUSTDOCFLAGS=-Zsanitizer=memory -Zsanitizer-memory-track-origins'
cargo +nightly -Zbuild-std run --target x86_64-unknown-linux-gnu
