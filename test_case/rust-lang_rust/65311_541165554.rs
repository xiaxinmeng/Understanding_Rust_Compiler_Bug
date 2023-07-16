plain
2019-10-11T18:00:21.4929090Z [RUSTC-TIMING] rls test:false 141.086
2019-10-11T18:01:04.7604980Z [RUSTC-TIMING] cargo test:false 258.472
2019-10-11T18:01:08.3829730Z [RUSTC-TIMING] rls test:false 3.586
2019-10-11T18:01:08.4300740Z     Finished release [optimized] target(s) in 7m 31s
2019-10-11T18:01:08.5045630Z duplicate artifacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:
2019-10-11T18:01:08.5045950Z 
2019-10-11T18:01:08.5046760Z the following dependencies are duplicated although they have the same features enabled:
2019-10-11T18:01:08.5047070Z the following dependencies have different features:
2019-10-11T18:01:08.5048510Z   scopeguard 1.0.0 (registry+https://github.com/rust-lang/crates.io-index)
2019-10-11T18:01:08.5050000Z     `rls` additionally enabled features {"use_std", "default"} at "/Users/vsts/agent/2.158.0/work/1/s/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/deps/libscopeguard-fb3bd4a4d7605406.rlib"
2019-10-11T18:01:08.5050930Z     `rustfmt` additionally enabled features {} at "/Users/vsts/agent/2.158.0/work/1/s/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/deps/libscopeguard-eff7e7769e630fbe.rlib"
2019-10-11T18:01:08.5051070Z 
2019-10-11T18:01:08.5052190Z to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set
2019-10-11T18:01:08.5053100Z thread 'main' panicked at 'tools should not compile multiple copies of the same crate', src/bootstrap/tool.rs:198:13
2019-10-11T18:01:08.5152570Z failed to run: /Users/vsts/agent/2.158.0/work/1/s/build/bootstrap/debug/bootstrap dist
2019-10-11T18:01:08.5152830Z Build completed unsuccessfully in 1:27:04
2019-10-11T18:01:08.5236930Z == clock drift check ==
2019-10-11T18:01:08.5311050Z   local time: Fri Oct 11 18:01:08 UTC 2019
2019-10-11T18:01:08.5311050Z   local time: Fri Oct 11 18:01:08 UTC 2019
2019-10-11T18:01:08.6201750Z   network time: Fri, 11 Oct 2019 18:01:08 GMT
2019-10-11T18:01:08.6204400Z == end clock drift check ==
2019-10-11T18:01:08.6447970Z ##[error]Bash exited with code '1'.
2019-10-11T18:01:08.6500260Z ##[section]Starting: Upload CPU usage statistics
2019-10-11T18:01:08.6541040Z ==============================================================================
2019-10-11T18:01:08.6541170Z Task         : Bash
2019-10-11T18:01:08.6541260Z Description  : Run a Bash script on macOS, Linux, or Windows
