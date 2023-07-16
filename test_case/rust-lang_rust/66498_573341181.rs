plain
2020-01-11T17:09:23.5141524Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-11T17:09:23.5152753Z ##[command]git config gc.auto 0
2020-01-11T17:09:23.5155931Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-11T17:09:23.5158238Z ##[command]git config --get-all http.proxy
2020-01-11T17:09:23.5161205Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66498/merge:refs/remotes/pull/66498/merge
---
2020-01-11T18:10:29.8674929Z .........................................i...............i.......................................... 4900/9516
2020-01-11T18:10:39.3803091Z .................................................................................................... 5000/9516
2020-01-11T18:10:46.2140824Z ....................................................................................i............... 5100/9516
2020-01-11T18:10:52.0480812Z .................................................................................................... 5200/9516
2020-01-11T18:11:02.7756140Z .......................................................ii.ii...........i............................ 5300/9516
2020-01-11T18:11:12.5575190Z .................................................................................................... 5500/9516
2020-01-11T18:11:23.0342329Z .................................................................................................... 5600/9516
2020-01-11T18:11:29.7455262Z ........................................i........................................................... 5700/9516
2020-01-11T18:11:36.7693017Z .................................................................................................... 5800/9516
2020-01-11T18:11:36.7693017Z .................................................................................................... 5800/9516
2020-01-11T18:11:48.0864599Z .................................................................................................... 5900/9516
2020-01-11T18:11:58.2541859Z ...............................ii...i..ii...........i............................................... 6000/9516
2020-01-11T18:12:17.5417420Z .................................................................................................... 6200/9516
2020-01-11T18:12:25.8736325Z .................................................................................................... 6300/9516
2020-01-11T18:12:25.8736325Z .................................................................................................... 6300/9516
2020-01-11T18:12:37.5030174Z ..........................................................i..ii..................................... 6400/9516
2020-01-11T18:13:05.7851210Z .................................................................................................... 6600/9516
2020-01-11T18:13:07.9623376Z .................................i.................................................................. 6700/9516
2020-01-11T18:13:10.2774108Z .................................................................................................... 6800/9516
2020-01-11T18:13:12.9057432Z .................................i.................................................................. 6900/9516
---
2020-01-11T18:14:53.6564674Z .................................................................................................... 7500/9516
2020-01-11T18:14:58.1989074Z .................................................................................................... 7600/9516
2020-01-11T18:15:04.2861382Z .................................................................................................... 7700/9516
2020-01-11T18:15:11.6366822Z .................................................................................................... 7800/9516
2020-01-11T18:15:21.6817406Z ..................................................................................iiii.............. 7900/9516
2020-01-11T18:15:38.6752378Z ................i......i............................................................................ 8100/9516
2020-01-11T18:15:44.0455361Z .................................................................................................... 8200/9516
2020-01-11T18:15:58.7304792Z .................................................................................................... 8300/9516
2020-01-11T18:16:09.0138507Z .................................................................................................... 8400/9516
---
2020-01-11T18:18:14.6338027Z 
2020-01-11T18:18:14.6338585Z ---- [ui] ui/consts/miri_unleashed/mutable_const2.rs stdout ----
2020-01-11T18:18:14.6338696Z diff of stderr:
2020-01-11T18:18:14.6343312Z 
2020-01-11T18:18:14.6343842Z 10 LL | const MUTABLE_BEHIND_RAW: *mut i32 = &UnsafeCell::new(42) as *const _ as *mut _;
2020-01-11T18:18:14.6344700Z 12 
2020-01-11T18:18:14.6344700Z 12 
2020-01-11T18:18:14.6345377Z - thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:346:17
2020-01-11T18:18:14.6345759Z + thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:344:17
2020-01-11T18:18:14.6346094Z 15 
2020-01-11T18:18:14.6346151Z 16 error: internal compiler error: unexpected panic
2020-01-11T18:18:14.6349185Z 
2020-01-11T18:18:14.6349274Z 
2020-01-11T18:18:14.6349274Z 
2020-01-11T18:18:14.6349335Z The actual stderr differed from the expected stderr.
2020-01-11T18:18:14.6349948Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_const2/mutable_const2.stderr
2020-01-11T18:18:14.6350250Z To update references, rerun the tests and pass the `--bless` flag
2020-01-11T18:18:14.6350581Z To only update this specific test, also pass `--test-args consts/miri_unleashed/mutable_const2.rs`
2020-01-11T18:18:14.6350692Z error: 1 errors occurred comparing output.
2020-01-11T18:18:14.6350740Z status: exit code: 101
2020-01-11T18:18:14.6350740Z status: exit code: 101
2020-01-11T18:18:14.6351841Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/mutable_const2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_const2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_const2/auxiliary" "-A" "unused"
2020-01-11T18:18:14.6352359Z ------------------------------------------
2020-01-11T18:18:14.6352399Z 
2020-01-11T18:18:14.6352643Z ------------------------------------------
2020-01-11T18:18:14.6352708Z stderr:
2020-01-11T18:18:14.6352708Z stderr:
2020-01-11T18:18:14.6352947Z ------------------------------------------
2020-01-11T18:18:14.6353000Z warning: skipping const checks
2020-01-11T18:18:14.6353291Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_const2.rs:15:38
2020-01-11T18:18:14.6353358Z    |
2020-01-11T18:18:14.6353412Z LL | const MUTABLE_BEHIND_RAW: *mut i32 = &UnsafeCell::new(42) as *const _ as *mut _;
2020-01-11T18:18:14.6353518Z 
2020-01-11T18:18:14.6353575Z error: internal compiler error: mutable allocation in constant
2020-01-11T18:18:14.6353858Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_const2.rs:15:1
2020-01-11T18:18:14.6353925Z    |
2020-01-11T18:18:14.6353925Z    |
2020-01-11T18:18:14.6353978Z LL | const MUTABLE_BEHIND_RAW: *mut i32 = &UnsafeCell::new(42) as *const _ as *mut _;
2020-01-11T18:18:14.6354086Z 
2020-01-11T18:18:14.6354086Z 
2020-01-11T18:18:14.6354716Z thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:344:17
2020-01-11T18:18:14.6354842Z 
2020-01-11T18:18:14.6354908Z error: internal compiler error: unexpected panic
2020-01-11T18:18:14.6354938Z 
2020-01-11T18:18:14.6354985Z note: the compiler unexpectedly panicked. this is a bug.
2020-01-11T18:18:14.6354985Z note: the compiler unexpectedly panicked. this is a bug.
2020-01-11T18:18:14.6355014Z 
2020-01-11T18:18:14.6355478Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-01-11T18:18:14.6355521Z 
2020-01-11T18:18:14.6355814Z note: rustc 1.42.0-nightly (a281e5c8e 2020-01-11) running on x86_64-unknown-linux-gnu
2020-01-11T18:18:14.6355866Z 
2020-01-11T18:18:14.6356224Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z unleash-the-miri-inside-of-you -C prefer-dynamic -C rpath -C debuginfo=0
2020-01-11T18:18:14.6356314Z 
2020-01-11T18:18:14.6356536Z ------------------------------------------
2020-01-11T18:18:14.6356569Z 
2020-01-11T18:18:14.6356597Z 
---
2020-01-11T18:18:14.6362982Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:386:22
2020-01-11T18:18:14.6363064Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-11T18:18:14.6380908Z 
2020-01-11T18:18:14.6381195Z 
2020-01-11T18:18:14.6383564Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-11T18:18:14.6384595Z 
2020-01-11T18:18:14.6384894Z 
2020-01-11T18:18:14.6446252Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-11T18:18:14.6446336Z Build completed unsuccessfully in 1:02:50
2020-01-11T18:18:14.6446336Z Build completed unsuccessfully in 1:02:50
2020-01-11T18:18:14.6452903Z == clock drift check ==
2020-01-11T18:18:14.6474685Z   local time: Sat Jan 11 18:18:14 UTC 2020
2020-01-11T18:18:14.7449983Z   network time: Sat, 11 Jan 2020 18:18:14 GMT
2020-01-11T18:18:14.7450125Z == end clock drift check ==
2020-01-11T18:18:15.4226632Z 
2020-01-11T18:18:15.4330821Z ##[error]Bash exited with code '1'.
2020-01-11T18:18:15.4375595Z ##[section]Starting: Checkout
2020-01-11T18:18:15.4377470Z ==============================================================================
2020-01-11T18:18:15.4377534Z Task         : Get sources
2020-01-11T18:18:15.4377587Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
