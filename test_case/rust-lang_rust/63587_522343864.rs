plain
2019-08-18T18:21:21.7505040Z [RUSTC-TIMING] clippy test:false 1.289
2019-08-18T18:21:22.8904500Z [RUSTC-TIMING] cargo_clippy test:false 1.108
2019-08-18T18:21:27.0171330Z [RUSTC-TIMING] clippy_driver test:false 5.234
2019-08-18T18:21:27.0412390Z     Finished release [optimized] target(s) in 3m 15s
2019-08-18T18:21:27.0808370Z duplicate artifacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:
2019-08-18T18:21:27.0808800Z 
2019-08-18T18:21:27.0809450Z the following dependencies are duplicated although they have the same features enabled:
2019-08-18T18:21:27.0809620Z the following dependencies have different features:
2019-08-18T18:21:27.0811940Z   url 2.1.0 (registry+https://github.com/rust-lang/crates.io-index)
2019-08-18T18:21:27.0812850Z     `clippy-driver` additionally enabled features {} at "/Users/vsts/agent/2.155.1/work/1/s/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/deps/liburl-f1a28a34ab627fbd.rlib"
2019-08-18T18:21:27.0825390Z     `cargo` additionally enabled features {"serde"} at "/Users/vsts/agent/2.155.1/work/1/s/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/deps/liburl-cb41a5d5e43c5de1.rlib"
2019-08-18T18:21:27.0825550Z 
2019-08-18T18:21:27.0826420Z to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set
2019-08-18T18:21:27.0827210Z thread 'main' panicked at 'tools should not compile multiple copies of the same crate', src/bootstrap/tool.rs:198:13
2019-08-18T18:21:27.0915580Z failed to run: /Users/vsts/agent/2.155.1/work/1/s/build/bootstrap/debug/bootstrap dist
2019-08-18T18:21:27.0916030Z Build completed unsuccessfully in 1:17:04
2019-08-18T18:21:27.0989760Z == clock drift check ==
2019-08-18T18:21:27.1072110Z   local time: Sun Aug 18 18:21:27 UTC 2019
2019-08-18T18:21:27.1072110Z   local time: Sun Aug 18 18:21:27 UTC 2019
2019-08-18T18:21:27.1863330Z   network time: Sun, 18 Aug 2019 18:21:27 GMT
2019-08-18T18:21:27.1866790Z == end clock drift check ==
2019-08-18T18:21:27.2086840Z ##[error]Bash exited with code '1'.
2019-08-18T18:21:27.2154180Z ##[section]Starting: Upload CPU usage statistics
2019-08-18T18:21:27.2185190Z ==============================================================================
2019-08-18T18:21:27.2185310Z Task         : Bash
2019-08-18T18:21:27.2185400Z Description  : Run a Bash script on macOS, Linux, or Windows
