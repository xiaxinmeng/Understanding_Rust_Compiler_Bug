plain
2019-10-26T23:36:38.9221471Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-26T23:36:38.9403881Z ##[command]git config gc.auto 0
2019-10-26T23:36:38.9482633Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-26T23:36:38.9542127Z ##[command]git config --get-all http.proxy
2019-10-26T23:36:38.9702576Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65664/merge:refs/remotes/pull/65664/merge
---
2019-10-27T00:37:15.3082542Z .................................................................................................... 1600/9253
2019-10-27T00:37:21.4808428Z .................................................................................................... 1700/9253
2019-10-27T00:37:33.6581969Z .........................................................i...............i.......................... 1800/9253
2019-10-27T00:37:41.3682287Z .................................................................................................... 1900/9253
2019-10-27T00:37:55.9896483Z ...............................................iiiii................................................ 2000/9253
2019-10-27T00:38:06.8541011Z .................................................................................................... 2200/9253
2019-10-27T00:38:09.4720579Z .................................................................................................... 2300/9253
2019-10-27T00:38:13.3371660Z .................................................................................................... 2400/9253
2019-10-27T00:38:36.8338983Z .................................................................................................... 2500/9253
---
2019-10-27T00:41:32.3513174Z ..................................................i...............i................................. 4800/9253
2019-10-27T00:41:41.5468328Z .................................................................................................... 4900/9253
2019-10-27T00:41:50.2491364Z .................................................................................................... 5000/9253
2019-10-27T00:41:56.5602702Z .................................................................................................... 5100/9253
2019-10-27T00:42:07.1093882Z ...................................................ii.ii...........i................................ 5200/9253
2019-10-27T00:42:16.8798088Z .................................................................................................... 5400/9253
2019-10-27T00:42:26.3632826Z .................................................................................................... 5500/9253
2019-10-27T00:42:34.4290224Z .....................i.............................................................................. 5600/9253
2019-10-27T00:42:40.0680161Z .................................................................................................... 5700/9253
2019-10-27T00:42:40.0680161Z .................................................................................................... 5700/9253
2019-10-27T00:42:52.1482285Z .................................................................................................... 5800/9253
2019-10-27T00:43:03.5917748Z ..................ii...i..ii...........i............................................................ 5900/9253
2019-10-27T00:43:25.8186703Z .................................................................................................... 6100/9253
2019-10-27T00:43:31.5959529Z .................................................................................................... 6200/9253
2019-10-27T00:43:31.5959529Z .................................................................................................... 6200/9253
2019-10-27T00:43:44.4604533Z .........................................i..ii...................................................... 6300/9253
2019-10-27T00:44:06.9912664Z .................................................................................................... 6500/9253
2019-10-27T00:44:09.2941131Z .......i............................................................................................ 6600/9253
2019-10-27T00:44:11.5829791Z ..................................................................................i................. 6700/9253
2019-10-27T00:44:14.3147968Z .................................................................................................... 6800/9253
---
2019-10-27T00:48:51.1728199Z  finished in 5.664
2019-10-27T00:48:51.1919037Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-27T00:48:51.3674946Z 
2019-10-27T00:48:51.3675294Z running 153 tests
2019-10-27T00:48:54.5927905Z i....iii......iii..iiii...i.............................i..i..................i....i...........ii.i. 100/153
2019-10-27T00:48:56.5654381Z i..iiii..............i.........iii.i.........ii......
2019-10-27T00:48:56.5658453Z 
2019-10-27T00:48:56.5658521Z  finished in 5.373
2019-10-27T00:48:56.5842549Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-27T00:48:56.7507102Z 
---
2019-10-27T00:48:58.8001313Z  finished in 2.215
2019-10-27T00:48:58.8169492Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-27T00:48:58.9856239Z 
2019-10-27T00:48:58.9856477Z running 9 tests
2019-10-27T00:48:58.9858927Z iiiiiiiii
2019-10-27T00:48:58.9859797Z 
2019-10-27T00:48:58.9859906Z  finished in 0.168
2019-10-27T00:48:59.0059916Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-27T00:48:59.1762586Z 
2019-10-27T00:48:59.1762586Z 
2019-10-27T00:48:59.1762808Z running 109 tests
2019-10-27T00:49:16.4612281Z .................................................F.................................................. 100/109
2019-10-27T00:49:17.9365687Z .........
2019-10-27T00:49:17.9365805Z failures:
2019-10-27T00:49:17.9365874Z 
2019-10-27T00:49:17.9372974Z ---- [incremental] incremental/hashes/unary_and_binary_exprs.rs stdout ----
2019-10-27T00:49:17.9373041Z 
2019-10-27T00:49:17.9373344Z error in revision `cfail2`: test compilation failed although it shouldn't!
2019-10-27T00:49:17.9373606Z status: exit code: 101
2019-10-27T00:49:17.9374667Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/unary_and_binary_exprs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/unary_and_binary_exprs/unary_and_binary_exprs.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/unary_and_binary_exprs" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/unary_and_binary_exprs/auxiliary"
2019-10-27T00:49:17.9375006Z ------------------------------------------
2019-10-27T00:49:17.9375326Z 
2019-10-27T00:49:17.9375941Z ------------------------------------------
2019-10-27T00:49:17.9375985Z stderr:
2019-10-27T00:49:17.9375985Z stderr:
2019-10-27T00:49:17.9376184Z ------------------------------------------
2019-10-27T00:49:17.9376474Z thread 'rustc' panicked at 'found unstable fingerprints for const_caller_location(7de0a38047d74950-f4f2ced447ab0242)', src/librustc/ty/query/plumbing.rs:490:9
2019-10-27T00:49:17.9376580Z 
2019-10-27T00:49:17.9376617Z error: internal compiler error: unexpected panic
2019-10-27T00:49:17.9376643Z 
2019-10-27T00:49:17.9376682Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-27T00:49:17.9376682Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-27T00:49:17.9376735Z 
2019-10-27T00:49:17.9377306Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-10-27T00:49:17.9377558Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-27T00:49:17.9377586Z 
2019-10-27T00:49:17.9377586Z 
2019-10-27T00:49:17.9378048Z note: compiler flags: -Z threads=1 -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -Z incremental-ignore-spans -C incremental -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-27T00:49:17.9378135Z 
2019-10-27T00:49:17.9378313Z ------------------------------------------
2019-10-27T00:49:17.9378339Z 
2019-10-27T00:49:17.9378362Z 
---
2019-10-27T00:49:17.9386718Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-27T00:49:17.9386801Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-27T00:49:17.9394495Z 
2019-10-27T00:49:17.9395611Z 
2019-10-27T00:49:17.9399645Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-27T00:49:17.9399915Z 
2019-10-27T00:49:17.9399942Z 
2019-10-27T00:49:17.9407719Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-27T00:49:17.9408479Z Build completed unsuccessfully in 1:06:03
2019-10-27T00:49:17.9408479Z Build completed unsuccessfully in 1:06:03
2019-10-27T00:49:17.9464442Z == clock drift check ==
2019-10-27T00:49:17.9477310Z   local time: Sun Oct 27 00:49:17 UTC 2019
2019-10-27T00:49:18.2265559Z   network time: Sun, 27 Oct 2019 00:49:18 GMT
2019-10-27T00:49:18.2265673Z == end clock drift check ==
2019-10-27T00:49:23.0643520Z 
2019-10-27T00:49:23.0767313Z ##[error]Bash exited with code '1'.
2019-10-27T00:49:23.0805518Z ##[section]Starting: Checkout
2019-10-27T00:49:23.0807061Z ==============================================================================
2019-10-27T00:49:23.0807109Z Task         : Get sources
2019-10-27T00:49:23.0807167Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
