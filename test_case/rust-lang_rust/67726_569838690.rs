plain
2019-12-30T23:06:51.3987249Z    Compiling rustbook v0.1.0 (/checkout/src/tools/rustbook)
2019-12-30T23:06:56.6536720Z [RUSTC-TIMING] rustbook test:false 5.243
2019-12-30T23:06:56.6564947Z     Finished release [optimized] target(s) in 10m 13s
2019-12-30T23:06:56.6817986Z [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "rustbook", path: "src/tools/rustbook", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: ["linkcheck"] } -- 613.460
2019-12-30T23:08:32.6026487Z error: The server responded with 404 Not Found for "https://github.com/rust-lang/rust/tree/master/src/libsyntax_expand/mbe"
2019-12-30T23:08:32.6027807Z     ┌── macro-expansion.md:13:1 ───
2019-12-30T23:08:32.6028448Z     │
2019-12-30T23:08:32.6028448Z     │
2019-12-30T23:08:32.6028980Z  13 │ [`src/libsyntax_expand/mbe/`][code_dir]. This chapter aims to explain how macro
2019-12-30T23:08:32.6048779Z     │
2019-12-30T23:08:32.6049135Z 
2019-12-30T23:08:32.6050136Z error: The server responded with 404 Not Found for "https://doc.rust-lang.org/nightly/nightly-rustc/rustc_mir/borrow_check/nll/type_check/index.html"
2019-12-30T23:08:32.6050533Z 
---
2019-12-30T23:43:05.4316540Z Verifying status of rustfmt...
2019-12-30T23:43:05.4316777Z Verifying status of clippy-driver...
2019-12-30T23:43:05.4317043Z This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
2019-12-30T23:43:05.4317113Z 
2019-12-30T23:43:05.4317365Z We detected that this PR updated 'clippy-driver', but its tests failed.
2019-12-30T23:43:05.4317415Z 
2019-12-30T23:43:05.4317690Z If you do intend to update 'clippy-driver', please check the error messages above and
2019-12-30T23:43:05.4317784Z commit another update.
2019-12-30T23:43:05.4317821Z 
2019-12-30T23:43:05.4319693Z If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
2019-12-30T23:43:05.4320357Z change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
2019-12-30T23:43:05.4320460Z proper steps.
2019-12-30T23:43:05.4321164Z Build completed unsuccessfully in 0:00:01
2019-12-30T23:43:05.4377163Z == clock drift check ==
2019-12-30T23:43:05.4391707Z   local time: Mon Dec 30 23:43:05 UTC 2019
2019-12-30T23:43:05.7121687Z   network time: Mon, 30 Dec 2019 23:43:05 GMT
2019-12-30T23:43:05.7121687Z   network time: Mon, 30 Dec 2019 23:43:05 GMT
2019-12-30T23:43:05.7121997Z == end clock drift check ==
2019-12-30T23:43:06.1820813Z 
2019-12-30T23:43:06.1920659Z ##[error]Bash exited with code '1'.
2019-12-30T23:43:06.1956270Z ##[section]Starting: Checkout
2019-12-30T23:43:06.1957850Z ==============================================================================
2019-12-30T23:43:06.1957937Z Task         : Get sources
2019-12-30T23:43:06.1958179Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
