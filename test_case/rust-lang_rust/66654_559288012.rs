plain
2019-11-27T22:39:15.9017945Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-27T22:39:15.9198095Z ##[command]git config gc.auto 0
2019-11-27T22:39:15.9273760Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-27T22:39:15.9324970Z ##[command]git config --get-all http.proxy
2019-11-27T22:39:15.9467787Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66654/merge:refs/remotes/pull/66654/merge
---
2019-11-27T23:35:29.3664487Z .................................................................................................... 1600/9297
2019-11-27T23:35:34.2446687Z .................................................................................................... 1700/9297
2019-11-27T23:35:46.9183205Z ...................................i................................................................ 1800/9297
2019-11-27T23:35:54.5729627Z .................................................................................................... 1900/9297
2019-11-27T23:36:08.2199834Z ....................iiiii........................................................................... 2000/9297
2019-11-27T23:36:18.2957489Z .................................................................................................... 2200/9297
2019-11-27T23:36:20.8593812Z .................................................................................................... 2300/9297
2019-11-27T23:36:25.7541058Z .................................................................................................... 2400/9297
2019-11-27T23:36:46.5730561Z .................................................................................................... 2500/9297
---
2019-11-27T23:39:26.8554452Z ....................i...............i............................................................... 4800/9297
2019-11-27T23:39:36.8948556Z ......................F............................................................................. 4900/9297
2019-11-27T23:39:42.3375024Z .................................................................................................... 5000/9297
2019-11-27T23:39:50.4419040Z .................................................................................................... 5100/9297
2019-11-27T23:39:57.5701872Z .........................ii.ii...........i.......................................................... 5200/9297
2019-11-27T23:40:06.6494136Z .................................................................................................... 5400/9297
2019-11-27T23:40:17.2677575Z .................................................................................................... 5500/9297
2019-11-27T23:40:24.1436208Z .......i............................................................................................ 5600/9297
2019-11-27T23:40:30.3823101Z .................................................................................................... 5700/9297
2019-11-27T23:40:30.3823101Z .................................................................................................... 5700/9297
2019-11-27T23:40:40.8788029Z .............................................................................................ii...i. 5800/9297
2019-11-27T23:40:53.0136355Z .ii...........i..................................................................................... 5900/9297
2019-11-27T23:41:11.0379104Z .................................................................................................... 6100/9297
2019-11-27T23:41:16.4742387Z .................................................................................................... 6200/9297
2019-11-27T23:41:16.4742387Z .................................................................................................... 6200/9297
2019-11-27T23:41:30.3943050Z ................i..ii............................................................................... 6300/9297
2019-11-27T23:41:49.6879464Z ....................................................................................i............... 6500/9297
2019-11-27T23:41:51.9847155Z .................................................................................................... 6600/9297
2019-11-27T23:41:54.2636855Z ...........................................................................i........................ 6700/9297
2019-11-27T23:41:56.9761978Z .................................................................................................... 6800/9297
---
2019-11-27T23:46:41.2473327Z failures:
2019-11-27T23:46:41.2513123Z 
2019-11-27T23:46:41.2513885Z ---- [ui] ui/issues/issue-52060.rs stdout ----
2019-11-27T23:46:41.2514101Z 
2019-11-27T23:46:41.2514327Z error: Error: expected failure status (Some(1)) but received status Some(101).
2019-11-27T23:46:41.2514487Z status: exit code: 101
2019-11-27T23:46:41.2515532Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-52060.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52060" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52060/auxiliary" "-A" "unused"
2019-11-27T23:46:41.2516132Z ------------------------------------------
2019-11-27T23:46:41.2516292Z 
2019-11-27T23:46:41.2516628Z ------------------------------------------
2019-11-27T23:46:41.2516825Z stderr:
2019-11-27T23:46:41.2516825Z stderr:
2019-11-27T23:46:41.2517176Z ------------------------------------------
2019-11-27T23:46:41.2517357Z error[E0013]: constants cannot refer to statics, use a constant instead
2019-11-27T23:46:41.2517750Z   --> /checkout/src/test/ui/issues/issue-52060.rs:4:26
2019-11-27T23:46:41.2517929Z    |
2019-11-27T23:46:41.2518064Z LL | static B: [u32; 1] = [0; A.len()];
2019-11-27T23:46:41.2518580Z 
2019-11-27T23:46:41.2518580Z 
2019-11-27T23:46:41.2519001Z thread 'rustc' panicked at 'assertion failed: inner.is_empty()', src/librustc_mir/transform/check_consts/validation.rs:713:21
2019-11-27T23:46:41.2519401Z 
2019-11-27T23:46:41.2519559Z error: internal compiler error: unexpected panic
2019-11-27T23:46:41.2519909Z 
2019-11-27T23:46:41.2520056Z note: the compiler unexpectedly panicked. this is a bug.
2019-11-27T23:46:41.2520056Z note: the compiler unexpectedly panicked. this is a bug.
2019-11-27T23:46:41.2520177Z 
2019-11-27T23:46:41.2520919Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-11-27T23:46:41.2521495Z note: rustc 1.41.0-dev running on x86_64-unknown-linux-gnu
2019-11-27T23:46:41.2521673Z 
2019-11-27T23:46:41.2521673Z 
2019-11-27T23:46:41.2522076Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2019-11-27T23:46:41.2522397Z error: aborting due to previous error
2019-11-27T23:46:41.2522515Z 
2019-11-27T23:46:41.2522868Z For more information about this error, try `rustc --explain E0013`.
2019-11-27T23:46:41.2523041Z 
---
2019-11-27T23:46:41.2556827Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-27T23:46:41.2557173Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-27T23:46:41.2573774Z 
2019-11-27T23:46:41.2574416Z 
2019-11-27T23:46:41.2576746Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-27T23:46:41.2577029Z 
2019-11-27T23:46:41.2577080Z 
2019-11-27T23:46:41.2590223Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-27T23:46:41.2590338Z Build completed unsuccessfully in 1:01:36
2019-11-27T23:46:41.2590338Z Build completed unsuccessfully in 1:01:36
2019-11-27T23:46:41.2647475Z == clock drift check ==
2019-11-27T23:46:41.2664751Z   local time: Wed Nov 27 23:46:41 UTC 2019
2019-11-27T23:46:41.7894175Z   network time: Wed, 27 Nov 2019 23:46:41 GMT
2019-11-27T23:46:41.7894913Z == end clock drift check ==
2019-11-27T23:46:42.4700512Z 
2019-11-27T23:46:42.4794734Z ##[error]Bash exited with code '1'.
2019-11-27T23:46:42.4840262Z ##[section]Starting: Checkout
2019-11-27T23:46:42.4842100Z ==============================================================================
2019-11-27T23:46:42.4842156Z Task         : Get sources
2019-11-27T23:46:42.4842205Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
