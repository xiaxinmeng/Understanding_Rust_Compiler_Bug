plain
   Compiling build_helper v0.1.0 (/checkout/src/build_helper)
   Compiling thread_local v1.0.1
   Compiling walkdir v2.3.1
   Compiling getopts v0.2.21
error: could not compile `once_cell`
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc --crate-name once_cell --edition=2018 /cargo/registry/src/github.com-1ecc6299db9ec823/once_cell-1.7.2/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=0 --cfg 'feature="alloc"' --cfg 'feature="default"' --cfg 'feature="race"' --cfg 'feature="std"' -C metadata=5bf50caf48c031d7 -C extra-filename=-5bf50caf48c031d7 --out-dir /checkout/obj/build/bootstrap/debug/deps -L dependency=/checkout/obj/build/bootstrap/debug/deps --cap-lints allow -Wrust_2018_idioms -Wunused_lifetimes -Wsemicolon_in_expressions_from_macros -Dwarnings` (signal: 6, SIGABRT: process abort signal)
error: build failed
failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
Build completed unsuccessfully in 0:00:56
make: *** [prepare] Error 1
---
[RUSTC-TIMING] itoa test:false 0.103
   Compiling pest v2.1.3
   Compiling walkdir v2.3.1
[RUSTC-TIMING] build_script_build test:false 0.230
rustc exited with signal: 11 (core dumped)
error: could not compile `memchr`
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name build_script_build --edition=2018 /cargo/registry/src/github.com-1ecc6299db9ec823/memchr-2.4.1/build.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=off --cfg 'feature="default"' --cfg 'feature="std"' -C metadata=dbf3c232759be82b -C extra-filename=-dbf3c232759be82b --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release/build/memchr-dbf3c232759be82b -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release/deps --cap-lints allow -Z binary-dep-depinfo` (exit status: 254)
[RUSTC-TIMING] same_file test:false 0.215
[RUSTC-TIMING] autocfg test:false 0.338
[RUSTC-TIMING] build_script_build test:false 0.366
[RUSTC-TIMING] build_script_build test:false 0.413
