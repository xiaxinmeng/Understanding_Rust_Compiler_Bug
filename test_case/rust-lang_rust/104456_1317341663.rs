plain
[RUSTC-TIMING] libffi test:false 0.940
[RUSTC-TIMING] miri test:false 41.229
[RUSTC-TIMING] miri test:false 1.891
    Finished release [optimized] target(s) in 53.26s
duplicate artifacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:
the following dependencies have different features:
the following dependencies have different features:
  rand_core 0.6.2 (registry+https://github.com/rust-lang/crates.io-index)
    `miri` additionally enabled features {"std", "alloc", "getrandom"} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/riscv64gc-unknown-linux-gnu/release/deps/librand_core-f84bd431fa787485.rlib"
    `cargo` additionally enabled features {} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/riscv64gc-unknown-linux-gnu/release/deps/librand_core-a493ba75465e62c6.rlib"

to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set
thread 'main' panicked at 'tools should not compile multiple copies of the same crate', tool.rs:201:13
Build completed unsuccessfully in 0:25:22
