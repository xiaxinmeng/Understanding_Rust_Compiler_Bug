plain
2019-11-24T14:55:39.3308491Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-24T14:55:39.3321752Z ##[command]git config gc.auto 0
2019-11-24T14:55:39.3325067Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-24T14:55:39.3328446Z ##[command]git config --get-all http.proxy
2019-11-24T14:55:39.3333755Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66703/merge:refs/remotes/pull/66703/merge
---
2019-11-24T15:51:27.2590468Z .................................................................................................... 1600/9282
2019-11-24T15:51:31.9946096Z .................................................................................................... 1700/9282
2019-11-24T15:51:44.8536150Z .............................i...................................................................... 1800/9282
2019-11-24T15:51:51.6425770Z .................................................................................................... 1900/9282
2019-11-24T15:52:06.6937511Z ..............iiiii................................................................................. 2000/9282
2019-11-24T15:52:16.4079683Z .................................................................................................... 2200/9282
2019-11-24T15:52:18.9286340Z .................................................................................................... 2300/9282
2019-11-24T15:52:23.9958948Z .................................................................................................... 2400/9282
2019-11-24T15:52:44.8037726Z .................................................................................................... 2500/9282
---
2019-11-24T15:55:19.6790682Z ..............i...............i..................................................................... 4800/9282
2019-11-24T15:55:29.4375330Z .................................................................................................... 4900/9282
2019-11-24T15:55:34.8258343Z .................................................................................................... 5000/9282
2019-11-24T15:55:43.8892357Z .................................................................................................... 5100/9282
2019-11-24T15:55:49.7006340Z ...................ii.ii...........i................................................................ 5200/9282
2019-11-24T15:55:58.3623969Z .................................................................................................... 5400/9282
2019-11-24T15:56:08.7734634Z .................................................................................................... 5500/9282
2019-11-24T15:56:15.9358456Z .i.................................................................................................. 5600/9282
2019-11-24T15:56:21.2858124Z .................................................................................................... 5700/9282
2019-11-24T15:56:21.2858124Z .................................................................................................... 5700/9282
2019-11-24T15:56:31.0319169Z .......................................................................................i.i..i..ii... 5800/9282
2019-11-24T15:56:52.4305862Z .................................................................................................... 6000/9282
2019-11-24T15:56:59.9651694Z .................................................................................................... 6100/9282
2019-11-24T15:57:04.3988849Z .................................................................................................... 6200/9282
2019-11-24T15:57:04.3988849Z .................................................................................................... 6200/9282
2019-11-24T15:57:17.2240491Z ..........i..ii..................................................................................... 6300/9282
2019-11-24T15:57:35.0183344Z ..............................................................................i..................... 6500/9282
2019-11-24T15:57:37.1792401Z .................................................................................................... 6600/9282
2019-11-24T15:57:39.2727407Z .....................................................................i.............................. 6700/9282
2019-11-24T15:57:42.0930106Z .................................................................................................... 6800/9282
---
2019-11-24T16:02:32.0153873Z  finished in 5.552
2019-11-24T16:02:32.0320564Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-24T16:02:32.2026242Z 
2019-11-24T16:02:32.2026930Z running 157 tests
2019-11-24T16:02:34.8418391Z iiii....iii......iii..iiii...i.............................i..i..................i....i...........ii 100/157
2019-11-24T16:02:36.6682864Z .i.i..iiii..............i.........iii.i..........ii......
2019-11-24T16:02:36.6683313Z 
2019-11-24T16:02:36.6690275Z  finished in 4.636
2019-11-24T16:02:36.6875632Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-24T16:02:36.8643479Z 
---
2019-11-24T16:02:38.6923388Z ---- [codegen-units] codegen-units/partitioning/extern-drop-glue.rs stdout ----
2019-11-24T16:02:38.6923443Z 
2019-11-24T16:02:38.6923629Z The following items were assigned to wrong codegen units:
2019-11-24T16:02:38.6923748Z 
2019-11-24T16:02:38.6923826Z fn core::ptr[0]::real_drop_in_place[0]<cgu_extern_drop_glue::Struct[0]>
2019-11-24T16:02:38.6924454Z   expected: extern_drop_glue-mod1[Internal] extern_drop_glue[Internal] 
2019-11-24T16:02:38.6924911Z   actual:   extern_drop_glue-fallback.cgu[Internal] 
2019-11-24T16:02:38.6925012Z 
2019-11-24T16:02:38.6925065Z fn core::ptr[0]::real_drop_in_place[0]<extern_drop_glue::LocalStruct[0]>
2019-11-24T16:02:38.6925152Z   expected: extern_drop_glue[Internal] 
2019-11-24T16:02:38.6925447Z   actual:   extern_drop_glue-fallback.cgu[External] 
2019-11-24T16:02:38.6925496Z 
2019-11-24T16:02:38.6925547Z fn core::ptr[0]::real_drop_in_place[0]<extern_drop_glue::mod1[0]::LocalStruct[0]>
2019-11-24T16:02:38.6926024Z   expected: extern_drop_glue-mod1[Internal] 
2019-11-24T16:02:38.6926466Z   actual:   extern_drop_glue-fallback.cgu[External] 
2019-11-24T16:02:38.6927194Z thread '[codegen-units] codegen-units/partitioning/extern-drop-glue.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2623:13
2019-11-24T16:02:38.6927421Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-24T16:02:38.6927530Z 
2019-11-24T16:02:38.6927874Z ---- [codegen-units] codegen-units/partitioning/local-drop-glue.rs stdout ----
2019-11-24T16:02:38.6927874Z ---- [codegen-units] codegen-units/partitioning/local-drop-glue.rs stdout ----
2019-11-24T16:02:38.6928236Z 
2019-11-24T16:02:38.6928429Z The following items were assigned to wrong codegen units:
2019-11-24T16:02:38.6928471Z 
2019-11-24T16:02:38.6928539Z fn core::ptr[0]::real_drop_in_place[0]<(u32, local_drop_glue::Struct[0])>
2019-11-24T16:02:38.6928932Z   expected: local_drop_glue-mod1[Internal] 
2019-11-24T16:02:38.6929176Z   actual:   local_drop_glue-fallback.cgu[Internal] 
2019-11-24T16:02:38.6929224Z 
2019-11-24T16:02:38.6929412Z fn core::ptr[0]::real_drop_in_place[0]<local_drop_glue::Outer[0]>
2019-11-24T16:02:38.6929550Z   expected: local_drop_glue[Internal] 
2019-11-24T16:02:38.6929839Z   actual:   local_drop_glue-fallback.cgu[External] 
2019-11-24T16:02:38.6929870Z 
2019-11-24T16:02:38.6929920Z fn core::ptr[0]::real_drop_in_place[0]<local_drop_glue::Struct[0]>
2019-11-24T16:02:38.6930359Z   expected: local_drop_glue-mod1[Internal] local_drop_glue[Internal] 
2019-11-24T16:02:38.6930740Z   actual:   local_drop_glue-fallback.cgu[Internal] 
2019-11-24T16:02:38.6930881Z 
2019-11-24T16:02:38.6931005Z fn core::ptr[0]::real_drop_in_place[0]<local_drop_glue::mod1[0]::Struct2[0]>
2019-11-24T16:02:38.6931250Z   expected: local_drop_glue-mod1[Internal] 
2019-11-24T16:02:38.6931617Z   actual:   local_drop_glue-fallback.cgu[External] 
2019-11-24T16:02:38.6932256Z thread '[codegen-units] codegen-units/partitioning/local-drop-glue.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2623:13
2019-11-24T16:02:38.6932411Z 
2019-11-24T16:02:38.6937149Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-24T16:02:38.6937449Z 
---
2019-11-24T16:02:38.6942807Z test result: FAILED. 34 passed; 2 failed; 3 ignored; 0 measured; 0 filtered out
2019-11-24T16:02:38.6955193Z 
2019-11-24T16:02:38.6956913Z 
2019-11-24T16:02:38.6957120Z 
2019-11-24T16:02:38.6959152Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen-units" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen-units" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-24T16:02:38.6959585Z 
2019-11-24T16:02:38.6959705Z 
2019-11-24T16:02:38.6959821Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-24T16:02:38.6959937Z Build completed unsuccessfully in 1:01:13
2019-11-24T16:02:38.6959937Z Build completed unsuccessfully in 1:01:13
2019-11-24T16:02:38.7010677Z == clock drift check ==
2019-11-24T16:02:38.7033607Z   local time: Sun Nov 24 16:02:38 UTC 2019
2019-11-24T16:02:39.9338294Z   network time: Sun, 24 Nov 2019 16:02:38 GMT
2019-11-24T16:02:40.9342028Z == end clock drift check ==
2019-11-24T16:02:42.7322231Z 
2019-11-24T16:02:42.7430909Z ##[error]Bash exited with code '1'.
2019-11-24T16:02:42.7467499Z ##[section]Starting: Checkout
2019-11-24T16:02:42.7470096Z ==============================================================================
2019-11-24T16:02:42.7470140Z Task         : Get sources
2019-11-24T16:02:42.7470192Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
