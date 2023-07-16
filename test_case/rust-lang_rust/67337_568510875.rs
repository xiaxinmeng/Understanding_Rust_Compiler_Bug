plain
2019-12-23T14:48:40.5384394Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-23T14:48:40.5579767Z ##[command]git config gc.auto 0
2019-12-23T14:48:40.5656063Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-23T14:48:40.5700120Z ##[command]git config --get-all http.proxy
2019-12-23T14:48:40.5845230Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67337/merge:refs/remotes/pull/67337/merge
---
2019-12-23T15:45:02.4408080Z .................................................................................................... 1600/9430
2019-12-23T15:45:06.7527759Z .................................................................................................... 1700/9430
2019-12-23T15:45:16.3752853Z F.......................................................................................i........... 1800/9430
2019-12-23T15:45:23.2296325Z .................................................................................................... 1900/9430
2019-12-23T15:45:29.2521604Z .........................................................................iiiii...................... 2000/9430
2019-12-23T15:45:49.0379803Z .................................................................................................... 2200/9430
2019-12-23T15:45:51.2050683Z .................................................................................................... 2300/9430
2019-12-23T15:45:53.5963642Z .................................................................................................... 2400/9430
2019-12-23T15:46:04.1334923Z .................................................................................................... 2500/9430
---
2019-12-23T15:48:44.1522817Z ....i...............i............................................................................... 4900/9430
2019-12-23T15:48:53.5586247Z .................................................................................................... 5000/9430
2019-12-23T15:48:58.1041287Z ................................................i................................................... 5100/9430
2019-12-23T15:49:07.0322888Z .................................................................................................... 5200/9430
2019-12-23T15:49:12.6130035Z ...............ii.ii...........i.................................................................... 5300/9430
2019-12-23T15:49:21.0371366Z .................................................................................................... 5500/9430
2019-12-23T15:49:31.1481369Z .................................................................................................i.. 5600/9430
2019-12-23T15:49:38.7600312Z .................................................................................................... 5700/9430
2019-12-23T15:49:43.4322996Z .................................................................................................... 5800/9430
2019-12-23T15:49:43.4322996Z .................................................................................................... 5800/9430
2019-12-23T15:49:52.4676458Z .....................................................................................ii...i..ii..... 5900/9430
2019-12-23T15:50:12.9447817Z .................................................................................................... 6100/9430
2019-12-23T15:50:20.3420344Z .................................................................................................... 6200/9430
2019-12-23T15:50:27.8063149Z .................................................................................................... 6300/9430
2019-12-23T15:50:27.8063149Z .................................................................................................... 6300/9430
2019-12-23T15:50:47.2222685Z ............i..ii................................................................................... 6400/9430
2019-12-23T15:51:04.9130075Z ........................................................................................i........... 6600/9430
2019-12-23T15:51:06.8432804Z .................................................................................................... 6700/9430
2019-12-23T15:51:08.9292087Z ........................................................................................i........... 6800/9430
2019-12-23T15:51:11.4221100Z .................................................................................................... 6900/9430
---
2019-12-23T15:52:42.8224204Z .................................................................................................... 7500/9430
2019-12-23T15:52:47.4815127Z .................................................................................................... 7600/9430
2019-12-23T15:52:53.9124223Z .................................................................................................... 7700/9430
2019-12-23T15:53:03.8271356Z .................................................................................................... 7800/9430
2019-12-23T15:53:09.7163842Z .iiii............................................................................................... 7900/9430
2019-12-23T15:53:23.2277383Z .................................................................................................... 8100/9430
2019-12-23T15:53:34.0784784Z .................................................................................................... 8200/9430
2019-12-23T15:53:45.1979183Z .................................................................................................... 8300/9430
2019-12-23T15:53:50.4618678Z .................................................................................................... 8400/9430
---
2019-12-23T15:55:36.1797116Z 
2019-12-23T15:55:36.1797713Z ---- [ui] ui/consts/miri_unleashed/mutable_const2.rs stdout ----
2019-12-23T15:55:36.1797780Z diff of stderr:
2019-12-23T15:55:36.1797818Z 
2019-12-23T15:55:36.1798077Z 10 LL | const MUTABLE_BEHIND_RAW: *mut i32 = &UnsafeCell::new(42) as *const _ as *mut _;
2019-12-23T15:55:36.1798283Z 12 
2019-12-23T15:55:36.1798283Z 12 
2019-12-23T15:55:36.1798689Z - thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:347:17
2019-12-23T15:55:36.1799143Z + thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:349:17
2019-12-23T15:55:36.1800187Z 15 
2019-12-23T15:55:36.1800245Z 16 error: internal compiler error: unexpected panic
2019-12-23T15:55:36.1800302Z 
2019-12-23T15:55:36.1800615Z 
2019-12-23T15:55:36.1800615Z 
2019-12-23T15:55:36.1800687Z The actual stderr differed from the expected stderr.
2019-12-23T15:55:36.1801166Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_const2/mutable_const2.stderr
2019-12-23T15:55:36.1801893Z To update references, rerun the tests and pass the `--bless` flag
2019-12-23T15:55:36.1802418Z To only update this specific test, also pass `--test-args consts/miri_unleashed/mutable_const2.rs`
2019-12-23T15:55:36.1802763Z error: 1 errors occurred comparing output.
2019-12-23T15:55:36.1802814Z status: exit code: 101
2019-12-23T15:55:36.1802814Z status: exit code: 101
2019-12-23T15:55:36.1803865Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/mutable_const2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_const2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_const2/auxiliary" "-A" "unused"
2019-12-23T15:55:36.1804487Z ------------------------------------------
2019-12-23T15:55:36.1804527Z 
2019-12-23T15:55:36.1804970Z ------------------------------------------
2019-12-23T15:55:36.1805169Z stderr:
2019-12-23T15:55:36.1805169Z stderr:
2019-12-23T15:55:36.1805484Z ------------------------------------------
2019-12-23T15:55:36.1805683Z warning: skipping const checks
2019-12-23T15:55:36.1806035Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_const2.rs:15:38
2019-12-23T15:55:36.1806219Z    |
2019-12-23T15:55:36.1806326Z LL | const MUTABLE_BEHIND_RAW: *mut i32 = &UnsafeCell::new(42) as *const _ as *mut _;
2019-12-23T15:55:36.1806450Z 
2019-12-23T15:55:36.1806514Z error: internal compiler error: mutable allocation in constant
2019-12-23T15:55:36.1806952Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_const2.rs:15:1
2019-12-23T15:55:36.1807157Z    |
2019-12-23T15:55:36.1807157Z    |
2019-12-23T15:55:36.1807262Z LL | const MUTABLE_BEHIND_RAW: *mut i32 = &UnsafeCell::new(42) as *const _ as *mut _;
2019-12-23T15:55:36.1807382Z 
2019-12-23T15:55:36.1807382Z 
2019-12-23T15:55:36.1807786Z thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:349:17
2019-12-23T15:55:36.1808068Z 
2019-12-23T15:55:36.1808118Z error: internal compiler error: unexpected panic
2019-12-23T15:55:36.1808175Z 
2019-12-23T15:55:36.1808264Z note: the compiler unexpectedly panicked. this is a bug.
2019-12-23T15:55:36.1808264Z note: the compiler unexpectedly panicked. this is a bug.
2019-12-23T15:55:36.1808316Z 
2019-12-23T15:55:36.1808820Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-12-23T15:55:36.1808861Z 
2019-12-23T15:55:36.1809658Z note: rustc 1.42.0-nightly (4ded390fd 2019-12-23) running on x86_64-unknown-linux-gnu
2019-12-23T15:55:36.1809897Z 
2019-12-23T15:55:36.1810392Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -Z unleash-the-miri-inside-of-you -C prefer-dynamic -C rpath -C debuginfo=0
2019-12-23T15:55:36.1810633Z 
2019-12-23T15:55:36.1810981Z ------------------------------------------
2019-12-23T15:55:36.1811144Z 
2019-12-23T15:55:36.1811226Z 
---
2019-12-23T15:55:36.1830883Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2019-12-23T15:55:36.1830990Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-23T15:55:36.1847631Z 
2019-12-23T15:55:36.1848404Z 
2019-12-23T15:55:36.1850819Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-23T15:55:36.1851387Z 
2019-12-23T15:55:36.1851441Z 
2019-12-23T15:55:36.1865193Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-23T15:55:36.1865299Z Build completed unsuccessfully in 1:00:48
2019-12-23T15:55:36.1865299Z Build completed unsuccessfully in 1:00:48
2019-12-23T15:55:36.1921339Z == clock drift check ==
2019-12-23T15:55:36.1940165Z   local time: Mon Dec 23 15:55:36 UTC 2019
2019-12-23T15:55:36.4766259Z   network time: Mon, 23 Dec 2019 15:55:36 GMT
2019-12-23T15:55:36.4771871Z == end clock drift check ==
2019-12-23T15:55:37.4116269Z 
2019-12-23T15:55:37.4201711Z ##[error]Bash exited with code '1'.
2019-12-23T15:55:37.4246723Z ##[section]Starting: Checkout
2019-12-23T15:55:37.4248486Z ==============================================================================
2019-12-23T15:55:37.4248544Z Task         : Get sources
2019-12-23T15:55:37.4248592Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
