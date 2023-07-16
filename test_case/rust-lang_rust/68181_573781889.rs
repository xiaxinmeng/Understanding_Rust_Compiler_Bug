plain
2020-01-13T17:37:51.0452210Z [RUSTC-TIMING] git_rustfmt test:false 11.786
2020-01-13T17:37:52.9977170Z [RUSTC-TIMING] rustfmt test:false 13.740
2020-01-13T17:37:53.2241250Z [RUSTC-TIMING] cargo_fmt test:false 13.964
2020-01-13T17:37:53.2468570Z     Finished release [optimized] target(s) in 5m 27s
2020-01-13T17:37:53.2878680Z duplicate artifacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:
2020-01-13T17:37:53.2879890Z the following dependencies are duplicated although they have the same features enabled:
2020-01-13T17:37:53.2880210Z the following dependencies have different features:
2020-01-13T17:37:53.2880210Z the following dependencies have different features:
2020-01-13T17:37:53.2881450Z   proc-macro2 0.4.30 (registry+https://github.com/rust-lang/crates.io-index)
2020-01-13T17:37:53.2882290Z     `rustfmt` additionally enabled features {"default"} at "/Users/runner/runners/2.163.1/work/1/s/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/deps/libproc_macro2-10a0b21f1377cd9a.rlib"
2020-01-13T17:37:53.2883080Z     `cargo` additionally enabled features {} at "/Users/runner/runners/2.163.1/work/1/s/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/deps/libproc_macro2-7ee6985e7de5b685.rlib"
2020-01-13T17:37:53.2883700Z   quote 0.6.12 (registry+https://github.com/rust-lang/crates.io-index)
2020-01-13T17:37:53.2884470Z     `rustfmt` additionally enabled features {"default"} at "/Users/runner/runners/2.163.1/work/1/s/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/deps/libquote-cc3fed200f7ff633.rlib"
2020-01-13T17:37:53.2885240Z     `cargo` additionally enabled features {} at "/Users/runner/runners/2.163.1/work/1/s/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/deps/libquote-38f3d268b489fc4a.rlib"
2020-01-13T17:37:53.2885340Z 
2020-01-13T17:37:53.2886020Z to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set
2020-01-13T17:37:53.2886710Z thread 'main' panicked at 'tools should not compile multiple copies of the same crate', src/bootstrap/tool.rs:195:13
2020-01-13T17:37:53.3005640Z failed to run: /Users/runner/runners/2.163.1/work/1/s/build/bootstrap/debug/bootstrap dist
2020-01-13T17:37:53.3006720Z Build completed unsuccessfully in 1:20:57
2020-01-13T17:37:53.3083420Z == clock drift check ==
2020-01-13T17:37:53.3145250Z   local time: Mon Jan 13 17:37:53 UTC 2020
2020-01-13T17:37:53.3145250Z   local time: Mon Jan 13 17:37:53 UTC 2020
2020-01-13T17:37:53.3856380Z   network time: Mon, 13 Jan 2020 17:37:53 GMT
2020-01-13T17:37:53.3859240Z == end clock drift check ==
2020-01-13T17:37:53.3922360Z 
2020-01-13T17:37:53.4045220Z ##[error]Bash exited with code '1'.
2020-01-13T17:37:53.4111660Z ##[section]Starting: Checkout
2020-01-13T17:37:53.4119000Z ==============================================================================
2020-01-13T17:37:53.4119110Z Task         : Get sources
2020-01-13T17:37:53.4119190Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
