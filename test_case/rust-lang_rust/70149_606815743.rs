
[RUSTC-TIMING] rustc_ap_rustc_expand test:false 47.923
[RUSTC-TIMING] rustfmt_nightly test:false 58.577
[RUSTC-TIMING] rustfmt_format_diff test:false 5.161
[RUSTC-TIMING] rustfmt test:false 10.388
[RUSTC-TIMING] cargo_fmt test:false 11.505
[RUSTC-TIMING] git_rustfmt test:false 11.720
    Finished release [optimized] target(s) in 4m 35s
duplicate artifacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:

the following dependencies are duplicated although they have the same features enabled:
the following dependencies have different features:
  crossbeam-utils 0.7.2 (registry+https://github.com/rust-lang/crates.io-index)
    `rustfmt` additionally enabled features {"nightly"} at "/Users/runner/runners/2.165.2/work/1/s/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/deps/libcrossbeam_utils-dbe732c9e11ac54e.rlib"
    `cargo` additionally enabled features {} at "/Users/runner/runners/2.165.2/work/1/s/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/deps/libcrossbeam_utils-6901409fd1f394ca.rlib"

to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set
thread 'main' panicked at 'tools should not compile multiple copies of the same crate', src/bootstrap/tool.rs:195:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
failed to run: /Users/runner/runners/2.165.2/work/1/s/build/bootstrap/debug/bootstrap dist
Build completed unsuccessfully in 1:07:37
== clock drift check ==
  local time: Tue Mar 31 19:03:13 UTC 2020
  network time: Tue, 31 Mar 2020 19:03:13 GMT
== end clock drift check ==

