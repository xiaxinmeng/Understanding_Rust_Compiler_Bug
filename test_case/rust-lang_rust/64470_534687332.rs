plain
2019-09-24T17:17:53.9901577Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-24T17:17:54.0076557Z ##[command]git config gc.auto 0
2019-09-24T17:17:54.0161266Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-24T17:17:54.0227170Z ##[command]git config --get-all http.proxy
2019-09-24T17:17:54.0354689Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64470/merge:refs/remotes/pull/64470/merge
---
2019-09-24T18:16:20.9833410Z .................................................................................................... 1500/9040
2019-09-24T18:16:26.4871351Z .................................................................................................... 1600/9040
2019-09-24T18:16:37.8372199Z ...........................................................................i...............i........ 1700/9040
2019-09-24T18:16:44.0153588Z .................................................................................................... 1800/9040
2019-09-24T18:16:51.3749611Z ..................................................................iiiii............................. 1900/9040
2019-09-24T18:17:10.2327324Z .................................................................................................... 2100/9040
2019-09-24T18:17:12.4696436Z .................................................................................................... 2200/9040
2019-09-24T18:17:15.3076755Z .................................................................................................... 2300/9040
2019-09-24T18:17:23.0263328Z .................................................................................................... 2400/9040
---
2019-09-24T18:20:07.8049155Z .......................................................i...............i............................ 4700/9040
2019-09-24T18:20:16.2755079Z .................................................................................................... 4800/9040
2019-09-24T18:20:24.2957705Z .................................................................................................... 4900/9040
2019-09-24T18:20:31.1802770Z .................................................................................................... 5000/9040
2019-09-24T18:20:40.1189020Z ..........................................ii.ii..................................................... 5100/9040
2019-09-24T18:20:49.3702871Z .................................................................................................... 5300/9040
2019-09-24T18:20:58.9916083Z .................................................................................................... 5400/9040
2019-09-24T18:21:05.8250746Z .......i............................................................................................ 5500/9040
2019-09-24T18:21:10.7966513Z .................................................................................................... 5600/9040
2019-09-24T18:21:10.7966513Z .................................................................................................... 5600/9040
2019-09-24T18:21:21.5458135Z .................................................................................................... 5700/9040
2019-09-24T18:21:33.4916922Z ..ii...i..ii...........i............................................................................ 5800/9040
2019-09-24T18:21:53.2226488Z .................................................................................................... 6000/9040
2019-09-24T18:22:01.3799718Z .................................................................................................... 6100/9040
2019-09-24T18:22:01.3799718Z .................................................................................................... 6100/9040
2019-09-24T18:22:14.3819584Z ....i..ii........................................................................................... 6200/9040
2019-09-24T18:22:31.9095644Z ................................................................i................................... 6400/9040
2019-09-24T18:22:33.8212098Z .................................................................................................... 6500/9040
2019-09-24T18:22:36.0783755Z ....................................i............................................................... 6600/9040
2019-09-24T18:22:39.7411010Z .................................................................................................... 6700/9040
---
2019-09-24T18:26:22.2084828Z normalized stderr:
2019-09-24T18:26:22.2084886Z warning: skipping const checks
2019-09-24T18:26:22.2085060Z   --> $DIR/const_fn_ptr_fail.rs:10:5
2019-09-24T18:26:22.2085095Z    |
2019-09-24T18:26:22.2085129Z LL |     X(x) // FIXME: this should error someday
2019-09-24T18:26:22.2085203Z 
2019-09-24T18:26:22.2085222Z 
2019-09-24T18:26:22.2085240Z 
2019-09-24T18:26:22.2085258Z 
2019-09-24T18:26:22.2085258Z 
2019-09-24T18:26:22.2086802Z The actual stderr differed from the expected stderr.
2019-09-24T18:26:22.2087272Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_fn_ptr_fail/const_fn_ptr_fail.stderr
2019-09-24T18:26:22.2087927Z To update references, rerun the tests and pass the `--bless` flag
2019-09-24T18:26:22.2088259Z To only update this specific test, also pass `--test-args consts/const-eval/const_fn_ptr_fail.rs`
2019-09-24T18:26:22.2088344Z error: 1 errors occurred comparing output.
2019-09-24T18:26:22.2088410Z status: exit code: 0
2019-09-24T18:26:22.2088410Z status: exit code: 0
2019-09-24T18:26:22.2089645Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_fn_ptr_fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_fn_ptr_fail/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_fn_ptr_fail/auxiliary"
2019-09-24T18:26:22.2090622Z ------------------------------------------
2019-09-24T18:26:22.2090822Z 
2019-09-24T18:26:22.2091061Z ------------------------------------------
2019-09-24T18:26:22.2091098Z stderr:
2019-09-24T18:26:22.2091098Z stderr:
2019-09-24T18:26:22.2091260Z ------------------------------------------
2019-09-24T18:26:22.2091316Z warning: skipping const checks
2019-09-24T18:26:22.2091506Z   --> /checkout/src/test/ui/consts/const-eval/const_fn_ptr_fail.rs:10:5
2019-09-24T18:26:22.2091545Z    |
2019-09-24T18:26:22.2091602Z LL |     X(x) // FIXME: this should error someday
2019-09-24T18:26:22.2091659Z 
2019-09-24T18:26:22.2091678Z 
2019-09-24T18:26:22.2091862Z ------------------------------------------
2019-09-24T18:26:22.2091888Z 
---
2019-09-24T18:26:22.2116950Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-09-24T18:26:22.2117034Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-24T18:26:22.2126746Z 
2019-09-24T18:26:22.2127029Z 
2019-09-24T18:26:22.2131900Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-24T18:26:22.2132305Z 
2019-09-24T18:26:22.2132333Z 
2019-09-24T18:26:22.2142110Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-24T18:26:22.2142220Z Build completed unsuccessfully in 1:01:22
2019-09-24T18:26:22.2142220Z Build completed unsuccessfully in 1:01:22
2019-09-24T18:26:22.2193028Z == clock drift check ==
2019-09-24T18:26:22.2207146Z   local time: Tue Sep 24 18:26:22 UTC 2019
2019-09-24T18:26:22.3705566Z   network time: Tue, 24 Sep 2019 18:26:22 GMT
2019-09-24T18:26:22.3708296Z == end clock drift check ==
2019-09-24T18:26:23.1792152Z ##[error]Bash exited with code '1'.
2019-09-24T18:26:23.1830565Z ##[section]Starting: Checkout
2019-09-24T18:26:23.1832498Z ==============================================================================
2019-09-24T18:26:23.1832560Z Task         : Get sources
2019-09-24T18:26:23.1832596Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
