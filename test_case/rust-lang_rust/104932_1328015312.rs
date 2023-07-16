plain
[RUSTC-TIMING] rustc_workspace_hack test:false 0.033
   Compiling rls v2.0.0 (/checkout/src/tools/rls)
[RUSTC-TIMING] rls test:false 0.800
    Finished release [optimized] target(s) in 22.97s
duplicate artifacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:
the following dependencies have different features:
the following dependencies have different features:
  libc 0.2.135 (registry+https://github.com/rust-lang/crates.io-index)
    `rls` additionally enabled features {} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/riscv64gc-unknown-linux-gnu/release/deps/liblibc-631c5159aa4417fb.rlib"
    `cargo` additionally enabled features {"extra_traits"} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/riscv64gc-unknown-linux-gnu/release/deps/liblibc-a042b22410d59e12.rlib"

to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set
thread 'main' panicked at 'tools should not compile multiple copies of the same crate', tool.rs:201:13
Build completed unsuccessfully in 0:17:01
