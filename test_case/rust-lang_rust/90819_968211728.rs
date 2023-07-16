plain
[RUSTC-TIMING] ppv_lite86 test:false 0.413
   Compiling rustc_fs_util v0.0.0 (/checkout/compiler/rustc_fs_util)
[RUSTC-TIMING] build_script_build test:false 0.526
[RUSTC-TIMING] opaque_debug test:false 0.185
rustc exited with signal: 11 (core dumped)
error: could not compile `opaque-debug`
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name opaque_debug --edition=2018 /cargo/registry/src/github.com-1ecc6299db9ec823/opaque-debug-0.3.0/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C metadata=8b9f88d5c7b6706c -C extra-filename=-8b9f88d5c7b6706c --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --cap-lints allow --cfg=bootstrap -Zsymbol-mangling-version=v0 -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Ztls-model=initial-exec -Zunstable-options '-Wrustc::internal' -Cprefer-dynamic -Z binary-dep-depinfo` (exit status: 254)
[RUSTC-TIMING] autocfg test:false 0.537
[RUSTC-TIMING] unicode_width test:false 0.090
[RUSTC-TIMING] version_check test:false 0.488
[RUSTC-TIMING] scoped_tls test:false 0.096
