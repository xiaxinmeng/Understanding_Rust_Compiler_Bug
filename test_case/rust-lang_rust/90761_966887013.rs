plain
   Compiling itoa v0.4.6
[RUSTC-TIMING] rustc_fs_util test:false 0.142
   Compiling snap v1.0.1
[RUSTC-TIMING] build_script_build test:false 0.223
rustc exited with signal: 11 (core dumped)
error: could not compile `serde_derive`
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name build_script_build /cargo/registry/src/github.com-1ecc6299db9ec823/serde_derive-1.0.125/build.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=off --cfg 'feature="default"' -C metadata=ac27467a3c2a9e00 -C extra-filename=-ac27467a3c2a9e00 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/build/serde_derive-ac27467a3c2a9e00 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --cap-lints allow -Z binary-dep-depinfo` (exit status: 254)
[RUSTC-TIMING] itoa test:false 0.142
[RUSTC-TIMING] tinyvec test:false 0.314
[RUSTC-TIMING] datafrog test:false 0.356
[RUSTC-TIMING] rustc_graphviz test:false 0.562
