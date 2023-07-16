plain
2020-03-04T04:08:52.3147790Z [RUSTC-TIMING] git_rustfmt test:false 9.494
2020-03-04T04:08:54.2911110Z [RUSTC-TIMING] rustfmt test:false 11.480
2020-03-04T04:08:54.5164080Z [RUSTC-TIMING] cargo_fmt test:false 11.708
2020-03-04T04:08:54.5410070Z     Finished release [optimized] target(s) in 5m 20s
2020-03-04T04:08:54.5742630Z duplicate artifacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:
2020-03-04T04:08:54.5743800Z the following dependencies are duplicated although they have the same features enabled:
2020-03-04T04:08:54.5744250Z the following dependencies have different features:
2020-03-04T04:08:54.5744250Z the following dependencies have different features:
2020-03-04T04:08:54.5745550Z   proc-macro2 0.4.30 (registry+https://github.com/rust-lang/crates.io-index)
2020-03-04T04:08:54.5746790Z     `rustfmt` additionally enabled features {"default"} at "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/deps/libproc_macro2-58288f9f59724080.rlib"
2020-03-04T04:08:54.5748230Z     `cargo` additionally enabled features {} at "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/deps/libproc_macro2-ad9101f6f50fce55.rlib"
2020-03-04T04:08:54.5749280Z   quote 0.6.12 (registry+https://github.com/rust-lang/crates.io-index)
2020-03-04T04:08:54.5750460Z     `rustfmt` additionally enabled features {"default"} at "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/deps/libquote-d6c8c62334473aa9.rlib"
2020-03-04T04:08:54.5751860Z     `cargo` additionally enabled features {} at "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/deps/libquote-571fc6c253801dac.rlib"
2020-03-04T04:08:54.5752910Z   syn 0.15.35 (registry+https://github.com/rust-lang/crates.io-index)
2020-03-04T04:08:54.5754070Z     `rustfmt` additionally enabled features {"visit"} at "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/deps/libsyn-7a2f4a52e285b5df.rlib"
2020-03-04T04:08:54.5756090Z     `cargo` additionally enabled features {} at "/Users/runner/runners/2.165.0/work/1/s/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/deps/libsyn-61f655d09c570865.rlib"
2020-03-04T04:08:54.5756640Z 
2020-03-04T04:08:54.5757700Z to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set
2020-03-04T04:08:54.5758880Z thread 'main' panicked at 'tools should not compile multiple copies of the same crate', src/bootstrap/tool.rs:195:13
2020-03-04T04:08:54.5778420Z failed to run: /Users/runner/runners/2.165.0/work/1/s/build/bootstrap/debug/bootstrap dist
2020-03-04T04:08:54.5779750Z Build completed unsuccessfully in 1:11:50
2020-03-04T04:08:54.5873770Z == clock drift check ==
2020-03-04T04:08:54.5951150Z   local time: Wed Mar  4 04:08:54 UTC 2020
2020-03-04T04:08:54.5951150Z   local time: Wed Mar  4 04:08:54 UTC 2020
2020-03-04T04:08:54.6566010Z   network time: Wed, 04 Mar 2020 04:08:54 GMT
2020-03-04T04:08:54.6571150Z == end clock drift check ==
2020-03-04T04:08:54.6655420Z 
2020-03-04T04:08:54.6728060Z ##[error]Bash exited with code '1'.
2020-03-04T04:08:54.6801000Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-03-04T04:08:54.6807290Z ==============================================================================
2020-03-04T04:08:54.6807680Z Task         : Get sources
2020-03-04T04:08:54.6808190Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
