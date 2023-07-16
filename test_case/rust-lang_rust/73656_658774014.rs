
 memory allocation of 1073741828 bytes failed[RUSTC-TIMING] hex test:false 1.487
rustc exited with signal: 6
error: could not compile `hex`.

Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name hex /cargo/registry/src/github.com-1285ae84e5963aae/hex-0.3.2/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -Cembed-bitcode=no -C debuginfo=0 -C metadata=7aa1d73c5dd23801 -C extra-filename=-7aa1d73c5dd23801 --out-dir /checkout/obj/build/i686-unknown-linux-gnu/stage1-tools/i686-unknown-linux-gnu/release/deps --target i686-unknown-linux-gnu -C linker=clang -L dependency=/checkout/obj/build/i686-unknown-linux-gnu/stage1-tools/i686-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/i686-unknown-linux-gnu/stage1-tools/release/deps --cap-lints allow -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Zbinary-dep-depinfo` (exit code: 254)
