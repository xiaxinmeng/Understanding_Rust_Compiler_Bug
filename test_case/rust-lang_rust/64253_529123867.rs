plain
2019-09-07T14:18:23.0688718Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-07T14:18:23.0881490Z ##[command]git config gc.auto 0
2019-09-07T14:18:23.0960563Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-07T14:18:23.1037124Z ##[command]git config --get-all http.proxy
2019-09-07T14:18:24.1116405Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64253/merge:refs/remotes/pull/64253/merge
---
2019-09-07T16:12:39.8054905Z    Compiling mdbook-linkcheck v0.3.0
2019-09-07T16:12:57.2285667Z    Compiling rustbook v0.1.0 (/checkout/src/tools/rustbook)
2019-09-07T16:13:01.8396984Z     Finished release [optimized] target(s) in 8m 43s
2019-09-07T16:14:33.8053134Z Error: there are broken links
2019-09-07T16:14:33.8053340Z  Caused By: "https://cs.au.dk/~amoeller/spa/" returned 503 Service Unavailable
2019-09-07T16:14:33.8090352Z 
2019-09-07T16:14:33.8091854Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rustbook" "linkcheck" "/checkout/src/doc/rustc-guide"
2019-09-07T16:14:33.8092015Z expected success, got: exit code: 101
2019-09-07T16:14:33.8092368Z 
---
2019-09-07T16:33:46.7484293Z 
2019-09-07T16:33:46.7486661Z error: error pattern ' tried to deallocate Stack memory but gave Machine(Rust) as the kind' not found!
2019-09-07T16:33:46.7492655Z test [compile-fail] compile-fail/slice-too-big.rs ... ok
2019-09-07T16:33:46.7492774Z status: exit code: 1
2019-09-07T16:33:46.7500917Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/stack_free.rs" "-L" "/tmp/compiletestSbZa3M" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/tmp/compiletestSbZa3M/stack_free.stage-id" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zmiri-disable-validation" "-L" "/tmp/compiletestSbZa3M/stack_free.stage-id.aux" "-A" "unused"
2019-09-07T16:33:46.7501349Z ------------------------------------------
2019-09-07T16:33:46.7501402Z 
2019-09-07T16:33:46.7501617Z ------------------------------------------
2019-09-07T16:33:46.7501659Z stderr:
---
2019-09-07T16:33:46.7511340Z     |     ^^^^^^^^^^^^^
2019-09-07T16:33:46.7511408Z     = note: inside call to `main` at /checkout/src/libstd/rt.rs:64:34
2019-09-07T16:33:46.7511453Z     = note: inside call to closure at /checkout/src/libstd/rt.rs:52:53
2019-09-07T16:33:46.7511507Z     = note: inside call to closure at /checkout/src/libstd/panicking.rs:296:40
2019-09-07T16:33:46.7511926Z     = note: inside call to `std::panicking::try::do_call::<[closure@DefId(1:5902 ~ std[e07c]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /checkout/src/libstd/panicking.rs:292:5
2019-09-07T16:33:46.7512325Z     = note: inside call to `std::panicking::try::<i32, [closure@DefId(1:5902 ~ std[e07c]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe]>` at /checkout/src/libstd/panic.rs:394:9
2019-09-07T16:33:46.7512736Z     = note: inside call to `std::panic::catch_unwind::<[closure@DefId(1:5902 ~ std[e07c]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /checkout/src/libstd/rt.rs:52:25
2019-09-07T16:33:46.7513196Z     = note: inside call to `std::rt::lang_start::<()>`
2019-09-07T16:33:46.7513233Z 
2019-09-07T16:33:46.7513272Z error: aborting due to previous error
2019-09-07T16:33:46.7513299Z 
---
2019-09-07T16:33:53.9663251Z This PR updated 'src/doc/rustc-guide', verifying if status is 'test-pass'...
2019-09-07T16:33:53.9678478Z 
2019-09-07T16:33:53.9679255Z ⚠️ We detected that this PR updated 'rustc-guide', but its tests failed.
2019-09-07T16:33:53.9679797Z 
2019-09-07T16:33:53.9680221Z If you do intend to update 'rustc-guide', please check the error messages above and
2019-09-07T16:33:53.9680416Z commit another update.
2019-09-07T16:33:53.9680714Z 
2019-09-07T16:33:53.9681085Z If you do NOT intend to update 'rustc-guide', please ensure you did not accidentally
2019-09-07T16:33:53.9681344Z change the submodule at 'src/doc/rustc-guide'. You may ask your reviewer for the
2019-09-07T16:33:53.9681560Z proper steps.
2019-09-07T16:33:53.9703165Z   local time: Sat Sep  7 16:33:53 UTC 2019
2019-09-07T16:33:54.1197710Z   network time: Sat, 07 Sep 2019 16:33:54 GMT
2019-09-07T16:33:54.1201604Z == end clock drift check ==
2019-09-07T16:33:54.1201604Z == end clock drift check ==
2019-09-07T16:33:54.7757933Z ##[error]Bash exited with code '3'.
2019-09-07T16:33:54.7800681Z ##[section]Starting: Checkout
2019-09-07T16:33:54.7802483Z ==============================================================================
2019-09-07T16:33:54.7802529Z Task         : Get sources
2019-09-07T16:33:54.7802569Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
