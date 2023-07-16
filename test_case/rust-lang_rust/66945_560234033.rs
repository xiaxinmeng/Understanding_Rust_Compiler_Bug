plain
2019-12-02T04:07:31.2357055Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-02T04:07:31.2374976Z ##[command]git config gc.auto 0
2019-12-02T04:07:31.2377702Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-02T04:07:31.2379517Z ##[command]git config --get-all http.proxy
2019-12-02T04:07:31.2381966Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66945/merge:refs/remotes/pull/66945/merge
---
2019-12-02T05:02:28.9673987Z .................................................................................................... 1600/9316
2019-12-02T05:02:33.2847130Z .................................................................................................... 1700/9316
2019-12-02T05:02:44.8115485Z ........................................i........................................................... 1800/9316
2019-12-02T05:02:52.1902617Z .................................................................................................... 1900/9316
2019-12-02T05:03:05.0084243Z .........................iiiii...................................................................... 2000/9316
2019-12-02T05:03:14.5467146Z .................................................................................................... 2200/9316
2019-12-02T05:03:16.8618580Z .................................................................................................... 2300/9316
2019-12-02T05:03:21.0187783Z .................................................................................................... 2400/9316
2019-12-02T05:03:41.3195939Z .................................................................................................... 2500/9316
---
2019-12-02T05:06:09.1630152Z ...........................i...............i........................................................ 4800/9316
2019-12-02T05:06:18.9742030Z .................................................................................................... 4900/9316
2019-12-02T05:06:24.8848015Z .................................................................................................... 5000/9316
2019-12-02T05:06:32.5215746Z .................................................................................................... 5100/9316
2019-12-02T05:06:39.5552917Z .................................ii.ii...........i.................................................. 5200/9316
2019-12-02T05:06:48.3387467Z .................................................................................................... 5400/9316
2019-12-02T05:06:57.5602658Z .................................................................................................... 5500/9316
2019-12-02T05:07:04.2074650Z ...............i.................................................................................... 5600/9316
2019-12-02T05:07:09.9522906Z .................................................................................................... 5700/9316
2019-12-02T05:07:09.9522906Z .................................................................................................... 5700/9316
2019-12-02T05:07:20.9381181Z .................................................................................................... 5800/9316
2019-12-02T05:07:32.8588475Z .ii...i..ii...........i............................................................................. 5900/9316
2019-12-02T05:07:49.8611093Z .................................................................................................... 6100/9316
2019-12-02T05:07:55.5975817Z .................................................................................................... 6200/9316
2019-12-02T05:07:55.5975817Z .................................................................................................... 6200/9316
2019-12-02T05:08:08.9930321Z ........................i..ii....................................................................... 6300/9316
2019-12-02T05:08:28.9354975Z ...............................................................................................i.... 6500/9316
2019-12-02T05:08:31.1129560Z .................................................................................................... 6600/9316
2019-12-02T05:08:33.4168556Z ......................................................................................i............. 6700/9316
2019-12-02T05:08:36.0801742Z .................................................................................................... 6800/9316
---
2019-12-02T05:13:12.5045075Z ---- [ui] ui/using-target-feature-unstable.rs stdout ----
2019-12-02T05:13:12.5045304Z 
2019-12-02T05:13:12.5046050Z error: test compilation failed although it shouldn't!
2019-12-02T05:13:12.5046259Z status: exit code: 101
2019-12-02T05:13:12.5047245Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/using-target-feature-unstable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/using-target-feature-unstable/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/using-target-feature-unstable/auxiliary"
2019-12-02T05:13:12.5047884Z ------------------------------------------
2019-12-02T05:13:12.5048050Z 
2019-12-02T05:13:12.5048416Z ------------------------------------------
2019-12-02T05:13:12.5048613Z stderr:
2019-12-02T05:13:12.5048613Z stderr:
2019-12-02T05:13:12.5049101Z ------------------------------------------
2019-12-02T05:13:12.5049288Z error: internal compiler error: src/librustc_typeck/collect.rs:2527: unknown target feature gate mmx_target_feature
2019-12-02T05:13:12.5049853Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:892:9
2019-12-02T05:13:12.5050068Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-02T05:13:12.5050197Z 
2019-12-02T05:13:12.5050350Z note: the compiler unexpectedly panicked. this is a bug.
2019-12-02T05:13:12.5050350Z note: the compiler unexpectedly panicked. this is a bug.
2019-12-02T05:13:12.5050471Z 
2019-12-02T05:13:12.5051155Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-12-02T05:13:12.5051753Z note: rustc 1.41.0-dev running on x86_64-unknown-linux-gnu
2019-12-02T05:13:12.5051915Z 
2019-12-02T05:13:12.5051915Z 
2019-12-02T05:13:12.5052341Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2019-12-02T05:13:12.5052668Z error: aborting due to previous error
2019-12-02T05:13:12.5052791Z 
2019-12-02T05:13:12.5052909Z 
2019-12-02T05:13:12.5053277Z ------------------------------------------
---
2019-12-02T05:13:12.5098931Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-02T05:13:12.5099254Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-02T05:13:12.5116210Z 
2019-12-02T05:13:12.5116475Z 
2019-12-02T05:13:12.5118624Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-02T05:13:12.5119365Z 
2019-12-02T05:13:12.5119492Z 
2019-12-02T05:13:12.5123519Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-02T05:13:12.5123756Z Build completed unsuccessfully in 0:59:47
2019-12-02T05:13:12.5123756Z Build completed unsuccessfully in 0:59:47
2019-12-02T05:13:12.5174843Z == clock drift check ==
2019-12-02T05:13:12.5210794Z   local time: Mon Dec  2 05:13:12 UTC 2019
2019-12-02T05:13:13.0497071Z   network time: Mon, 02 Dec 2019 05:13:13 GMT
2019-12-02T05:13:13.0497741Z == end clock drift check ==
2019-12-02T05:13:13.9026277Z 
2019-12-02T05:13:13.9111885Z ##[error]Bash exited with code '1'.
2019-12-02T05:13:13.9156109Z ##[section]Starting: Checkout
2019-12-02T05:13:13.9158063Z ==============================================================================
2019-12-02T05:13:13.9158118Z Task         : Get sources
2019-12-02T05:13:13.9158179Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
