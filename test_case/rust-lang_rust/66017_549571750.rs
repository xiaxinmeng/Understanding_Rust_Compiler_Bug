plain
2019-11-04T21:00:01.9594777Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-04T21:00:01.9794374Z ##[command]git config gc.auto 0
2019-11-04T21:00:01.9878749Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-04T21:00:01.9945948Z ##[command]git config --get-all http.proxy
2019-11-04T21:00:02.0094304Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66017/merge:refs/remotes/pull/66017/merge
---
2019-11-04T22:01:16.3784005Z .................................................................................................... 1600/9276
2019-11-04T22:01:22.1760816Z .................................................................................................... 1700/9276
2019-11-04T22:01:35.0497290Z ..............................................................i...............i..................... 1800/9276
2019-11-04T22:01:42.5549053Z .................................................................................................... 1900/9276
2019-11-04T22:01:58.2437292Z ....................................................iiiii........................................... 2000/9276
2019-11-04T22:02:09.4343041Z .................................................................................................... 2200/9276
2019-11-04T22:02:12.0307920Z .................................................................................................... 2300/9276
2019-11-04T22:02:15.5633352Z .................................................................................................... 2400/9276
2019-11-04T22:02:39.5544987Z .................................................................................................... 2500/9276
2019-11-04T22:02:39.5544987Z .................................................................................................... 2500/9276
2019-11-04T22:02:42.4003116Z .................................................................................................... 2600/9276
2019-11-04T22:02:50.4237480Z .................................................................................................... 2700/9276
2019-11-04T22:02:59.6567255Z ....................i............................................................................... 2800/9276
2019-11-04T22:03:08.6213900Z .................................................................................................... 2900/9276
2019-11-04T22:03:13.3245934Z ...................i................................................................................ 3000/9276
2019-11-04T22:03:22.1647611Z .................................................................................................... 3100/9276
2019-11-04T22:03:27.9262213Z .................................................................................................... 3200/9276
2019-11-04T22:03:36.9409424Z .ii................................................................................................. 3300/9276
2019-11-04T22:03:54.2687887Z ..............................................................................................i..... 3500/9276
2019-11-04T22:04:01.9385631Z .........................................i.......................................................... 3600/9276
2019-11-04T22:04:08.6515267Z .................................................................................................... 3700/9276
2019-11-04T22:04:15.2426161Z .................................................................................................... 3800/9276
---
2019-11-04T22:05:34.5518844Z ....................................................i...............i............................... 4800/9276
2019-11-04T22:05:43.8218320Z .................................................................................................... 4900/9276
2019-11-04T22:05:52.6809376Z .................................................................................................... 5000/9276
2019-11-04T22:05:58.8421506Z .................................................................................................... 5100/9276
2019-11-04T22:06:09.6697751Z .....................................................Fii.ii...........i............................. 5200/9276
2019-11-04T22:06:19.9867639Z .................................................................................................... 5400/9276
2019-11-04T22:06:30.4704251Z .................................................................................................... 5500/9276
2019-11-04T22:06:38.0417351Z ...........................i........................................................................ 5600/9276
2019-11-04T22:06:44.7021796Z .................................................................................................... 5700/9276
2019-11-04T22:06:44.7021796Z .................................................................................................... 5700/9276
2019-11-04T22:06:57.0020735Z .................................................................................................... 5800/9276
2019-11-04T22:07:08.8417751Z ............ii...i..ii...........i.................................................................. 5900/9276
2019-11-04T22:07:29.5669220Z .................................................................................................... 6100/9276
2019-11-04T22:07:38.4662008Z .................................................................................................... 6200/9276
2019-11-04T22:07:38.4662008Z .................................................................................................... 6200/9276
2019-11-04T22:07:52.7849490Z ...............................i..ii................................................................ 6300/9276
2019-11-04T22:08:14.1505153Z ..................................................................................................i. 6500/9276
2019-11-04T22:08:16.4551995Z .................................................................................................... 6600/9276
2019-11-04T22:08:18.7783594Z ............................................................................i....................... 6700/9276
2019-11-04T22:08:21.6294049Z .................................................................................................... 6800/9276
---
2019-11-04T22:13:22.3675500Z failures:
2019-11-04T22:13:22.3718710Z 
2019-11-04T22:13:22.3719238Z ---- [ui] ui/iterators/into-iter-on-arrays-lint.rs stdout ----
2019-11-04T22:13:22.3719281Z 
2019-11-04T22:13:22.3719700Z error: /checkout/src/test/ui/iterators/into-iter-on-arrays-lint.rs:9: unexpected warning: '9:11: 9:20: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!'
2019-11-04T22:13:22.3719767Z 
2019-11-04T22:13:22.3720157Z error: /checkout/src/test/ui/iterators/into-iter-on-arrays-lint.rs:11: unexpected warning: '11:12: 11:21: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!'
2019-11-04T22:13:22.3720225Z 
2019-11-04T22:13:22.3720597Z error: /checkout/src/test/ui/iterators/into-iter-on-arrays-lint.rs:13: unexpected warning: '13:9: 13:18: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!'
2019-11-04T22:13:22.3720644Z 
2019-11-04T22:13:22.3721031Z error: /checkout/src/test/ui/iterators/into-iter-on-arrays-lint.rs:15: unexpected warning: '15:15: 15:24: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!'
2019-11-04T22:13:22.3721132Z error: 4 unexpected errors found, 0 expected errors not found
2019-11-04T22:13:22.3721195Z status: exit code: 0
2019-11-04T22:13:22.3721195Z status: exit code: 0
2019-11-04T22:13:22.3722359Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/iterators/into-iter-on-arrays-lint.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/into-iter-on-arrays-lint/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/into-iter-on-arrays-lint/auxiliary"
2019-11-04T22:13:22.3722486Z unexpected errors (from JSON output): [
2019-11-04T22:13:22.3722596Z         line_num: 9,
2019-11-04T22:13:22.3722638Z         kind: Some(
2019-11-04T22:13:22.3722689Z             Warning,
2019-11-04T22:13:22.3722729Z         ),
2019-11-04T22:13:22.3722729Z         ),
2019-11-04T22:13:22.3722804Z         msg: "9:11: 9:20: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!",
2019-11-04T22:13:22.3722906Z     Error {
2019-11-04T22:13:22.3722965Z         line_num: 11,
2019-11-04T22:13:22.3723006Z         kind: Some(
2019-11-04T22:13:22.3723048Z             Warning,
2019-11-04T22:13:22.3723048Z             Warning,
2019-11-04T22:13:22.3723105Z         ),
2019-11-04T22:13:22.3723163Z         msg: "11:12: 11:21: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!",
2019-11-04T22:13:22.3723271Z     Error {
2019-11-04T22:13:22.3723312Z         line_num: 13,
2019-11-04T22:13:22.3723353Z         kind: Some(
2019-11-04T22:13:22.3723410Z             Warning,
2019-11-04T22:13:22.3723410Z             Warning,
2019-11-04T22:13:22.3723450Z         ),
2019-11-04T22:13:22.3723507Z         msg: "13:9: 13:18: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!",
2019-11-04T22:13:22.3723787Z     Error {
2019-11-04T22:13:22.3723829Z         line_num: 15,
2019-11-04T22:13:22.3723938Z         kind: Some(
2019-11-04T22:13:22.3724004Z             Warning,
2019-11-04T22:13:22.3724004Z             Warning,
2019-11-04T22:13:22.3724043Z         ),
2019-11-04T22:13:22.3724099Z         msg: "15:15: 15:24: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!",
2019-11-04T22:13:22.3724205Z ]
2019-11-04T22:13:22.3724232Z 
2019-11-04T22:13:22.3724637Z thread '[ui] ui/iterators/into-iter-on-arrays-lint.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1520:13
2019-11-04T22:13:22.3724721Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
2019-11-04T22:13:22.3725411Z 
2019-11-04T22:13:22.3757805Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-04T22:13:22.3773215Z 
2019-11-04T22:13:22.3773296Z 
2019-11-04T22:13:22.3778297Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-04T22:13:22.3778561Z 
2019-11-04T22:13:22.3778609Z 
2019-11-04T22:13:22.3784579Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-04T22:13:22.3784649Z Build completed unsuccessfully in 1:06:26
2019-11-04T22:13:22.3784649Z Build completed unsuccessfully in 1:06:26
2019-11-04T22:13:22.3837597Z == clock drift check ==
2019-11-04T22:13:22.3852274Z   local time: Mon Nov  4 22:13:22 UTC 2019
2019-11-04T22:13:22.6645465Z   network time: Mon, 04 Nov 2019 22:13:22 GMT
2019-11-04T22:13:22.6652871Z == end clock drift check ==
2019-11-04T22:13:24.0745245Z 
2019-11-04T22:13:24.0861388Z ##[error]Bash exited with code '1'.
2019-11-04T22:13:24.0896477Z ##[section]Starting: Checkout
2019-11-04T22:13:24.0898466Z ==============================================================================
2019-11-04T22:13:24.0898527Z Task         : Get sources
2019-11-04T22:13:24.0898579Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
