plain
2019-09-07T15:38:10.4552781Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-07T15:38:10.4761395Z ##[command]git config gc.auto 0
2019-09-07T15:38:10.4849885Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-07T15:38:10.4933624Z ##[command]git config --get-all http.proxy
2019-09-07T15:38:10.5081313Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64257/merge:refs/remotes/pull/64257/merge
---
2019-09-07T17:36:07.0726086Z    Compiling mdbook-linkcheck v0.3.0
2019-09-07T17:36:26.1479512Z    Compiling rustbook v0.1.0 (/checkout/src/tools/rustbook)
2019-09-07T17:36:30.9565478Z     Finished release [optimized] target(s) in 8m 59s
2019-09-07T17:38:23.9753280Z Error: there are broken links
2019-09-07T17:38:23.9756269Z  Caused By: "https://cs.au.dk/~amoeller/spa/" returned 503 Service Unavailable
2019-09-07T17:38:23.9760734Z 
2019-09-07T17:38:23.9761829Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rustbook" "linkcheck" "/checkout/src/doc/rustc-guide"
2019-09-07T17:38:23.9762072Z expected success, got: exit code: 101
2019-09-07T17:38:23.9762209Z 
---
2019-09-07T17:58:13.1930087Z test [compile-fail] compile-fail/reallocate-dangling.rs ... ok
2019-09-07T17:58:13.3567273Z 
2019-09-07T17:58:13.3573368Z error: error pattern ' tried to deallocate Stack memory but gave Machine(Rust) as the kind' not found!
2019-09-07T17:58:13.3577775Z status: exit code: 1
2019-09-07T17:58:13.3584898Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/stack_free.rs" "-L" "/tmp/compiletestMNNPGd" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/tmp/compiletestMNNPGd/stack_free.stage-id" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zmiri-disable-validation" "-L" "/tmp/compiletestMNNPGd/stack_free.stage-id.aux" "-A" "unused"
2019-09-07T17:58:13.3585314Z ------------------------------------------
2019-09-07T17:58:13.3592785Z 
2019-09-07T17:58:13.3593597Z ------------------------------------------
2019-09-07T17:58:13.3593923Z stderr:
---
2019-09-07T17:58:13.3600549Z     |     ^^^^^^^^^^^^^
2019-09-07T17:58:13.3600776Z     = note: inside call to `main` at /checkout/src/libstd/rt.rs:64:34
2019-09-07T17:58:13.3605478Z     = note: inside call to closure at /checkout/src/libstd/rt.rs:52:53
2019-09-07T17:58:13.3615880Z     = note: inside call to closure at /checkout/src/libstd/panicking.rs:296:40
2019-09-07T17:58:13.3616974Z     = note: inside call to `std::panicking::try::do_call::<[closure@DefId(1:5902 ~ std[af4c]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /checkout/src/libstd/panicking.rs:292:5
2019-09-07T17:58:13.3617936Z     = note: inside call to `std::panicking::try::<i32, [closure@DefId(1:5902 ~ std[af4c]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe]>` at /checkout/src/libstd/panic.rs:394:9
2019-09-07T17:58:13.3618897Z     = note: inside call to `std::panic::catch_unwind::<[closure@DefId(1:5902 ~ std[af4c]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /checkout/src/libstd/rt.rs:52:25
2019-09-07T17:58:13.3619537Z     = note: inside call to `std::rt::lang_start::<()>`
2019-09-07T17:58:13.3619775Z 
2019-09-07T17:58:13.3619991Z error: aborting due to previous error
2019-09-07T17:58:13.3620186Z 
---
2019-09-07T17:58:20.8984807Z This PR updated 'src/doc/rustc-guide', verifying if status is 'test-pass'...
2019-09-07T17:58:20.8994287Z 
2019-09-07T17:58:20.8996178Z ⚠️ We detected that this PR updated 'rustc-guide', but its tests failed.
2019-09-07T17:58:20.8996646Z 
2019-09-07T17:58:20.8997269Z If you do intend to update 'rustc-guide', please check the error messages above and
2019-09-07T17:58:20.8997572Z commit another update.
2019-09-07T17:58:20.8997862Z 
2019-09-07T17:58:20.8998548Z If you do NOT intend to update 'rustc-guide', please ensure you did not accidentally
2019-09-07T17:58:20.8999126Z change the submodule at 'src/doc/rustc-guide'. You may ask your reviewer for the
2019-09-07T17:58:20.8999451Z proper steps.
2019-09-07T17:58:20.9017449Z   local time: Sat Sep  7 17:58:20 UTC 2019
2019-09-07T17:58:21.0653358Z   network time: Sat, 07 Sep 2019 17:58:21 GMT
2019-09-07T17:58:21.0656043Z == end clock drift check ==
2019-09-07T17:58:21.0656043Z == end clock drift check ==
2019-09-07T17:58:21.6397875Z ##[error]Bash exited with code '3'.
2019-09-07T17:58:21.6448320Z ##[section]Starting: Checkout
2019-09-07T17:58:21.6450572Z ==============================================================================
2019-09-07T17:58:21.6450633Z Task         : Get sources
2019-09-07T17:58:21.6450706Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
