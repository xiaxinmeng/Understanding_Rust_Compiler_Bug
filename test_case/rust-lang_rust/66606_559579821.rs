plain
2019-11-28T17:21:32.3339713Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-28T17:21:32.3533460Z ##[command]git config gc.auto 0
2019-11-28T17:21:32.3587559Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-28T17:21:32.3644609Z ##[command]git config --get-all http.proxy
2019-11-28T17:21:32.3795844Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66606/merge:refs/remotes/pull/66606/merge
---
2019-11-28T18:19:03.7090739Z .................................................................................................... 1600/9308
2019-11-28T18:19:08.1992766Z .................................................................................................... 1700/9308
2019-11-28T18:19:20.4318635Z ........................................i........................................................... 1800/9308
2019-11-28T18:19:28.1560947Z .................................................................................................... 1900/9308
2019-11-28T18:19:41.4575442Z .........................iiiii...................................................................... 2000/9308
2019-11-28T18:19:51.2708035Z .................................................................................................... 2200/9308
2019-11-28T18:19:53.6801878Z .................................................................................................... 2300/9308
2019-11-28T18:19:58.0762386Z .................................................................................................... 2400/9308
2019-11-28T18:20:19.0664206Z .................................................................................................... 2500/9308
---
2019-11-28T18:22:54.2137357Z ..........................i...............i......................................................... 4800/9308
2019-11-28T18:23:04.2654789Z .................................................................................................... 4900/9308
2019-11-28T18:23:10.0167307Z .................................................................................................... 5000/9308
2019-11-28T18:23:17.7586892Z .................................................................................................... 5100/9308
2019-11-28T18:23:25.1216054Z ...............................ii.ii...........i.................................................... 5200/9308
2019-11-28T18:23:34.1710800Z .................................................................................................... 5400/9308
2019-11-28T18:23:43.9146915Z .................................................................................................... 5500/9308
2019-11-28T18:23:50.8331886Z .............i...................................................................................... 5600/9308
2019-11-28T18:23:56.7452015Z .................................................................................................... 5700/9308
2019-11-28T18:23:56.7452015Z .................................................................................................... 5700/9308
2019-11-28T18:24:07.6006079Z ...................................................................................................i 5800/9308
2019-11-28T18:24:19.4105889Z i...i..ii...........i............................................................................... 5900/9308
2019-11-28T18:24:36.9358889Z .................................................................................................... 6100/9308
2019-11-28T18:24:42.3566202Z .................................................................................................... 6200/9308
2019-11-28T18:24:42.3566202Z .................................................................................................... 6200/9308
2019-11-28T18:24:55.5512197Z ......................i..ii......................................................................... 6300/9308
2019-11-28T18:25:14.7484456Z ..........................................................................................i......... 6500/9308
2019-11-28T18:25:16.8878452Z .................................................................................................... 6600/9308
2019-11-28T18:25:18.9568845Z .................................................................................i.................. 6700/9308
2019-11-28T18:25:21.4688957Z .................................................................................................... 6800/9308
---
2019-11-28T18:29:59.3312328Z failures:
2019-11-28T18:29:59.3318335Z 
2019-11-28T18:29:59.3319310Z ---- [compile-fail] compile-fail/consts/const-fn-error.rs stdout ----
2019-11-28T18:29:59.3319348Z 
2019-11-28T18:29:59.3320169Z error: /checkout/src/test/compile-fail/consts/const-fn-error.rs:7: unexpected error: '7:14: 7:18: references in constant functions may only refer to immutable values [E0658]'
2019-11-28T18:29:59.3321186Z error: /checkout/src/test/compile-fail/consts/const-fn-error.rs:7: expected error not found: E0017
2019-11-28T18:29:59.3321391Z 
2019-11-28T18:29:59.3321550Z error: 1 unexpected errors found, 1 expected errors not found
2019-11-28T18:29:59.3321601Z status: exit code: 1
2019-11-28T18:29:59.3321601Z status: exit code: 1
2019-11-28T18:29:59.3322659Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/consts/const-fn-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/consts/const-fn-error" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/consts/const-fn-error/auxiliary" "-A" "unused"
2019-11-28T18:29:59.3322951Z unexpected errors (from JSON output): [
2019-11-28T18:29:59.3323170Z         line_num: 7,
2019-11-28T18:29:59.3323241Z         kind: Some(
2019-11-28T18:29:59.3323325Z             Error,
2019-11-28T18:29:59.3323534Z         ),
2019-11-28T18:29:59.3323534Z         ),
2019-11-28T18:29:59.3323602Z         msg: "7:14: 7:18: references in constant functions may only refer to immutable values [E0658]",
2019-11-28T18:29:59.3323994Z ]
2019-11-28T18:29:59.3324043Z 
2019-11-28T18:29:59.3324079Z not found errors (from test file): [
2019-11-28T18:29:59.3324153Z     Error {
---
2019-11-28T18:29:59.3326729Z 
2019-11-28T18:29:59.3329086Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-28T18:29:59.3331357Z 
2019-11-28T18:29:59.3331418Z 
2019-11-28T18:29:59.3333226Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-28T18:29:59.3334340Z 
2019-11-28T18:29:59.3334386Z 
2019-11-28T18:29:59.3339024Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-28T18:29:59.3339082Z Build completed unsuccessfully in 1:02:31
2019-11-28T18:29:59.3339082Z Build completed unsuccessfully in 1:02:31
2019-11-28T18:29:59.3391772Z == clock drift check ==
2019-11-28T18:29:59.3407996Z   local time: Thu Nov 28 18:29:59 UTC 2019
2019-11-28T18:29:59.6194981Z   network time: Thu, 28 Nov 2019 18:29:59 GMT
2019-11-28T18:29:59.6199854Z == end clock drift check ==
2019-11-28T18:30:00.4305469Z 
2019-11-28T18:30:00.4411715Z ##[error]Bash exited with code '1'.
2019-11-28T18:30:00.4445687Z ##[section]Starting: Checkout
2019-11-28T18:30:00.4447528Z ==============================================================================
2019-11-28T18:30:00.4447571Z Task         : Get sources
2019-11-28T18:30:00.4447621Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
