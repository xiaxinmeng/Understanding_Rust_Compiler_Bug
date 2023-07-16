plain
[RUSTC-TIMING] rls test:false 31.644
[RUSTC-TIMING] cargo test:false 61.452
[RUSTC-TIMING] rls test:false 6.108
    Finished release [optimized] target(s) in 1m 23s
duplicate artifacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:

the following dependencies are duplicated although they have the same features enabled:
the following dependencies have different features:
  clap 3.1.1 (registry+https://github.com/rust-lang/crates.io-index)
    `rls` additionally enabled features {"lazy_static", "derive", "clap_derive"} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/powerpc64-unknown-linux-gnu/release/deps/libclap-19eaec9c34485c0b.rlib"
    `cargo` additionally enabled features {} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-tools/powerpc64-unknown-linux-gnu/release/deps/libclap-8029a1b8bc4924ef.rlib"

to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set
thread 'main' panicked at 'tools should not compile multiple copies of the same crate', src/bootstrap/tool.rs:197:13
Build completed unsuccessfully in 0:19:51
