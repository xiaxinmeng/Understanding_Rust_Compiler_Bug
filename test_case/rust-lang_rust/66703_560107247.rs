plain
2019-12-01T12:04:36.1507984Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-01T12:04:36.1685125Z ##[command]git config gc.auto 0
2019-12-01T12:04:36.1766154Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-01T12:04:36.1859694Z ##[command]git config --get-all http.proxy
2019-12-01T12:04:36.2002555Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66703/merge:refs/remotes/pull/66703/merge
---
2019-12-01T13:05:32.3367801Z .................................................................................................... 1600/9316
2019-12-01T13:05:37.0404515Z .................................................................................................... 1700/9316
2019-12-01T13:05:49.9964909Z ........................................i........................................................... 1800/9316
2019-12-01T13:05:58.0103961Z .................................................................................................... 1900/9316
2019-12-01T13:06:13.2282724Z .........................iiiii...................................................................... 2000/9316
2019-12-01T13:06:23.6042439Z .................................................................................................... 2200/9316
2019-12-01T13:06:26.2622199Z .................................................................................................... 2300/9316
2019-12-01T13:06:30.9842046Z .................................................................................................... 2400/9316
2019-12-01T13:06:53.3495967Z .................................................................................................... 2500/9316
---
2019-12-01T13:09:41.0889785Z ...........................i...............i........................................................ 4800/9316
2019-12-01T13:09:52.1605906Z .................................................................................................... 4900/9316
2019-12-01T13:09:58.4613339Z .................................................................................................... 5000/9316
2019-12-01T13:10:06.9525196Z .................................................................................................... 5100/9316
2019-12-01T13:10:15.0892328Z .................................ii.ii...........i.................................................. 5200/9316
2019-12-01T13:10:25.1773872Z .................................................................................................... 5400/9316
2019-12-01T13:10:35.6903957Z .................................................................................................... 5500/9316
2019-12-01T13:10:43.6569083Z ...............i.................................................................................... 5600/9316
2019-12-01T13:10:52.1969927Z .................................................................................................... 5700/9316
2019-12-01T13:10:52.1969927Z .................................................................................................... 5700/9316
2019-12-01T13:11:04.4450159Z .................................................................................................... 5800/9316
2019-12-01T13:11:17.6014383Z .ii...i..ii...........i............................................................................. 5900/9316
2019-12-01T13:11:36.2170090Z .................................................................................................... 6100/9316
2019-12-01T13:11:41.3788244Z .................................................................................................... 6200/9316
2019-12-01T13:11:41.3788244Z .................................................................................................... 6200/9316
2019-12-01T13:11:55.7555482Z ........................i..ii....................................................................... 6300/9316
2019-12-01T13:12:16.1924632Z ...............................................................................................i.... 6500/9316
2019-12-01T13:12:18.5340561Z .................................................................................................... 6600/9316
2019-12-01T13:12:20.8955941Z ......................................................................................i............. 6700/9316
2019-12-01T13:12:23.7320339Z .................................................................................................... 6800/9316
---
2019-12-01T13:17:51.8690804Z  finished in 6.181
2019-12-01T13:17:51.8895754Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-01T13:17:52.0750192Z 
2019-12-01T13:17:52.0750957Z running 164 tests
2019-12-01T13:17:54.9803963Z iiii....iii......iii..iiii...i.............................i..i..................i....i...........ii 100/164
2019-12-01T13:17:57.0753052Z .i.i..iiii..iiiiiii............i.........iii.i..........ii......
2019-12-01T13:17:57.0756587Z 
2019-12-01T13:17:57.0757889Z  finished in 5.186
2019-12-01T13:17:57.0962217Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-01T13:17:57.2680692Z 
---
2019-12-01T13:17:59.2872744Z ---- [codegen-units] codegen-units/partitioning/extern-drop-glue.rs stdout ----
2019-12-01T13:17:59.2872783Z 
2019-12-01T13:17:59.2872833Z The following items were assigned to wrong codegen units:
2019-12-01T13:17:59.2872883Z 
2019-12-01T13:17:59.2873383Z fn core::ptr[0]::real_drop_in_place[0]<cgu_extern_drop_glue::Struct[0]>
2019-12-01T13:17:59.2873815Z   expected: extern_drop_glue-mod1[Internal] extern_drop_glue[Internal] 
2019-12-01T13:17:59.2874304Z   actual:   extern_drop_glue-fallback.cgu[Internal] 
2019-12-01T13:17:59.2874499Z 
2019-12-01T13:17:59.2874579Z fn core::ptr[0]::real_drop_in_place[0]<extern_drop_glue::LocalStruct[0]>
2019-12-01T13:17:59.2874637Z   expected: extern_drop_glue[Internal] 
2019-12-01T13:17:59.2874961Z   actual:   extern_drop_glue-fallback.cgu[External] 
2019-12-01T13:17:59.2875148Z 
2019-12-01T13:17:59.2876451Z fn core::ptr[0]::real_drop_in_place[0]<extern_drop_glue::mod1[0]::LocalStruct[0]>
2019-12-01T13:17:59.2877088Z   expected: extern_drop_glue-mod1[Internal] 
2019-12-01T13:17:59.2877587Z   actual:   extern_drop_glue-fallback.cgu[External] 
2019-12-01T13:17:59.2878131Z thread '[codegen-units] codegen-units/partitioning/extern-drop-glue.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2623:13
2019-12-01T13:17:59.2879405Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-01T13:17:59.2879453Z 
2019-12-01T13:17:59.2879844Z ---- [codegen-units] codegen-units/partitioning/local-drop-glue.rs stdout ----
2019-12-01T13:17:59.2879844Z ---- [codegen-units] codegen-units/partitioning/local-drop-glue.rs stdout ----
2019-12-01T13:17:59.2879880Z 
2019-12-01T13:17:59.2879926Z The following items were assigned to wrong codegen units:
2019-12-01T13:17:59.2879970Z 
2019-12-01T13:17:59.2880033Z fn core::ptr[0]::real_drop_in_place[0]<(u32, local_drop_glue::Struct[0])>
2019-12-01T13:17:59.2880264Z   expected: local_drop_glue-mod1[Internal] 
2019-12-01T13:17:59.2880593Z   actual:   local_drop_glue-fallback.cgu[Internal] 
2019-12-01T13:17:59.2880625Z 
2019-12-01T13:17:59.2880688Z fn core::ptr[0]::real_drop_in_place[0]<local_drop_glue::Outer[0]>
2019-12-01T13:17:59.2880735Z   expected: local_drop_glue[Internal] 
2019-12-01T13:17:59.2880989Z   actual:   local_drop_glue-fallback.cgu[External] 
2019-12-01T13:17:59.2881039Z 
2019-12-01T13:17:59.2881086Z fn core::ptr[0]::real_drop_in_place[0]<local_drop_glue::Struct[0]>
2019-12-01T13:17:59.2881358Z   expected: local_drop_glue-mod1[Internal] local_drop_glue[Internal] 
2019-12-01T13:17:59.2881614Z   actual:   local_drop_glue-fallback.cgu[Internal] 
2019-12-01T13:17:59.2881647Z 
2019-12-01T13:17:59.2881693Z fn core::ptr[0]::real_drop_in_place[0]<local_drop_glue::mod1[0]::Struct2[0]>
2019-12-01T13:17:59.2881940Z   expected: local_drop_glue-mod1[Internal] 
2019-12-01T13:17:59.2882204Z   actual:   local_drop_glue-fallback.cgu[External] 
2019-12-01T13:17:59.2882237Z 
2019-12-01T13:17:59.2882279Z fn local_drop_glue::{{impl}}[0]::drop[0]
2019-12-01T13:17:59.2882343Z   expected: local_drop_glue[External] 
2019-12-01T13:17:59.2882582Z   actual:   local_drop_glue-fallback.cgu[External] 
2019-12-01T13:17:59.2882944Z thread '[codegen-units] codegen-units/partitioning/local-drop-glue.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2623:13
2019-12-01T13:17:59.2883000Z 
2019-12-01T13:17:59.2883026Z 
2019-12-01T13:17:59.2883236Z failures:
---
2019-12-01T13:17:59.2884230Z 
2019-12-01T13:17:59.2887538Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-01T13:17:59.2893830Z 
2019-12-01T13:17:59.2893903Z 
2019-12-01T13:17:59.2895707Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen-units" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen-units" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-01T13:17:59.2896192Z 
2019-12-01T13:17:59.2896270Z 
2019-12-01T13:17:59.2906744Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-01T13:17:59.2906815Z Build completed unsuccessfully in 1:07:32
2019-12-01T13:17:59.2906815Z Build completed unsuccessfully in 1:07:32
2019-12-01T13:17:59.2961116Z == clock drift check ==
2019-12-01T13:17:59.2987126Z   local time: Sun Dec  1 13:17:59 UTC 2019
2019-12-01T13:18:00.2110710Z   network time: Sun, 01 Dec 2019 13:17:59 GMT
2019-12-01T13:18:00.2148375Z == end clock drift check ==
2019-12-01T13:18:03.4048129Z 
2019-12-01T13:18:03.4136472Z ##[error]Bash exited with code '1'.
2019-12-01T13:18:03.4199166Z ##[section]Starting: Checkout
2019-12-01T13:18:03.4201043Z ==============================================================================
2019-12-01T13:18:03.4201105Z Task         : Get sources
2019-12-01T13:18:03.4201174Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
