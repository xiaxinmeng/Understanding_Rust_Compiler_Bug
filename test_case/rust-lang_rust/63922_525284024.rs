plain
2019-08-27T12:43:27.0372082Z [RUSTC-TIMING] miri test:false 62.399
2019-08-27T12:43:35.7343959Z [RUSTC-TIMING] cargo_miri test:false 8.635
2019-08-27T12:43:38.1043367Z [RUSTC-TIMING] miri test:false 11.031
2019-08-27T12:43:38.1237469Z     Finished release [optimized] target(s) in 1m 37s
2019-08-27T12:43:38.1468444Z duplicate artifacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:
2019-08-27T12:43:38.1469144Z 
2019-08-27T12:43:38.1469554Z the following dependencies are duplicated although they have the same features enabled:
2019-08-27T12:43:38.1469725Z the following dependencies have different features:
2019-08-27T12:43:38.1469861Z   getrandom 0.1.8 (registry+https://github.com/rust-lang/crates.io-index)
2019-08-27T12:43:38.1495969Z     `miri` additionally enabled features {} at "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\stage1-tools\\x86_64-pc-windows-gnu\\release\\deps\\libgetrandom-4754dc118ff8df3b.rlib"
2019-08-27T12:43:38.1517620Z     `cargo` additionally enabled features {"std"} at "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\stage1-tools\\x86_64-pc-windows-gnu\\release\\deps\\libgetrandom-ac194b7958ad050f.rlib"
2019-08-27T12:43:38.1517783Z 
2019-08-27T12:43:38.1517978Z to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set
2019-08-27T12:43:38.1533378Z thread 'main' panicked at 'tools should not compile multiple copies of the same crate', src\bootstrap\tool.rs:198:13
2019-08-27T12:43:38.3388253Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap dist
2019-08-27T12:43:38.3388686Z Build completed unsuccessfully in 2:32:30
2019-08-27T12:43:38.3974860Z == clock drift check ==
2019-08-27T12:43:38.4650581Z   local time: Tue Aug 27 12:43:38 CUT 2019
2019-08-27T12:43:38.4650581Z   local time: Tue Aug 27 12:43:38 CUT 2019
2019-08-27T12:43:38.9487665Z   network time: Tue, 27 Aug 2019 12:43:38 GMT
2019-08-27T12:43:38.9496207Z == end clock drift check ==
2019-08-27T12:43:39.5013459Z ##[error]Bash exited with code '1'.
2019-08-27T12:43:39.6020770Z ##[section]Starting: Upload CPU usage statistics
2019-08-27T12:43:39.6973116Z ==============================================================================
2019-08-27T12:43:39.6973197Z Task         : Bash
2019-08-27T12:43:39.6973489Z Description  : Run a Bash script on macOS, Linux, or Windows
