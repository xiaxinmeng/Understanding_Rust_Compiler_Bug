plain
[RUSTC-TIMING] clippy_lints test:false 41.564
[RUSTC-TIMING] cargo_clippy test:false 0.996
[RUSTC-TIMING] clippy_driver test:false 1.763
    Finished release [optimized] target(s) in 50.71s
duplicate artifacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:
the following dependencies have different features:
the following dependencies have different features:
  syn 2.0.8 (registry+https://github.com/rust-lang/crates.io-index)
    `clippy-driver` additionally enabled features {"full"} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/mips-unknown-linux-gnu/release/deps/libsyn-ed4b833c77d09371.rlib"
    `cargo` additionally enabled features {} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/mips-unknown-linux-gnu/release/deps/libsyn-b24d1c6bd1c1e71f.rlib"

to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set
thread 'main' panicked at 'tools should not compile multiple copies of the same crate', tool.rs:250:13
Build completed unsuccessfully in 0:20:01
