plain
2019-07-30T13:49:39.8692330Z [RUSTC-TIMING] git_rustfmt test:false 12.685
2019-07-30T13:49:41.4642870Z [RUSTC-TIMING] rustfmt test:false 14.284
2019-07-30T13:49:41.7453490Z [RUSTC-TIMING] cargo_fmt test:false 14.566
2019-07-30T13:49:41.7637720Z     Finished release [optimized] target(s) in 3m 12s
2019-07-30T13:49:41.8022060Z duplicate artifacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:
2019-07-30T13:49:41.8022610Z 
2019-07-30T13:49:41.8022740Z the following dependencies are duplicated although they have the same features enabled:
2019-07-30T13:49:41.8023220Z the following dependencies have different features:
2019-07-30T13:49:41.8024190Z   crossbeam-utils 0.6.5 (registry+https://github.com/rust-lang/crates.io-index)
2019-07-30T13:49:41.8025120Z     `rustfmt` additionally enabled features {"nightly"} at "/Users/vsts/agent/2.154.3/work/1/s/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/deps/libcrossbeam_utils-6c8cd67381bcfe02.rlib"
2019-07-30T13:49:41.8026150Z     `cargo` additionally enabled features {} at "/Users/vsts/agent/2.154.3/work/1/s/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/deps/libcrossbeam_utils-683657da3f16e5d0.rlib"
2019-07-30T13:49:41.8026300Z 
2019-07-30T13:49:41.8027090Z to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set
2019-07-30T13:49:41.8027860Z thread 'main' panicked at 'tools should not compile multiple copies of the same crate', src/bootstrap/tool.rs:198:13
2019-07-30T13:49:41.8150070Z failed to run: /Users/vsts/agent/2.154.3/work/1/s/build/bootstrap/debug/bootstrap dist
2019-07-30T13:49:41.8150730Z Build completed unsuccessfully in 1:17:33
2019-07-30T13:49:41.8150730Z Build completed unsuccessfully in 1:17:33
2019-07-30T13:49:41.8467730Z ##[error]Bash exited with code '1'.
2019-07-30T13:49:41.8528950Z ##[section]Starting: Upload CPU usage statistics
2019-07-30T13:49:41.8562050Z ==============================================================================
2019-07-30T13:49:41.8562170Z Task         : Bash
2019-07-30T13:49:41.8562270Z Description  : Run a Bash script on macOS, Linux, or Windows
