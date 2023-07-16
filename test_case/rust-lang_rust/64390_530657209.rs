plain
2019-09-12T04:13:28.3924340Z [RUSTC-TIMING] rustfmt test:false 13.734
2019-09-12T04:13:29.6811370Z [RUSTC-TIMING] cargo_fmt test:false 15.020
2019-09-12T04:13:30.3339430Z [RUSTC-TIMING] git_rustfmt test:false 15.675
2019-09-12T04:13:30.3530620Z     Finished release [optimized] target(s) in 6m 02s
2019-09-12T04:13:30.3915900Z duplicate artifacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:
2019-09-12T04:13:30.3916420Z 
2019-09-12T04:13:30.3916910Z the following dependencies are duplicated although they have the same features enabled:
2019-09-12T04:13:30.3917280Z the following dependencies have different features:
2019-09-12T04:13:30.3918220Z   smallvec 0.6.10 (registry+https://github.com/rust-lang/crates.io-index)
2019-09-12T04:13:30.3919150Z     `rustfmt` additionally enabled features {"union", "may_dangle"} at "/Users/vsts/agent/2.155.1/work/1/s/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/deps/libsmallvec-c40ca41794accb8b.rlib"
2019-09-12T04:13:30.3920090Z     `cargo` additionally enabled features {} at "/Users/vsts/agent/2.155.1/work/1/s/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/deps/libsmallvec-481a098d400b358b.rlib"
2019-09-12T04:13:30.3920220Z 
2019-09-12T04:13:30.3921250Z to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set
2019-09-12T04:13:30.3922090Z thread 'main' panicked at 'tools should not compile multiple copies of the same crate', src/bootstrap/tool.rs:198:13
2019-09-12T04:13:30.4018700Z failed to run: /Users/vsts/agent/2.155.1/work/1/s/build/bootstrap/debug/bootstrap dist
2019-09-12T04:13:30.4019010Z Build completed unsuccessfully in 1:17:08
2019-09-12T04:13:30.4095180Z == clock drift check ==
2019-09-12T04:13:30.4148660Z   local time: Thu Sep 12 04:13:30 UTC 2019
2019-09-12T04:13:30.4148660Z   local time: Thu Sep 12 04:13:30 UTC 2019
2019-09-12T04:13:30.5168490Z   network time: Thu, 12 Sep 2019 04:13:30 GMT
2019-09-12T04:13:30.5171320Z == end clock drift check ==
2019-09-12T04:13:30.5377270Z ##[error]Bash exited with code '1'.
2019-09-12T04:13:30.5438650Z ##[section]Starting: Upload CPU usage statistics
2019-09-12T04:13:30.5443720Z ==============================================================================
2019-09-12T04:13:30.5443850Z Task         : Bash
2019-09-12T04:13:30.5443930Z Description  : Run a Bash script on macOS, Linux, or Windows
