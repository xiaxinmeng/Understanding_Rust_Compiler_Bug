plain
2019-11-16T02:45:21.5823170Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-16T02:45:21.6033160Z ##[command]git config gc.auto 0
2019-11-16T02:45:21.6132487Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-16T02:45:21.6188383Z ##[command]git config --get-all http.proxy
2019-11-16T02:45:21.6367276Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66461/merge:refs/remotes/pull/66461/merge
---
2019-11-16T03:46:16.6988323Z .................................................................................................... 1500/9244
2019-11-16T03:46:22.9966432Z .................................................................................................... 1600/9244
2019-11-16T03:46:32.4391462Z .................................................................................................... 1700/9244
2019-11-16T03:46:41.5415628Z .....i.............................................................................................. 1800/9244
2019-11-16T03:46:48.4940475Z .........................................................................................iiiii...... 1900/9244
2019-11-16T03:47:10.9024178Z .................................................................................................... 2100/9244
2019-11-16T03:47:13.4464137Z .................................................................................................... 2200/9244
2019-11-16T03:47:16.1568950Z .................................................................................................... 2300/9244
2019-11-16T03:47:22.8122291Z .................................................................................................... 2400/9244
---
2019-11-16T03:50:25.8241950Z ..........F.............................................................................i........... 4700/9244
2019-11-16T03:50:32.9433158Z ....i............................................................................................... 4800/9244
2019-11-16T03:50:42.7019069Z .................................................................................................... 4900/9244
2019-11-16T03:50:48.2384582Z .................................................................................................... 5000/9244
2019-11-16T03:50:59.2912122Z ............................................................................................ii.ii... 5100/9244
2019-11-16T03:51:08.3699550Z ............................i....................................................................... 5300/9244
2019-11-16T03:51:17.1186462Z .................................................................................................... 5400/9244
2019-11-16T03:51:25.9233158Z ..........................................................................i......................... 5500/9244
2019-11-16T03:51:33.8208711Z .................................................................................................... 5600/9244
2019-11-16T03:51:33.8208711Z .................................................................................................... 5600/9244
2019-11-16T03:51:40.8531040Z .................................................................................................... 5700/9244
2019-11-16T03:51:51.1558539Z ............................................................ii...i..ii...........i.................. 5800/9244
2019-11-16T03:52:14.5162466Z .................................................................................................... 6000/9244
2019-11-16T03:52:22.7481265Z .................................................................................................... 6100/9244
2019-11-16T03:52:22.7481265Z .................................................................................................... 6100/9244
2019-11-16T03:52:27.4694626Z ...............................................................................i..ii................ 6200/9244
2019-11-16T03:52:57.4497469Z ..........................F......................................................................... 6400/9244
2019-11-16T03:53:00.8262684Z ...............................................i.................................................... 6500/9244
2019-11-16T03:53:03.2044987Z .................................................................................................... 6600/9244
2019-11-16T03:53:05.7561417Z ..................................i................................................................. 6700/9244
---
2019-11-16T03:58:05.9074555Z 
2019-11-16T03:58:05.9074685Z 
2019-11-16T03:58:05.9074868Z The actual stderr differed from the expected stderr.
2019-11-16T03:58:05.9075667Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45730/issue-45730.stderr
2019-11-16T03:58:05.9076182Z To update references, rerun the tests and pass the `--bless` flag
2019-11-16T03:58:05.9077139Z To only update this specific test, also pass `--test-args issues/issue-45730.rs`
2019-11-16T03:58:05.9077601Z error: 1 errors occurred comparing output.
2019-11-16T03:58:05.9077786Z status: exit code: 1
2019-11-16T03:58:05.9077786Z status: exit code: 1
2019-11-16T03:58:05.9078833Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-45730.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45730" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45730/auxiliary" "-A" "unused"
2019-11-16T03:58:05.9079573Z ------------------------------------------
2019-11-16T03:58:05.9079757Z 
2019-11-16T03:58:05.9080157Z ------------------------------------------
2019-11-16T03:58:05.9080359Z stderr:
2019-11-16T03:58:05.9080359Z stderr:
2019-11-16T03:58:05.9080841Z ------------------------------------------
2019-11-16T03:58:05.9081093Z error[E0641]: cannot cast to a pointer of an unknown kind
2019-11-16T03:58:05.9081525Z   --> /checkout/src/test/ui/issues/issue-45730.rs:3:23
2019-11-16T03:58:05.9081896Z    |
2019-11-16T03:58:05.9082120Z LL |     let x: *const _ = 0 as _; //~ ERROR cannot cast
2019-11-16T03:58:05.9082799Z    |                            |
2019-11-16T03:58:05.9082968Z    |                            help: consider giving more type information
2019-11-16T03:58:05.9083154Z    |
2019-11-16T03:58:05.9083326Z    = note: The type information given here is insufficient to check whether the pointer cast is valid
2019-11-16T03:58:05.9083326Z    = note: The type information given here is insufficient to check whether the pointer cast is valid
2019-11-16T03:58:05.9083482Z 
2019-11-16T03:58:05.9083683Z error[E0641]: cannot cast to a pointer of an unknown kind
2019-11-16T03:58:05.9084119Z   --> /checkout/src/test/ui/issues/issue-45730.rs:5:23
2019-11-16T03:58:05.9084344Z    |
2019-11-16T03:58:05.9084506Z LL |     let x: *const _ = 0 as *const _; //~ ERROR cannot cast
2019-11-16T03:58:05.9085151Z    |                            |
2019-11-16T03:58:05.9085327Z    |                            help: consider giving more type information
2019-11-16T03:58:05.9085504Z    |
2019-11-16T03:58:05.9085674Z    = note: The type information given here is insufficient to check whether the pointer cast is valid
2019-11-16T03:58:05.9085674Z    = note: The type information given here is insufficient to check whether the pointer cast is valid
2019-11-16T03:58:05.9085820Z 
2019-11-16T03:58:05.9085982Z error[E0641]: cannot cast to a pointer of an unknown kind
2019-11-16T03:58:05.9086414Z   --> /checkout/src/test/ui/issues/issue-45730.rs:8:13
2019-11-16T03:58:05.9087010Z    |
2019-11-16T03:58:05.9087214Z LL |     let x = 0 as *const i32 as *const _ as *mut _; //~ ERROR cannot cast
2019-11-16T03:58:05.9087959Z    |                                            |
2019-11-16T03:58:05.9088140Z    |                                            help: consider giving more type information
2019-11-16T03:58:05.9088326Z    |
2019-11-16T03:58:05.9088521Z    = note: The type information given here is insufficient to check whether the pointer cast is valid
---
2019-11-16T03:58:05.9092887Z 13 
2019-11-16T03:58:05.9093030Z 
2019-11-16T03:58:05.9093165Z 
2019-11-16T03:58:05.9093329Z The actual stderr differed from the expected stderr.
2019-11-16T03:58:05.9095628Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/order-dependent-cast-inference/order-dependent-cast-inference.stderr
2019-11-16T03:58:05.9096037Z To update references, rerun the tests and pass the `--bless` flag
2019-11-16T03:58:05.9096402Z To only update this specific test, also pass `--test-args order-dependent-cast-inference.rs`
2019-11-16T03:58:05.9097054Z error: 1 errors occurred comparing output.
2019-11-16T03:58:05.9097133Z status: exit code: 1
2019-11-16T03:58:05.9097133Z status: exit code: 1
2019-11-16T03:58:05.9098247Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/order-dependent-cast-inference.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/order-dependent-cast-inference" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/order-dependent-cast-inference/auxiliary" "-A" "unused"
2019-11-16T03:58:05.9098686Z ------------------------------------------
2019-11-16T03:58:05.9098726Z 
2019-11-16T03:58:05.9098999Z ------------------------------------------
2019-11-16T03:58:05.9099049Z stderr:
2019-11-16T03:58:05.9099049Z stderr:
2019-11-16T03:58:05.9099282Z ------------------------------------------
2019-11-16T03:58:05.9099357Z error[E0641]: cannot cast to a pointer of an unknown kind
2019-11-16T03:58:05.9099645Z   --> /checkout/src/test/ui/order-dependent-cast-inference.rs:5:17
2019-11-16T03:58:05.9099701Z    |
2019-11-16T03:58:05.9099768Z LL |     let mut y = 0 as *const _;
2019-11-16T03:58:05.9100061Z    |                      |
2019-11-16T03:58:05.9100116Z    |                      help: consider giving more type information
2019-11-16T03:58:05.9100184Z    |
2019-11-16T03:58:05.9100251Z    = note: The type information given here is insufficient to check whether the pointer cast is valid
---
2019-11-16T03:58:05.9111352Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-16T03:58:05.9112736Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-16T03:58:05.9130705Z 
2019-11-16T03:58:05.9131175Z 
2019-11-16T03:58:05.9133138Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-16T03:58:05.9135477Z 
2019-11-16T03:58:05.9135657Z 
2019-11-16T03:58:05.9140933Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-16T03:58:05.9142135Z Build completed unsuccessfully in 1:06:34
2019-11-16T03:58:05.9142135Z Build completed unsuccessfully in 1:06:34
2019-11-16T03:58:05.9197101Z == clock drift check ==
2019-11-16T03:58:05.9210131Z   local time: Sat Nov 16 03:58:05 UTC 2019
2019-11-16T03:58:06.4568893Z   network time: Sat, 16 Nov 2019 03:58:06 GMT
2019-11-16T03:58:06.4571960Z == end clock drift check ==
2019-11-16T03:58:07.2439097Z 
2019-11-16T03:58:07.2511292Z ##[error]Bash exited with code '1'.
2019-11-16T03:58:07.2556393Z ##[section]Starting: Checkout
2019-11-16T03:58:07.2558712Z ==============================================================================
2019-11-16T03:58:07.2558774Z Task         : Get sources
2019-11-16T03:58:07.2558848Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
