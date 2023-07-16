plain
2019-11-13T21:01:39.2189350Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-13T21:01:39.9941145Z ##[command]git config gc.auto 0
2019-11-13T21:01:39.9945513Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-13T21:01:39.9949477Z ##[command]git config --get-all http.proxy
2019-11-13T21:01:39.9952946Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66385/merge:refs/remotes/pull/66385/merge
---
2019-11-13T22:00:36.4127090Z .................................................................................................... 1500/9239
2019-11-13T22:00:42.0827223Z .................................................................................................... 1600/9239
2019-11-13T22:00:50.7962573Z .................................................................................................... 1700/9239
2019-11-13T22:00:58.8414971Z ....i............................................................................................... 1800/9239
2019-11-13T22:01:05.1416842Z ........................................................................................iiiii....... 1900/9239
2019-11-13T22:01:25.5130430Z .................................................................................................... 2100/9239
2019-11-13T22:01:27.6570302Z .................................................................................................... 2200/9239
2019-11-13T22:01:29.9643268Z .................................................................................................... 2300/9239
2019-11-13T22:01:36.4529356Z .................................................................................................... 2400/9239
---
2019-11-13T22:04:21.8011313Z .......................................................................................i............ 4700/9239
2019-11-13T22:04:28.1041664Z ...i................................................................................................ 4800/9239
2019-11-13T22:04:36.9297027Z .................................................................................................... 4900/9239
2019-11-13T22:04:41.8953179Z .................................................................................................... 5000/9239
2019-11-13T22:04:52.3207071Z ..........................................................................................ii.ii..... 5100/9239
2019-11-13T22:05:00.2334857Z .........................i.......................................................................... 5300/9239
2019-11-13T22:05:08.4053552Z .................................................................................................... 5400/9239
2019-11-13T22:05:16.6923788Z ........................................................................i........................... 5500/9239
2019-11-13T22:05:23.6617888Z .................................................................................................... 5600/9239
2019-11-13T22:05:23.6617888Z .................................................................................................... 5600/9239
2019-11-13T22:05:30.3608720Z .................................................................................................... 5700/9239
2019-11-13T22:05:39.8176486Z ..........................................................ii...i..ii...........i.................... 5800/9239
2019-11-13T22:06:00.9889955Z .................................................................................................... 6000/9239
2019-11-13T22:06:08.8532248Z .................................................................................................... 6100/9239
2019-11-13T22:06:08.8532248Z .................................................................................................... 6100/9239
2019-11-13T22:06:13.5810112Z .............................................................................i..ii.................. 6200/9239
2019-11-13T22:06:40.8735799Z .................................................................................................... 6400/9239
2019-11-13T22:06:43.5940289Z .............................................i...................................................... 6500/9239
2019-11-13T22:06:45.6330862Z .................................................................................................... 6600/9239
2019-11-13T22:06:47.8021106Z .............................i...................................................................... 6700/9239
---
2019-11-13T22:11:23.0549559Z failures:
2019-11-13T22:11:23.0549807Z 
2019-11-13T22:11:23.0550399Z ---- [compile-fail] compile-fail/consts/const-fn-error.rs stdout ----
2019-11-13T22:11:23.0550648Z 
2019-11-13T22:11:23.0556079Z error: /checkout/src/test/compile-fail/consts/const-fn-error.rs:7: unexpected error: '7:9: 7:10: constant function contains unimplemented expression type [E0019]'
2019-11-13T22:11:23.0557156Z error: 1 unexpected errors found, 0 expected errors not found
2019-11-13T22:11:23.0557230Z status: exit code: 1
2019-11-13T22:11:23.0557230Z status: exit code: 1
2019-11-13T22:11:23.0558192Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/consts/const-fn-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/consts/const-fn-error" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/consts/const-fn-error/auxiliary" "-A" "unused"
2019-11-13T22:11:23.0559858Z unexpected errors (from JSON output): [
2019-11-13T22:11:23.0565412Z         line_num: 7,
2019-11-13T22:11:23.0565454Z         kind: Some(
2019-11-13T22:11:23.0565493Z             Error,
2019-11-13T22:11:23.0565550Z         ),
2019-11-13T22:11:23.0565550Z         ),
2019-11-13T22:11:23.0565594Z         msg: "7:9: 7:10: constant function contains unimplemented expression type [E0019]",
2019-11-13T22:11:23.0565690Z ]
2019-11-13T22:11:23.0565715Z 
2019-11-13T22:11:23.0566180Z thread '[compile-fail] compile-fail/consts/const-fn-error.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1520:13
2019-11-13T22:11:23.0566260Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-13T22:11:23.0566260Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-13T22:11:23.0566293Z 
2019-11-13T22:11:23.0566514Z ---- [compile-fail] compile-fail/issue-52443.rs stdout ----
2019-11-13T22:11:23.0566544Z 
2019-11-13T22:11:23.0566848Z error: /checkout/src/test/compile-fail/issue-52443.rs:9: unexpected error: '9:16: 9:17: constant contains unimplemented expression type [E0019]'
2019-11-13T22:11:23.0566932Z error: 1 unexpected errors found, 0 expected errors not found
2019-11-13T22:11:23.0566993Z status: exit code: 1
2019-11-13T22:11:23.0566993Z status: exit code: 1
2019-11-13T22:11:23.0567668Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/issue-52443.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-52443" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/issue-52443/auxiliary" "-A" "unused"
2019-11-13T22:11:23.0567771Z unexpected errors (from JSON output): [
2019-11-13T22:11:23.0567851Z         line_num: 9,
2019-11-13T22:11:23.0567906Z         kind: Some(
2019-11-13T22:11:23.0567951Z             Error,
2019-11-13T22:11:23.0567988Z         ),
2019-11-13T22:11:23.0567988Z         ),
2019-11-13T22:11:23.0568049Z         msg: "9:16: 9:17: constant contains unimplemented expression type [E0019]",
2019-11-13T22:11:23.0568126Z ]
2019-11-13T22:11:23.0568149Z 
2019-11-13T22:11:23.0568442Z thread '[compile-fail] compile-fail/issue-52443.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1520:13
2019-11-13T22:11:23.0568478Z 
---
2019-11-13T22:11:23.0569451Z 
2019-11-13T22:11:23.0569677Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-13T22:11:23.0569804Z 
2019-11-13T22:11:23.0569826Z 
2019-11-13T22:11:23.0571874Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-13T22:11:23.0572171Z 
2019-11-13T22:11:23.0572201Z 
2019-11-13T22:11:23.0628945Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-13T22:11:23.0629228Z Build completed unsuccessfully in 1:03:34
2019-11-13T22:11:23.0629228Z Build completed unsuccessfully in 1:03:34
2019-11-13T22:11:23.0638831Z == clock drift check ==
2019-11-13T22:11:23.0651143Z   local time: Wed Nov 13 22:11:23 UTC 2019
2019-11-13T22:11:23.3450962Z   network time: Wed, 13 Nov 2019 22:11:23 GMT
2019-11-13T22:11:23.3451039Z == end clock drift check ==
2019-11-13T22:11:24.1436197Z 
2019-11-13T22:11:24.1525955Z ##[error]Bash exited with code '1'.
2019-11-13T22:11:24.1579407Z ##[section]Starting: Checkout
2019-11-13T22:11:24.1580855Z ==============================================================================
2019-11-13T22:11:24.1580898Z Task         : Get sources
2019-11-13T22:11:24.1581459Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
