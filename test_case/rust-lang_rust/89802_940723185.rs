plain
[RUSTC-TIMING] rustc_workspace_hack test:false 0.061
[RUSTC-TIMING] miri test:false 44.924
[RUSTC-TIMING] miri test:false 2.723
    Finished release [optimized] target(s) in 1m 40s
duplicate artifacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:

the following dependencies are duplicated although they have the same features enabled:
the following dependencies have different features:
  winapi 0.3.9 (registry+https://github.com/rust-lang/crates.io-index)
    `miri` additionally enabled features {} at "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\release\\deps\\libwinapi-9eb4f517d1f870f8.rlib"
    `rls` additionally enabled features {"winreg", "winuser"} at "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2-tools\\x86_64-pc-windows-msvc\\release\\deps\\libwinapi-cc6ac958f961c2cc.rlib"

to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set
thread 'main' panicked at 'tools should not compile multiple copies of the same crate', src\bootstrap\tool.rs:199:13
Build completed unsuccessfully in 0:45:55
    Finished dev [unoptimized] target(s) in 0.33s
    Finished dev [unoptimized] target(s) in 0.33s
error: Tool `miri` was not recorded in tool state.
{"book":"test-pass","reference":"test-pass","edition-guide":"test-pass","rls":"test-pass","rustbook":"test-fail","embedded-book":"test-pass","nomicon":"test-pass","rust-by-example":"test-pass"}Build completed unsuccessfully in 0:00:01
