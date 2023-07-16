plain
2019-09-18T03:47:33.0639540Z [RUSTC-TIMING] cargo test:false 184.425
2019-09-18T03:47:50.1505190Z [RUSTC-TIMING] rls test:false 179.841
2019-09-18T03:47:53.4857240Z [RUSTC-TIMING] rls test:false 3.273
2019-09-18T03:47:53.5328340Z     Finished release [optimized] target(s) in 5m 01s
2019-09-18T03:47:53.5894810Z duplicate artifacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:
2019-09-18T03:47:53.5895270Z 
2019-09-18T03:47:53.5895830Z the following dependencies are duplicated although they have the same features enabled:
2019-09-18T03:47:53.5896200Z the following dependencies have different features:
2019-09-18T03:47:53.5897380Z   tar 0.4.20 (registry+https://github.com/rust-lang/crates.io-index)
2019-09-18T03:47:53.5898310Z     `rls` additionally enabled features {} at "/Users/vsts/agent/2.155.1/work/1/s/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/deps/libtar-9b87067b769484f5.rlib"
2019-09-18T03:47:53.5899210Z     `cargo` additionally enabled features {"default", "xattr"} at "/Users/vsts/agent/2.155.1/work/1/s/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/deps/libtar-1b9488c5c76ec99f.rlib"
2019-09-18T03:47:53.5899360Z 
2019-09-18T03:47:53.5900760Z to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set
2019-09-18T03:47:53.5901630Z thread 'main' panicked at 'tools should not compile multiple copies of the same crate', src/bootstrap/tool.rs:198:13
2019-09-18T03:47:53.6019080Z failed to run: /Users/vsts/agent/2.155.1/work/1/s/build/bootstrap/debug/bootstrap dist
2019-09-18T03:47:53.6019620Z Build completed unsuccessfully in 1:22:59
2019-09-18T03:47:53.6093610Z == clock drift check ==
2019-09-18T03:47:53.6140600Z   local time: Wed Sep 18 03:47:53 UTC 2019
2019-09-18T03:47:53.6140600Z   local time: Wed Sep 18 03:47:53 UTC 2019
2019-09-18T03:47:53.7174280Z   network time: Wed, 18 Sep 2019 03:47:53 GMT
2019-09-18T03:47:53.7177340Z == end clock drift check ==
2019-09-18T03:47:53.7405440Z ##[error]Bash exited with code '1'.
2019-09-18T03:47:53.7517380Z ##[section]Starting: Upload CPU usage statistics
2019-09-18T03:47:53.7524810Z ==============================================================================
2019-09-18T03:47:53.7524980Z Task         : Bash
2019-09-18T03:47:53.7525120Z Description  : Run a Bash script on macOS, Linux, or Windows
