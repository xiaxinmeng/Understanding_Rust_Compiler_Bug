plain
2019-12-19T22:32:39.3198200Z 
2019-12-19T22:32:39.3517670Z [RUSTC-TIMING] cargo_clippy test:false 0.766
2019-12-19T22:32:42.7652090Z [RUSTC-TIMING] clippy_driver test:false 4.181
2019-12-19T22:32:42.7874270Z     Finished release [optimized] target(s) in 1m 47s
2019-12-19T22:32:42.8159030Z duplicate artifacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:
2019-12-19T22:32:42.8161180Z the following dependencies are duplicated although they have the same features enabled:
2019-12-19T22:32:42.8161640Z the following dependencies have different features:
2019-12-19T22:32:42.8161640Z the following dependencies have different features:
2019-12-19T22:32:42.8163020Z   smallvec 1.0.0 (registry+https://github.com/rust-lang/crates.io-index)
2019-12-19T22:32:42.8164260Z     `clippy-driver` additionally enabled features {"union"} at "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage1-tools/i686-apple-darwin/release/deps/libsmallvec-4d9460a0718804c5.rlib"
2019-12-19T22:32:42.8165460Z     `cargo` additionally enabled features {} at "/Users/runner/runners/2.163.1/work/1/s/build/i686-apple-darwin/stage1-tools/i686-apple-darwin/release/deps/libsmallvec-39f1bfb959d0ea73.rlib"
2019-12-19T22:32:42.8165930Z 
2019-12-19T22:32:42.8167020Z to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set
2019-12-19T22:32:42.8168170Z thread 'main' panicked at 'tools should not compile multiple copies of the same crate', src/bootstrap/tool.rs:198:13
2019-12-19T22:32:42.8255580Z failed to run: /Users/runner/runners/2.163.1/work/1/s/build/bootstrap/debug/bootstrap dist
2019-12-19T22:32:42.8255730Z Build completed unsuccessfully in 1:08:44
2019-12-19T22:32:42.8324630Z == clock drift check ==
2019-12-19T22:32:42.9434430Z   local time: Thu Dec 19 22:32:42 UTC 2019
2019-12-19T22:32:42.9434430Z   local time: Thu Dec 19 22:32:42 UTC 2019
2019-12-19T22:32:42.9434980Z   network time: Thu, 19 Dec 2019 22:32:42 GMT
2019-12-19T22:32:42.9435390Z == end clock drift check ==
2019-12-19T22:32:42.9435520Z 
2019-12-19T22:32:42.9525770Z ##[error]Bash exited with code '1'.
2019-12-19T22:32:42.9585690Z ##[section]Starting: Checkout
2019-12-19T22:32:42.9589360Z ==============================================================================
2019-12-19T22:32:42.9589460Z Task         : Get sources
2019-12-19T22:32:42.9589540Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
