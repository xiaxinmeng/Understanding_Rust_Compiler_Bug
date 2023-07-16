plain
2019-11-10T21:17:36.8680153Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-10T21:17:36.8908450Z ##[command]git config gc.auto 0
2019-11-10T21:17:36.8997654Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-10T21:17:36.9060647Z ##[command]git config --get-all http.proxy
2019-11-10T21:17:36.9199934Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65881/merge:refs/remotes/pull/65881/merge
---
2019-11-10T22:17:38.7930419Z .................................................................................................... 1400/9228
2019-11-10T22:17:45.5632307Z .................................................................................................... 1500/9228
2019-11-10T22:17:51.9298204Z .................................................................................................... 1600/9228
2019-11-10T22:18:01.7505696Z .................................................................................................... 1700/9228
2019-11-10T22:18:10.4725645Z .i.................................................................................................. 1800/9228
2019-11-10T22:18:17.5248409Z .....................................................................................iiiii.......... 1900/9228
2019-11-10T22:18:39.9258437Z .................................................................................................... 2100/9228
2019-11-10T22:18:42.4606267Z .................................................................................................... 2200/9228
2019-11-10T22:18:45.3064349Z .................................................................................................... 2300/9228
2019-11-10T22:18:56.3020391Z .................................................................................................... 2400/9228
---
2019-11-10T22:21:53.0075024Z .................................................................................i...............i.. 4700/9228
2019-11-10T22:22:00.2407497Z .................................................................................................... 4800/9228
2019-11-10T22:22:09.5644645Z .................................................................................................... 4900/9228
2019-11-10T22:22:14.8181334Z .................................................................................................... 5000/9228
2019-11-10T22:22:26.2574998Z ...................................................................................ii.ii...........i 5100/9228
2019-11-10T22:22:35.4926773Z ..................i................................................................................. 5300/9228
2019-11-10T22:22:45.5213930Z .................................................................................................... 5400/9228
2019-11-10T22:22:52.6184156Z .................................................................i.................................. 5500/9228
2019-11-10T22:23:00.1444265Z .................................................................................................... 5600/9228
2019-11-10T22:23:00.1444265Z .................................................................................................... 5600/9228
2019-11-10T22:23:08.5063175Z .................................................................................................... 5700/9228
2019-11-10T22:23:17.3803217Z ..................................................ii...i..ii...........i............................ 5800/9228
2019-11-10T22:23:40.2572240Z .................................................................................................... 6000/9228
2019-11-10T22:23:48.7990986Z .................................................................................................... 6100/9228
2019-11-10T22:23:48.7990986Z .................................................................................................... 6100/9228
2019-11-10T22:23:54.1070938Z .....................................................................i..ii.......................... 6200/9228
2019-11-10T22:24:23.5306625Z .................................................................................................... 6400/9228
2019-11-10T22:24:25.7577628Z .....................................i.............................................................. 6500/9228
2019-11-10T22:24:28.1229963Z .................................................................................................... 6600/9228
2019-11-10T22:24:30.5601292Z .....................i.............................................................................. 6700/9228
---
2019-11-10T22:29:23.0076366Z ---- [ui] ui/codegen-object-shim.rs stdout ----
2019-11-10T22:29:23.0076419Z 
2019-11-10T22:29:23.0076726Z error: test compilation failed although it shouldn't!
2019-11-10T22:29:23.0076780Z status: exit code: 101
2019-11-10T22:29:23.0080149Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/codegen-object-shim.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codegen-object-shim/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/codegen-object-shim/auxiliary"
2019-11-10T22:29:23.0080621Z ------------------------------------------
2019-11-10T22:29:23.0080676Z 
2019-11-10T22:29:23.0080897Z ------------------------------------------
2019-11-10T22:29:23.0080943Z stderr:
2019-11-10T22:29:23.0080943Z stderr:
2019-11-10T22:29:23.0081570Z ------------------------------------------
2019-11-10T22:29:23.0082161Z error: internal compiler error: src/librustc_mir/monomorphize/collector.rs:749: Instance { def: Virtual(DefId(5:3701 ~ alloc[b012]::string[0]::ToString[0]::to_string[0]), 0), substs: [dyn std::string::ToString] } being reified
2019-11-10T22:29:23.0082586Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:939:9
2019-11-10T22:29:23.0082645Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-10T22:29:23.0082678Z 
2019-11-10T22:29:23.0082724Z note: the compiler unexpectedly panicked. this is a bug.
2019-11-10T22:29:23.0082724Z note: the compiler unexpectedly panicked. this is a bug.
2019-11-10T22:29:23.0082771Z 
2019-11-10T22:29:23.0083204Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-11-10T22:29:23.0083503Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-11-10T22:29:23.0083537Z 
2019-11-10T22:29:23.0083537Z 
2019-11-10T22:29:23.0084410Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2019-11-10T22:29:23.0084519Z error: aborting due to previous error
2019-11-10T22:29:23.0084548Z 
2019-11-10T22:29:23.0084573Z 
2019-11-10T22:29:23.0085357Z ------------------------------------------
---
2019-11-10T22:29:23.0116066Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-10T22:29:23.0116344Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-10T22:29:23.0134352Z 
2019-11-10T22:29:23.0135554Z 
2019-11-10T22:29:23.0137933Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-10T22:29:23.0138228Z 
2019-11-10T22:29:23.0138258Z 
2019-11-10T22:29:23.0143558Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-10T22:29:23.0143655Z Build completed unsuccessfully in 1:05:05
2019-11-10T22:29:23.0143655Z Build completed unsuccessfully in 1:05:05
2019-11-10T22:29:23.0201656Z == clock drift check ==
2019-11-10T22:29:23.0298966Z   local time: Sun Nov 10 22:29:23 UTC 2019
2019-11-10T22:29:23.3166906Z   network time: Sun, 10 Nov 2019 22:29:23 GMT
2019-11-10T22:29:23.3171863Z == end clock drift check ==
2019-11-10T22:29:24.0279476Z 
2019-11-10T22:29:24.0351778Z ##[error]Bash exited with code '1'.
2019-11-10T22:29:24.0412801Z ##[section]Starting: Checkout
2019-11-10T22:29:24.0414485Z ==============================================================================
2019-11-10T22:29:24.0414534Z Task         : Get sources
2019-11-10T22:29:24.0414576Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
