plain
[RUSTC-TIMING] clippy_lints test:false 44.498
[RUSTC-TIMING] cargo_clippy test:false 0.561
[RUSTC-TIMING] clippy_driver test:false 2.216
    Finished release [optimized] target(s) in 49.01s
duplicate artifacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:

the following dependencies are duplicated although they have the same features enabled:
the following dependencies have different features:
  memchr 2.5.0 (registry+https://github.com/rust-lang/crates.io-index)
    `clippy-driver` additionally enabled features {} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-freebsd/release/deps/libmemchr-44aa6ff4f08e293f.rlib"
    `cargo` additionally enabled features {"use_std"} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-freebsd/release/deps/libmemchr-70e29af0fd3ef292.rlib"

to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set
thread 'main' panicked at 'tools should not compile multiple copies of the same crate', tool.rs:197:13
Build completed unsuccessfully in 0:18:01
