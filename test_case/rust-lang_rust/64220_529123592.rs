plain
2019-09-07T16:08:46.6476362Z     Finished release [optimized] target(s) in 8m 39s
2019-09-07T16:08:46.6716484Z [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "rustbook", path: "src/tools/rustbook", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: ["linkcheck"] } -- 519.277
2019-09-07T16:10:13.7296265Z Error: there are broken links
2019-09-07T16:10:13.7298497Z  Caused By: There was an error while fetching "https://public.etherpad-mozilla.org/p/rust-compiler-meeting", https://public.etherpad-mozilla.org/p/rust-compiler-meeting: error trying to connect: Connection refused (os error 111)
2019-09-07T16:10:13.7299198Z  Caused By: "https://cs.au.dk/~amoeller/spa/" returned 503 Service Unavailable
2019-09-07T16:10:13.7312355Z 
2019-09-07T16:10:13.7313335Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rustbook" "linkcheck" "/checkout/src/doc/rustc-guide"
2019-09-07T16:10:13.7313692Z expected success, got: exit code: 101
2019-09-07T16:10:13.7313885Z 
---
2019-09-07T16:29:26.3751143Z test [compile-fail] compile-fail/slice-too-big.rs ... ok
2019-09-07T16:29:26.3845345Z thread '
2019-09-07T16:29:26.3846701Z error: error pattern ' tried to deallocate Stack memory but gave Machine(Rust) as the kind' not found!
2019-09-07T16:29:26.3846832Z status: exit code: 1
2019-09-07T16:29:26.3848271Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/stack_free.rs" "-L" "/tmp/compiletest1nr2yu" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/tmp/compiletest1nr2yu/stack_free.stage-id" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zmiri-disable-validation" "-L" "/tmp/compiletest1nr2yu/stack_free.stage-id.aux" "-A" "unused"
2019-09-07T16:29:26.3848866Z ------------------------------------------
2019-09-07T16:29:26.3848945Z 
2019-09-07T16:29:26.3849235Z ------------------------------------------
2019-09-07T16:29:26.3849330Z stderr:
---
2019-09-07T16:29:26.3852347Z     |     ^^^^^^^^^^^^^
2019-09-07T16:29:26.3852425Z     = note: inside call to `main` at /checkout/src/libstd/rt.rs:64:34
2019-09-07T16:29:26.3852531Z     = note: inside call to closure at /checkout/src/libstd/rt.rs:52:53
2019-09-07T16:29:26.3852756Z     = note: inside call to closure at /checkout/src/libstd/panicking.rs:296:40
2019-09-07T16:29:26.3853492Z     = note: inside call to `std::panicking::try::do_call::<[closure@DefId(1:5902 ~ std[e07c]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /checkout/src/libstd/panicking.rs:292:5
2019-09-07T16:29:26.3854075Z     = note: inside call to `std::panicking::try::<i32, [closure@DefId(1:5902 ~ std[e07c]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe]>` at /checkout/src/libstd/panic.rs:394:9
2019-09-07T16:29:26.3854762Z     = note: inside call to `std::panic::catch_unwind::<[closure@DefId(1:5902 ~ std[e07c]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /checkout/src/libstd/rt.rs:52:25
2019-09-07T16:29:26.3855022Z     = note: inside call to `std::rt::lang_start::<()>`
2019-09-07T16:29:26.3855068Z 
2019-09-07T16:29:26.3855144Z error: aborting due to previous error
2019-09-07T16:29:26.3855184Z 
---
2019-09-07T16:29:33.7850473Z Verifying status of rustfmt...
2019-09-07T16:29:33.7864553Z Verifying status of clippy-driver...
2019-09-07T16:29:33.7876604Z This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
2019-09-07T16:29:33.7895875Z 
2019-09-07T16:29:33.7896695Z ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
2019-09-07T16:29:33.7897120Z 
2019-09-07T16:29:33.7898536Z If you do intend to update 'clippy-driver', please check the error messages above and
2019-09-07T16:29:33.7898692Z commit another update.
2019-09-07T16:29:33.7898731Z 
2019-09-07T16:29:33.7899691Z If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
2019-09-07T16:29:33.7900129Z change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
2019-09-07T16:29:33.7900249Z proper steps.
2019-09-07T16:29:33.7919337Z   local time: Sat Sep  7 16:29:33 UTC 2019
2019-09-07T16:29:34.0678523Z   network time: Sat, 07 Sep 2019 16:29:34 GMT
2019-09-07T16:29:34.0679839Z == end clock drift check ==
2019-09-07T16:29:34.0679839Z == end clock drift check ==
2019-09-07T16:29:34.7299236Z ##[error]Bash exited with code '3'.
2019-09-07T16:29:34.7342864Z ##[section]Starting: Upload CPU usage statistics
2019-09-07T16:29:34.7349369Z ==============================================================================
2019-09-07T16:29:34.7349471Z Task         : Bash
2019-09-07T16:29:34.7349532Z Description  : Run a Bash script on macOS, Linux, or Windows
