plain
   Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
error: could not compile `lazy_static`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc --crate-name lazy_static /cargo/registry/src/github.com-1ecc6299db9ec823/lazy_static-1.4.0/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=0 -C metadata=1f5d3923e02f6398 -C extra-filename=-1f5d3923e02f6398 --out-dir /checkout/obj/build/bootstrap/debug/deps -L dependency=/checkout/obj/build/bootstrap/debug/deps --cap-lints allow -Wrust_2018_idioms -Wunused_lifetimes -Wsemicolon_in_expressions_from_macros -Dwarnings` (signal: 11, SIGSEGV: invalid memory reference)
error: build failed
failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
Build completed unsuccessfully in 0:00:54
make: *** [prepare] Error 1
---
[RUSTC-TIMING] flate2 test:false 0.773
[RUSTC-TIMING] crossbeam_utils test:false 0.966
[RUSTC-TIMING] crossbeam_epoch test:false 0.723
[RUSTC-TIMING] rayon test:false 0.178
rustc exited with signal: 11 (core dumped)
error: could not compile `rayon`
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rayon --edition=2018 /cargo/registry/src/github.com-1ecc6299db9ec823/rayon-1.3.1/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C metadata=5531beba540f0424 -C extra-filename=-5531beba540f0424 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release/deps --extern crossbeam_deque=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps/libcrossbeam_deque-12cac83920907799.rmeta --extern either=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps/libeither-25313dce9eae3e67.rmeta --extern rayon_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps/librayon_core-224ff4e33ff777ff.rmeta --cap-lints allow --cfg=bootstrap -Zsymbol-mangling-version=v0 -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Ztls-model=initial-exec -Z binary-dep-depinfo --cfg step_by` (exit status: 254)
[RUSTC-TIMING] yaml_rust test:false 2.473
[RUSTC-TIMING] rayon_core test:false 0.993
[RUSTC-TIMING] tar test:false 1.716
[RUSTC-TIMING] clap test:false 5.929
