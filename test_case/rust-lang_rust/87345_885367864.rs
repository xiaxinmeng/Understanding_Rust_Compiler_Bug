plain
[RUSTC-TIMING] cargo test:false 119.678
[RUSTC-TIMING] rls test:false 72.282
[RUSTC-TIMING] rls test:false 4.601
    Finished release [optimized] target(s) in 2m 55s
duplicate artifacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:

the following dependencies are duplicated although they have the same features enabled:
the following dependencies have different features:
  winapi 0.3.9 (registry+https://github.com/rust-lang/crates.io-index)
    `rls` additionally enabled features {"mstcpip", "windef", "cfg", "mswsock", "evntrace", "in6addr", "inaddr", "winioctl"} at "D:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\stage1-tools\\i686-pc-windows-gnu\\release\\deps\\libwinapi-b5d072094a4afc64.rlib"
thread 'main' panicked at 'tools should not compile multiple copies of the same crate', src\bootstrap\tool.rs:199:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    `cargo` additionally enabled features {} at "D:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\stage1-tools\\i686-pc-windows-gnu\\release\\deps\\libwinapi-05314490ae7cb55e.rlib"

to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set
