`
   Finished release [optimized] target(s) in 7m 58s
duplicate artifacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:

the following dependencies are duplicated although they have the same features enabled:
the following dependencies have different features:
  winapi 0.3.8 (registry+https://github.com/rust-lang/crates.io-index)
    `rustfmt` additionally enabled features {"fibersapi"} at "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\stage1-tools\\x86_64-pc-windows-msvc\\release\\deps\\libwinapi-8368770fccd67b10.rlib"
    `cargo` additionally enabled features {} at "D:\\a\\1\\s\\build\\x86_64-pc-windows-msvc\\stage1-tools\\x86_64-pc-windows-msvc\\release\\deps\\libwinapi-2441560a7cfe905d.rlib"

to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set
thread 'main' panicked at 'tools should not compile multiple copies of the same crate', src\bootstrap\tool.rs:195:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap dist
Build completed unsuccessfully in 2:22:26
