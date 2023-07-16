plain
[RUSTC-TIMING] rls test:false 76.588
[RUSTC-TIMING] cargo test:false 144.628
[RUSTC-TIMING] rls test:false 6.650
    Finished release [optimized] target(s) in 5m 54s
duplicate artifacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:

the following dependencies are duplicated although they have the same features enabled:
the following dependencies have different features:
  winapi 0.3.9 (registry+https://github.com/rust-lang/crates.io-index)
    `rls` additionally enabled features {"winnt"} at "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\release\\deps\\libwinapi-b256d0ae8f699008.rlib"
    `clippy-driver` additionally enabled features {} at "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\release\\deps\\libwinapi-befa53badc4f2607.rlib"

to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set
thread 'main' panicked at 'tools should not compile multiple copies of the same crate', src\bootstrap\tool.rs:197:13
Build completed unsuccessfully in 0:37:47
{"embedded-book":"test-pass","reference":"test-pass","edition-guide":"test-pass","rust-by-example":"test-pass","book":"test-pass","nomicon":"test-pass","rustbook":"test-fail"}Building rustbuild
    Finished dev [unoptimized] target(s) in 0.33s
    Finished dev [unoptimized] target(s) in 0.33s
error: Tool `rls` was not recorded in tool state.
error: Tool `miri` was not recorded in tool state.
