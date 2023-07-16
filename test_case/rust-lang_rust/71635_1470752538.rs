
/home/gh-estebank/rust/build/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-e4ea1f0b9159d6a6.so(+0x99a323)[0x7f8b380a5323]
/lib/x86_64-linux-gnu/libc.so.6(+0x42520)[0x7f8b37342520]
/home/gh-estebank/foo/target/debug/deps/libwat_derive-f14316905b1905db.so(+0xe61e)[0x7f8b2c16f61e]
/home/gh-estebank/foo/target/debug/deps/libwat_derive-f14316905b1905db.so(+0xe623)[0x7f8b2c16f623]
/home/gh-estebank/foo/target/debug/deps/libwat_derive-f14316905b1905db.so(+0xe623)[0x7f8b2c16f623]
/home/gh-estebank/foo/target/debug/deps/libwat_derive-f14316905b1905db.so(+0xe623)[0x7f8b2c16f623]
8< 8< 8< 8< 8< 8< 8< 8< 8< 8< 8< 8< 8< 8< 8< 8< 8< 8< 8< 8< 8< 8< 8< 8< 8<
/home/gh-estebank/foo/target/debug/deps/libwat_derive-f14316905b1905db.so(+0xe623)[0x7f8b2c16f623]
/home/gh-estebank/foo/target/debug/deps/libwat_derive-f14316905b1905db.so(+0xe623)[0x7f8b2c16f623]
/home/gh-estebank/foo/target/debug/deps/libwat_derive-f14316905b1905db.so(+0xe623)[0x7f8b2c16f623]
/home/gh-estebank/foo/target/debug/deps/libwat_derive-f14316905b1905db.so(+0xe623)[0x7f8b2c16f623]
error: could not compile `foo` (bin "foo")

Caused by:
  process didn't exit successfully: `rustc --crate-name foo --edition=2021 src/main.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=293 --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 -C metadata=a47245182c2adce6 -C extra-filename=-a47245182c2adce6 --out-dir /home/gh-estebank/foo/target/debug/deps -C incremental=/home/gh-estebank/foo/target/debug/incremental -L dependency=/home/gh-estebank/foo/target/debug/deps --extern wat_derive=/home/gh-estebank/foo/target/debug/deps/libwat_derive-f14316905b1905db.so` (signal: 11, SIGSEGV: invalid memory reference)
