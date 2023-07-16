plain
2020-02-04T03:27:04.6149117Z ========================== Starting Command Output ===========================
2020-02-04T03:27:04.6169863Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/281d4252-2b89-44f1-a612-a2e494560cce.sh
2020-02-04T03:27:04.6350827Z 
2020-02-04T03:27:04.6423256Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-04T03:27:04.6429315Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68795/merge to s
2020-02-04T03:27:04.6431044Z Task         : Get sources
2020-02-04T03:27:04.6431078Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-04T03:27:04.6431109Z Version      : 1.0.0
2020-02-04T03:27:04.6431184Z Author       : Microsoft
---
2020-02-04T03:27:05.5065277Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-04T03:27:05.5161172Z ##[command]git config gc.auto 0
2020-02-04T03:27:05.5243680Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-04T03:27:05.5311368Z ##[command]git config --get-all http.proxy
2020-02-04T03:27:05.5461151Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68795/merge:refs/remotes/pull/68795/merge
---
2020-02-04T04:22:42.3342155Z .................................................................................................... 1700/9582
2020-02-04T04:22:47.1794338Z .................................................................................................... 1800/9582
2020-02-04T04:22:58.9095636Z ............................i....................................................................... 1900/9582
2020-02-04T04:23:05.9169507Z .................................................................................................... 2000/9582
2020-02-04T04:23:19.7335521Z ..................iiiii............................................................................. 2100/9582
2020-02-04T04:23:29.0760728Z .................................................................................................... 2300/9582
2020-02-04T04:23:31.3963656Z .................................................................................................... 2400/9582
2020-02-04T04:23:36.0147019Z .................................................................................................... 2500/9582
2020-02-04T04:23:56.5944944Z .................................................................................................... 2600/9582
---
2020-02-04T04:26:28.0009767Z .............................................................i...............i...................... 4900/9582
2020-02-04T04:26:35.2767374Z .................................................................................................... 5000/9582
2020-02-04T04:26:43.0054011Z .................................................................................................... 5100/9582
2020-02-04T04:26:47.5400929Z ....i............................................................................................... 5200/9582
2020-02-04T04:26:58.1592069Z ..............................................................................ii.ii........i...i.... 5300/9582
2020-02-04T04:27:06.2601816Z ................i................................................................................... 5500/9582
2020-02-04T04:27:15.3421221Z .................................................................................................... 5600/9582
2020-02-04T04:27:21.9037634Z .................................................................i.................................. 5700/9582
2020-02-04T04:27:28.8224831Z .................................................................................................... 5800/9582
2020-02-04T04:27:28.8224831Z .................................................................................................... 5800/9582
2020-02-04T04:27:36.0162423Z .................................................................................................... 5900/9582
2020-02-04T04:27:44.9282744Z ........................................................ii...i..ii...........i...................... 6000/9582
2020-02-04T04:28:05.7936058Z .................................................................................................... 6200/9582
2020-02-04T04:28:13.1517587Z .................................................................................................... 6300/9582
2020-02-04T04:28:13.1517587Z .................................................................................................... 6300/9582
2020-02-04T04:28:20.8579858Z ....................................................................................i..ii........... 6400/9582
2020-02-04T04:28:44.9340041Z .................................................................................................... 6600/9582
2020-02-04T04:28:54.2818035Z .......................................................................i............................ 6700/9582
2020-02-04T04:28:56.3954768Z .................................................................................................... 6800/9582
2020-02-04T04:28:58.5481709Z .........................................................................i.......................... 6900/9582
---
2020-02-04T04:30:35.5802993Z .................................................................................................... 7600/9582
2020-02-04T04:30:40.1308322Z .................................................................................................... 7700/9582
2020-02-04T04:30:46.8380544Z .................................................................................................... 7800/9582
2020-02-04T04:30:54.9932722Z .................................................................................................... 7900/9582
2020-02-04T04:31:02.3087500Z ..................................iiiiiii.i......................................................... 8000/9582
2020-02-04T04:31:16.6356607Z .................................................................................................... 8200/9582
2020-02-04T04:31:24.7436223Z .................................................................................................... 8300/9582
2020-02-04T04:31:37.9037689Z .................................................................................................... 8400/9582
2020-02-04T04:31:45.2351541Z .................................................................................................... 8500/9582
---
2020-02-04T04:33:37.3392429Z ---- [ui] ui/async-await/async-closure.rs stdout ----
2020-02-04T04:33:37.3392729Z 
2020-02-04T04:33:37.3393265Z error: test compilation failed although it shouldn't!
2020-02-04T04:33:37.3393726Z status: exit code: 101
2020-02-04T04:33:37.3394884Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-closure/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-closure/auxiliary"
2020-02-04T04:33:37.3395911Z ------------------------------------------
2020-02-04T04:33:37.3396175Z 
2020-02-04T04:33:37.3396641Z ------------------------------------------
2020-02-04T04:33:37.3396931Z stderr:
2020-02-04T04:33:37.3396931Z stderr:
2020-02-04T04:33:37.3397771Z ------------------------------------------
2020-02-04T04:33:37.3398955Z error: internal compiler error: src/librustc/traits/codegen/mod.rs:57: Encountered error `OutputTypeParameterMismatch(Binder(<[closure@/checkout/src/test/ui/async-await/async-closure.rs:50:5: 53:7] as std::ops::Fn<(u8,)>>), Binder(<[closure@/checkout/src/test/ui/async-await/async-closure.rs:50:5: 53:7] as std::ops::Fn<(u8,)>>), Sorts(ExpectedFound { expected: impl std::future::Future, found: std::future::GenFuture<[static generator@/checkout/src/test/ui/async-await/async-closure.rs:50:31: 53:6 x:u8 {WakeOnceThenComplete, ()}]> }))` selecting `Binder(<[closure@/checkout/src/test/ui/async-await/async-closure.rs:50:5: 53:7] as std::ops::Fn<(u8,)>>)` during codegen
2020-02-04T04:33:37.3401485Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:883:9
2020-02-04T04:33:37.3401786Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-04T04:33:37.3401994Z 
2020-02-04T04:33:37.3402242Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-04T04:33:37.3402242Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-04T04:33:37.3402435Z 
2020-02-04T04:33:37.3403119Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-04T04:33:37.3403398Z 
2020-02-04T04:33:37.3403929Z note: rustc 1.42.0-nightly (981894c8b 2020-02-04) running on x86_64-unknown-linux-gnu
2020-02-04T04:33:37.3404205Z 
2020-02-04T04:33:37.3404813Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-04T04:33:37.3405328Z error: aborting due to previous error
2020-02-04T04:33:37.3405530Z 
2020-02-04T04:33:37.3405720Z 
2020-02-04T04:33:37.3406172Z ------------------------------------------
---
2020-02-04T04:33:37.3427278Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-04T04:33:37.3427598Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-04T04:33:37.3443507Z 
2020-02-04T04:33:37.3443881Z 
2020-02-04T04:33:37.3456224Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-04T04:33:37.3458649Z 
2020-02-04T04:33:37.3458881Z 
2020-02-04T04:33:37.3461594Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-04T04:33:37.3461687Z Build completed unsuccessfully in 1:00:58
2020-02-04T04:33:37.3461687Z Build completed unsuccessfully in 1:00:58
2020-02-04T04:33:37.3518567Z == clock drift check ==
2020-02-04T04:33:37.3535288Z   local time: Tue Feb  4 04:33:37 UTC 2020
2020-02-04T04:33:37.9049227Z   network time: Tue, 04 Feb 2020 04:33:37 GMT
2020-02-04T04:33:37.9049337Z == end clock drift check ==
2020-02-04T04:33:38.2815543Z 
2020-02-04T04:33:38.2916742Z ##[error]Bash exited with code '1'.
2020-02-04T04:33:38.2929182Z ##[section]Finishing: Run build
2020-02-04T04:33:38.2948636Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68795/merge to s
2020-02-04T04:33:38.2950451Z Task         : Get sources
2020-02-04T04:33:38.2950498Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-04T04:33:38.2950545Z Version      : 1.0.0
2020-02-04T04:33:38.2950600Z Author       : Microsoft
2020-02-04T04:33:38.2950600Z Author       : Microsoft
2020-02-04T04:33:38.2950646Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-04T04:33:38.2950696Z ==============================================================================
2020-02-04T04:33:38.7232111Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-04T04:33:38.7273025Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68795/merge to s
2020-02-04T04:33:38.7387658Z Cleaning up task key
2020-02-04T04:33:38.7388420Z Start cleaning up orphan processes.
2020-02-04T04:33:38.7506317Z Terminate orphan process: pid (3828) (python)
2020-02-04T04:33:38.7726167Z ##[section]Finishing: Finalize Job
