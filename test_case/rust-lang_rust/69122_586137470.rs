plain
2020-02-14T07:41:31.2466964Z failures:
2020-02-14T07:41:31.2467188Z 
2020-02-14T07:41:31.2468151Z ---- backtrace::test_debug stdout ----
2020-02-14T07:41:31.2468852Z thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
2020-02-14T07:41:31.2469315Z   left: `"Backtrace [\n    { fn: \"__rust_maybe_catch_panic\" },\n    { fn: \"std::rt::lang_start_internal\", file: \"./rust/rt.rs\", line: 300 },\n    { fn: \"std::rt::lang_start\", file: \"./rust/rt.rs\", line: 400 },\n]"`,
2020-02-14T07:41:31.2470232Z  right: `"Backtrace [\n    { fn: \"__rust_maybe_catch_panic\" },\n    { fn: \"std::rt::lang_start_internal\", file: \"/rust/rt.rs\", line: 300 },\n    { fn: \"std::rt::lang_start\", file: \"/rust/rt.rs\", line: 400 },\n]"`', src/libstd/backtrace.rs:486:5
2020-02-14T07:41:31.2470946Z 
2020-02-14T07:41:31.2471179Z 
2020-02-14T07:41:31.2471807Z failures:
2020-02-14T07:41:31.2472061Z     backtrace::test_debug
---
2020-02-14T07:41:31.2596857Z 
2020-02-14T07:41:31.2596898Z 
2020-02-14T07:41:31.2597554Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target arm-unknown-linux-gnueabihf
2020-02-14T07:41:31.2597823Z Build completed unsuccessfully in 2:45:22
2020-02-14T07:41:31.2598461Z error: test failed, to rerun pass '-p std --lib'
2020-02-14T07:41:31.2659241Z   local time: Fri Feb 14 07:41:31 UTC 2020
2020-02-14T07:41:31.5540902Z   network time: Fri, 14 Feb 2020 07:41:31 GMT
2020-02-14T07:41:31.5545990Z == end clock drift check ==
2020-02-14T07:41:32.0895147Z 
2020-02-14T07:41:32.0895147Z 
2020-02-14T07:41:32.0990657Z ##[error]Bash exited with code '1'.
2020-02-14T07:41:32.1038944Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-02-14T07:41:32.1041106Z ==============================================================================
2020-02-14T07:41:32.1041222Z Task         : Get sources
2020-02-14T07:41:32.1041306Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
