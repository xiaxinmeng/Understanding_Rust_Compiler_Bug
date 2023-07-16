plain
2020-01-11T20:58:54.2889980Z [RUSTC-TIMING] git_rustfmt test:false 9.865
2020-01-11T20:58:55.9649140Z [RUSTC-TIMING] cargo_fmt test:false 11.543
2020-01-11T20:58:56.0902350Z [RUSTC-TIMING] rustfmt test:false 11.670
2020-01-11T20:58:56.1102760Z     Finished release [optimized] target(s) in 5m 01s
2020-01-11T20:58:56.1489970Z duplicate artifacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:
2020-01-11T20:58:56.1490630Z the following dependencies are duplicated although they have the same features enabled:
2020-01-11T20:58:56.1490930Z the following dependencies have different features:
2020-01-11T20:58:56.1490930Z the following dependencies have different features:
2020-01-11T20:58:56.1492170Z   proc-macro2 0.4.30 (registry+https://github.com/rust-lang/crates.io-index)
2020-01-11T20:58:56.1492970Z     `rustfmt` additionally enabled features {"default"} at "/Users/runner/runners/2.163.1/work/1/s/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/deps/libproc_macro2-fd9e5a07d9968100.rlib"
2020-01-11T20:58:56.1493740Z     `cargo` additionally enabled features {} at "/Users/runner/runners/2.163.1/work/1/s/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/deps/libproc_macro2-b2ae2ec8bf87d4cf.rlib"
2020-01-11T20:58:56.1494350Z   quote 0.6.12 (registry+https://github.com/rust-lang/crates.io-index)
2020-01-11T20:58:56.1495060Z     `rustfmt` additionally enabled features {"default"} at "/Users/runner/runners/2.163.1/work/1/s/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/deps/libquote-0169323817c75a85.rlib"
2020-01-11T20:58:56.1496150Z     `cargo` additionally enabled features {} at "/Users/runner/runners/2.163.1/work/1/s/build/x86_64-apple-darwin/stage1-tools/x86_64-apple-darwin/release/deps/libquote-422014b721eee20f.rlib"
2020-01-11T20:58:56.1496280Z 
2020-01-11T20:58:56.1497140Z to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set
2020-01-11T20:58:56.1497790Z thread 'main' panicked at 'tools should not compile multiple copies of the same crate', src/bootstrap/tool.rs:195:13
2020-01-11T20:58:56.1613620Z failed to run: /Users/runner/runners/2.163.1/work/1/s/build/bootstrap/debug/bootstrap dist
2020-01-11T20:58:56.1613850Z Build completed unsuccessfully in 1:06:45
2020-01-11T20:58:56.1678300Z == clock drift check ==
2020-01-11T20:58:56.1732510Z   local time: Sat Jan 11 20:58:56 UTC 2020
2020-01-11T20:58:56.1732510Z   local time: Sat Jan 11 20:58:56 UTC 2020
2020-01-11T20:58:56.2128810Z   network time: Sat, 11 Jan 2020 20:58:56 GMT
2020-01-11T20:58:56.2135350Z == end clock drift check ==
2020-01-11T20:58:56.2193410Z 
2020-01-11T20:58:56.2375760Z ##[error]Bash exited with code '1'.
2020-01-11T20:58:56.2422500Z ##[section]Starting: Checkout
2020-01-11T20:58:56.2425000Z ==============================================================================
2020-01-11T20:58:56.2425100Z Task         : Get sources
2020-01-11T20:58:56.2425190Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
