plain
2019-08-12T18:47:36.0123742Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-12T18:47:36.0312652Z ##[command]git config gc.auto 0
2019-08-12T18:47:36.0390109Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-12T18:47:36.0453496Z ##[command]git config --get-all http.proxy
2019-08-12T18:47:36.0595858Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63499/merge:refs/remotes/pull/63499/merge
---
2019-08-12T18:48:10.9257196Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-12T18:48:10.9257232Z 
2019-08-12T18:48:10.9257486Z   git checkout -b <new-branch-name>
2019-08-12T18:48:10.9257518Z 
2019-08-12T18:48:10.9257581Z HEAD is now at 31ea08eda Merge 9ed22c8c9ca5462753b6d10bc23ba167dd9c9c36 into 60960a260f7b5c695fd0717311d72ce62dd4eb43
2019-08-12T18:48:10.9439049Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-12T18:48:10.9442694Z ==============================================================================
2019-08-12T18:48:10.9442779Z Task         : Bash
2019-08-12T18:48:10.9442831Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-12T19:52:15.9293439Z .................................................................................................... 1300/8896
2019-08-12T19:52:22.5702012Z .................................................................................................... 1400/8896
2019-08-12T19:52:29.0112914Z .................................................................................................... 1500/8896
2019-08-12T19:52:40.0162279Z ....................................................................................i............... 1600/8896
2019-08-12T19:52:47.9000142Z i................................................................................................... 1700/8896
2019-08-12T19:52:54.9162089Z ...........................................................................iiiii.................... 1800/8896
2019-08-12T19:53:17.8113933Z .................................................................................................... 2000/8896
2019-08-12T19:53:20.3627124Z .................................................................................................... 2100/8896
2019-08-12T19:53:23.1729872Z .................................................................................................... 2200/8896
2019-08-12T19:53:31.2246534Z .................................................................................................... 2300/8896
---
2019-08-12T19:57:33.9581042Z .................................................................................................... 5300/8896
2019-08-12T19:57:41.4455863Z ........i........................................................................................... 5400/8896
2019-08-12T19:57:47.1605206Z .................................................................................................... 5500/8896
2019-08-12T19:57:59.7967277Z .................................................................................................... 5600/8896
2019-08-12T19:58:14.4249188Z ...ii...i..ii...........i........................................................................... 5700/8896
2019-08-12T19:58:31.8448031Z .................................................................................................... 5900/8896
2019-08-12T19:58:36.7280495Z .................................................................................................... 6000/8896
2019-08-12T19:58:36.7280495Z .................................................................................................... 6000/8896
2019-08-12T19:58:51.3152013Z ....i..ii........................................................................................... 6100/8896
2019-08-12T19:59:10.7997563Z ...............................................i.................................................... 6300/8896
2019-08-12T19:59:13.0384557Z .................................................................................................... 6400/8896
2019-08-12T19:59:15.5817968Z ...................i................................................................................ 6500/8896
2019-08-12T19:59:20.2534398Z .................................................................................................... 6600/8896
---
2019-08-12T20:03:28.2488915Z failures:
2019-08-12T20:03:28.2532270Z 
2019-08-12T20:03:28.2533349Z ---- [ui] ui/async-await/drop-order/drop-order-for-async-fn-parameters-by-ref-binding.rs stdout ----
2019-08-12T20:03:28.2533491Z 
2019-08-12T20:03:28.2533806Z error: test compilation failed although it shouldn't!
2019-08-12T20:03:28.2533865Z status: exit code: 101
2019-08-12T20:03:28.2535093Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/drop-order/drop-order-for-async-fn-parameters-by-ref-binding.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-order/drop-order-for-async-fn-parameters-by-ref-binding/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-order/drop-order-for-async-fn-parameters-by-ref-binding/auxiliary" "-A" "unused"
2019-08-12T20:03:28.2535565Z ------------------------------------------
2019-08-12T20:03:28.2535607Z 
2019-08-12T20:03:28.2535850Z ------------------------------------------
2019-08-12T20:03:28.2535916Z stderr:
2019-08-12T20:03:28.2535916Z stderr:
2019-08-12T20:03:28.2536153Z ------------------------------------------
2019-08-12T20:03:28.2536475Z thread 'rustc' panicked at 'index 1 out of range for slice of length 0', src/libcore/slice/mod.rs:2583:5
2019-08-12T20:03:28.2536749Z 
2019-08-12T20:03:28.2536799Z error: internal compiler error: unexpected panic
2019-08-12T20:03:28.2536831Z 
2019-08-12T20:03:28.2536897Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-12T20:03:28.2536897Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-12T20:03:28.2536931Z 
2019-08-12T20:03:28.2537421Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-12T20:03:28.2537762Z note: rustc 1.38.0-dev running on x86_64-unknown-linux-gnu
2019-08-12T20:03:28.2537798Z 
2019-08-12T20:03:28.2537798Z 
2019-08-12T20:03:28.2538104Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2019-08-12T20:03:28.2538191Z 
2019-08-12T20:03:28.2538427Z ------------------------------------------
2019-08-12T20:03:28.2538471Z 
2019-08-12T20:03:28.2538499Z 
2019-08-12T20:03:28.2538499Z 
2019-08-12T20:03:28.2538801Z ---- [ui] ui/async-await/drop-order/drop-order-for-async-fn-parameters.rs stdout ----
2019-08-12T20:03:28.2538903Z 
2019-08-12T20:03:28.2539148Z error: test compilation failed although it shouldn't!
2019-08-12T20:03:28.2539210Z status: exit code: 101
2019-08-12T20:03:28.2540151Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/drop-order/drop-order-for-async-fn-parameters.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-order/drop-order-for-async-fn-parameters/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-order/drop-order-for-async-fn-parameters/auxiliary" "-A" "unused"
2019-08-12T20:03:28.2540520Z ------------------------------------------
2019-08-12T20:03:28.2540557Z 
2019-08-12T20:03:28.2540810Z ------------------------------------------
2019-08-12T20:03:28.2540871Z stderr:
2019-08-12T20:03:28.2540871Z stderr:
2019-08-12T20:03:28.2541109Z ------------------------------------------
2019-08-12T20:03:28.2541419Z thread 'rustc' panicked at 'index 1 out of range for slice of length 0', src/libcore/slice/mod.rs:2583:5
2019-08-12T20:03:28.2541543Z 
2019-08-12T20:03:28.2541591Z error: internal compiler error: unexpected panic
2019-08-12T20:03:28.2541640Z 
2019-08-12T20:03:28.2541690Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-12T20:03:28.2541690Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-12T20:03:28.2541723Z 
2019-08-12T20:03:28.2542359Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-12T20:03:28.2542890Z note: rustc 1.38.0-dev running on x86_64-unknown-linux-gnu
2019-08-12T20:03:28.2542938Z 
2019-08-12T20:03:28.2542938Z 
2019-08-12T20:03:28.2543306Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2019-08-12T20:03:28.2543390Z 
2019-08-12T20:03:28.2543631Z ------------------------------------------
2019-08-12T20:03:28.2543665Z 
2019-08-12T20:03:28.2543708Z 
2019-08-12T20:03:28.2543708Z 
2019-08-12T20:03:28.2543983Z ---- [ui] ui/async-await/drop-order/drop-order-when-cancelled.rs stdout ----
2019-08-12T20:03:28.2544022Z 
2019-08-12T20:03:28.2544266Z error: test compilation failed although it shouldn't!
2019-08-12T20:03:28.2544338Z status: exit code: 101
2019-08-12T20:03:28.2545237Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/drop-order/drop-order-when-cancelled.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-order/drop-order-when-cancelled/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-order/drop-order-when-cancelled/auxiliary" "-A" "unused"
2019-08-12T20:03:28.2545809Z ------------------------------------------
2019-08-12T20:03:28.2545846Z 
2019-08-12T20:03:28.2546103Z ------------------------------------------
2019-08-12T20:03:28.2546153Z stderr:
2019-08-12T20:03:28.2546153Z stderr:
2019-08-12T20:03:28.2546385Z ------------------------------------------
2019-08-12T20:03:28.2546714Z thread 'rustc' panicked at 'index 1 out of range for slice of length 0', src/libcore/slice/mod.rs:2583:5
2019-08-12T20:03:28.2546871Z 
2019-08-12T20:03:28.2546934Z error: internal compiler error: unexpected panic
2019-08-12T20:03:28.2546977Z 
2019-08-12T20:03:28.2547027Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-12T20:03:28.2547027Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-12T20:03:28.2547060Z 
2019-08-12T20:03:28.2547435Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-12T20:03:28.2547757Z note: rustc 1.38.0-dev running on x86_64-unknown-linux-gnu
2019-08-12T20:03:28.2547809Z 
2019-08-12T20:03:28.2547809Z 
2019-08-12T20:03:28.2548118Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2019-08-12T20:03:28.2548187Z 
2019-08-12T20:03:28.2548435Z ------------------------------------------
2019-08-12T20:03:28.2548470Z 
2019-08-12T20:03:28.2548497Z 
2019-08-12T20:03:28.2548497Z 
2019-08-12T20:03:28.2549319Z ---- [ui] ui/async-await/issues/issue-58885.rs stdout ----
2019-08-12T20:03:28.2549364Z 
2019-08-12T20:03:28.2549673Z error: test compilation failed although it shouldn't!
2019-08-12T20:03:28.2549727Z status: exit code: 101
2019-08-12T20:03:28.2550584Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-58885.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-58885" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-58885/auxiliary" "-A" "unused"
2019-08-12T20:03:28.2550957Z ------------------------------------------
2019-08-12T20:03:28.2551009Z 
2019-08-12T20:03:28.2551251Z ------------------------------------------
2019-08-12T20:03:28.2551300Z stderr:
2019-08-12T20:03:28.2551300Z stderr:
2019-08-12T20:03:28.2551552Z ------------------------------------------
2019-08-12T20:03:28.2552120Z thread 'rustc' panicked at 'index 1 out of range for slice of length 0', src/libcore/slice/mod.rs:2583:5
2019-08-12T20:03:28.2552260Z 
2019-08-12T20:03:28.2552308Z error: internal compiler error: unexpected panic
2019-08-12T20:03:28.2552784Z 
2019-08-12T20:03:28.2552843Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-12T20:03:28.2552843Z note: the compiler unexpectedly panicked. this is a bug.
2019-08-12T20:03:28.2552892Z 
2019-08-12T20:03:28.2553347Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-08-12T20:03:28.2553678Z note: rustc 1.38.0-dev running on x86_64-unknown-linux-gnu
2019-08-12T20:03:28.2553715Z 
2019-08-12T20:03:28.2553715Z 
2019-08-12T20:03:28.2554023Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2019-08-12T20:03:28.2554109Z 
2019-08-12T20:03:28.2554346Z ------------------------------------------
2019-08-12T20:03:28.2554379Z 
2019-08-12T20:03:28.2554407Z 
---
2019-08-12T20:03:28.2578819Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-12T20:03:28.2578910Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-12T20:03:28.2595314Z 
2019-08-12T20:03:28.2596229Z 
2019-08-12T20:03:28.2600433Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-12T20:03:28.2601249Z 
2019-08-12T20:03:28.2601421Z 
2019-08-12T20:03:28.2608320Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-12T20:03:28.2608517Z Build completed unsuccessfully in 1:08:43
2019-08-12T20:03:28.2608517Z Build completed unsuccessfully in 1:08:43
2019-08-12T20:03:28.9987926Z ##[error]Bash exited with code '1'.
2019-08-12T20:03:29.0061851Z ##[section]Starting: Checkout
2019-08-12T20:03:29.0064703Z ==============================================================================
2019-08-12T20:03:29.0064783Z Task         : Get sources
2019-08-12T20:03:29.0064837Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
