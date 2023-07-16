shell
$ rustup toolchain install --force nightly # requires nightly-2020-01-11-x86_64-unknown-linux-gnu or later
$ rustup component add rust-src
$ git clone https://github.com/BurntSushi/ripgrep
$ cd ripgrep
$ export \
  CC=clang \
  CXX=clang++ \
  CFLAGS=-fsanitize=memory \
  CXXFLAGS=-fsanitize=memory \
  RUSTFLAGS=-Zsanitizer=memory \
  RUSTDOCFLAGS=-Zsanitizer=memory
$ cargo +nightly test -Z build-std --workspace --exclude grep-pcre2 --target x86_64-unknown-linux-gnu

...

     Running target/x86_64-unknown-linux-gnu/debug/deps/globset-61555583ff5868c8

running 249 tests
test glob::tests::any1 ... ok
test glob::tests::cls10 ... ok
test glob::tests::cls1 ... ok
test glob::tests::any2 ... ok
test glob::tests::cls13 ... ok
test glob::tests::cls14 ... ok
test glob::tests::cls12 ... ok
test glob::tests::cls11 ... ok
test glob::tests::cls15 ... ok
test glob::tests::cls16 ... ok

...

   Doc-tests grep-cli

running 8 tests
test src/lib.rs -  (line 77) ... ignored
test src/lib.rs -  (line 85) ... ignored
test src/decompress.rs - decompress::DecompressionReader (line 291) ... ok
test src/process.rs - process::CommandReader (line 155) ... ok
test src/pattern.rs - pattern::patterns_from_reader (line 142) ... ok
test src/escape.rs - escape::unescape (line 86) ... ok
test src/lib.rs -  (line 91) ... ok
test src/escape.rs - escape::escape (line 35) ... ok

test result: ok. 6 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out

...
