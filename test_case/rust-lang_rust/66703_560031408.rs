plain
2019-11-30T22:53:35.7240907Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-30T22:53:35.7533357Z ##[command]git config gc.auto 0
2019-11-30T22:53:35.7592805Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-30T22:53:35.7644953Z ##[command]git config --get-all http.proxy
2019-11-30T22:53:35.7769229Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66703/merge:refs/remotes/pull/66703/merge
---
2019-11-30T23:45:34.0423922Z .................................................................................................... 1600/9312
2019-11-30T23:45:38.0985929Z .................................................................................................... 1700/9312
2019-11-30T23:45:49.2137375Z ........................................i........................................................... 1800/9312
2019-11-30T23:45:55.9384333Z .................................................................................................... 1900/9312
2019-11-30T23:46:08.6929350Z .........................iiiii...................................................................... 2000/9312
2019-11-30T23:46:17.2925879Z .................................................................................................... 2200/9312
2019-11-30T23:46:19.4355558Z .................................................................................................... 2300/9312
2019-11-30T23:46:23.1996440Z .................................................................................................... 2400/9312
2019-11-30T23:46:41.4945638Z .................................................................................................... 2500/9312
---
2019-11-30T23:48:55.7955904Z ...........................i...............i........................................................ 4800/9312
2019-11-30T23:49:04.5495374Z .................................................................................................... 4900/9312
2019-11-30T23:49:09.6669169Z .................................................................................................... 5000/9312
2019-11-30T23:49:16.5744915Z .................................................................................................... 5100/9312
2019-11-30T23:49:23.0154365Z ................................ii.ii...........i................................................... 5200/9312
2019-11-30T23:49:30.9501400Z .................................................................................................... 5400/9312
2019-11-30T23:49:39.8215659Z .................................................................................................... 5500/9312
2019-11-30T23:49:45.4895929Z ..............i..................................................................................... 5600/9312
2019-11-30T23:49:50.9786052Z .................................................................................................... 5700/9312
2019-11-30T23:49:50.9786052Z .................................................................................................... 5700/9312
2019-11-30T23:50:01.4003638Z .................................................................................................... 5800/9312
2019-11-30T23:50:12.6078126Z ii...i..ii............i............................................................................. 5900/9312
2019-11-30T23:50:27.0520622Z .................................................................................................... 6100/9312
2019-11-30T23:50:30.3275714Z .................................................................................................... 6200/9312
2019-11-30T23:50:30.3275714Z .................................................................................................... 6200/9312
2019-11-30T23:50:42.4498954Z .......................i..ii........................................................................ 6300/9312
2019-11-30T23:50:59.7261055Z ...........................................................................................i........ 6500/9312
2019-11-30T23:51:01.6558980Z .................................................................................................... 6600/9312
2019-11-30T23:51:03.5277161Z ..................................................................................i................. 6700/9312
2019-11-30T23:51:05.7559390Z .................................................................................................... 6800/9312
---
2019-11-30T23:55:33.9363484Z  finished in 5.392
2019-11-30T23:55:33.9532400Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-30T23:55:34.1380224Z 
2019-11-30T23:55:34.1380449Z running 164 tests
2019-11-30T23:55:36.5434326Z iiii....iii......iii..iiii...i.............................i..i..................i....i...........ii 100/164
2019-11-30T23:55:38.2313098Z .i.i..iiii..iiiiiii............i.........iii.i..........ii......
2019-11-30T23:55:38.2314538Z 
2019-11-30T23:55:38.2316220Z  finished in 4.278
2019-11-30T23:55:38.2455705Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-11-30T23:55:38.3909763Z 
---
2019-11-30T23:55:40.1084488Z ---- [codegen-units] codegen-units/partitioning/extern-drop-glue.rs stdout ----
2019-11-30T23:55:40.1084584Z 
2019-11-30T23:55:40.1084757Z The following items were assigned to wrong codegen units:
2019-11-30T23:55:40.1084786Z 
2019-11-30T23:55:40.1084865Z fn core::ptr[0]::real_drop_in_place[0]<cgu_extern_drop_glue::Struct[0]>
2019-11-30T23:55:40.1085127Z   expected: extern_drop_glue-mod1[Internal] extern_drop_glue[Internal] 
2019-11-30T23:55:40.1085305Z   actual:   extern_drop_glue-fallback.cgu[Internal] 
2019-11-30T23:55:40.1085349Z 
2019-11-30T23:55:40.1085388Z fn core::ptr[0]::real_drop_in_place[0]<extern_drop_glue::LocalStruct[0]>
2019-11-30T23:55:40.1085437Z   expected: extern_drop_glue[Internal] 
2019-11-30T23:55:40.1085647Z   actual:   extern_drop_glue-fallback.cgu[External] 
2019-11-30T23:55:40.1085763Z 
2019-11-30T23:55:40.1086462Z fn core::ptr[0]::real_drop_in_place[0]<extern_drop_glue::mod1[0]::LocalStruct[0]>
2019-11-30T23:55:40.1087824Z   expected: extern_drop_glue-mod1[Internal] 
2019-11-30T23:55:40.1088038Z   actual:   extern_drop_glue-fallback.cgu[External] 
2019-11-30T23:55:40.1088599Z thread '[codegen-units] codegen-units/partitioning/extern-drop-glue.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2623:13
2019-11-30T23:55:40.1088986Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-30T23:55:40.1089053Z 
2019-11-30T23:55:40.1090639Z ---- [codegen-units] codegen-units/partitioning/local-drop-glue.rs stdout ----
2019-11-30T23:55:40.1090639Z ---- [codegen-units] codegen-units/partitioning/local-drop-glue.rs stdout ----
2019-11-30T23:55:40.1096654Z 
2019-11-30T23:55:40.1097535Z The following items were assigned to wrong codegen units:
2019-11-30T23:55:40.1098329Z 
2019-11-30T23:55:40.1098943Z fn core::ptr[0]::real_drop_in_place[0]<(u32, local_drop_glue::Struct[0])>
2019-11-30T23:55:40.1099862Z   expected: local_drop_glue-mod1[Internal] 
2019-11-30T23:55:40.1100732Z   actual:   local_drop_glue-fallback.cgu[Internal] 
2019-11-30T23:55:40.1101808Z 
2019-11-30T23:55:40.1102345Z fn core::ptr[0]::real_drop_in_place[0]<local_drop_glue::Outer[0]>
2019-11-30T23:55:40.1102855Z   expected: local_drop_glue[Internal] 
2019-11-30T23:55:40.1103698Z   actual:   local_drop_glue-fallback.cgu[External] 
2019-11-30T23:55:40.1103895Z 
2019-11-30T23:55:40.1104417Z fn core::ptr[0]::real_drop_in_place[0]<local_drop_glue::Struct[0]>
2019-11-30T23:55:40.1105180Z   expected: local_drop_glue-mod1[Internal] local_drop_glue[Internal] 
2019-11-30T23:55:40.1105655Z   actual:   local_drop_glue-fallback.cgu[Internal] 
2019-11-30T23:55:40.1105952Z 
2019-11-30T23:55:40.1106477Z fn core::ptr[0]::real_drop_in_place[0]<local_drop_glue::mod1[0]::Struct2[0]>
2019-11-30T23:55:40.1107197Z   expected: local_drop_glue-mod1[Internal] 
2019-11-30T23:55:40.1107724Z   actual:   local_drop_glue-fallback.cgu[External] 
2019-11-30T23:55:40.1108034Z 
2019-11-30T23:55:40.1108548Z fn local_drop_glue::{{impl}}[0]::drop[0]
2019-11-30T23:55:40.1109054Z   expected: local_drop_glue[External] 
2019-11-30T23:55:40.1109756Z   actual:   local_drop_glue-fallback.cgu[External] 
2019-11-30T23:55:40.1110845Z thread '[codegen-units] codegen-units/partitioning/local-drop-glue.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2623:13
2019-11-30T23:55:40.1111091Z 
2019-11-30T23:55:40.1111804Z 
2019-11-30T23:55:40.1112332Z failures:
---
2019-11-30T23:55:40.1113542Z 
2019-11-30T23:55:40.1114543Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-30T23:55:40.1114723Z 
2019-11-30T23:55:40.1114830Z 
2019-11-30T23:55:40.1116499Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen-units" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen-units" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-30T23:55:40.1118156Z 
2019-11-30T23:55:40.1118268Z 
2019-11-30T23:55:40.1118677Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-30T23:55:40.1118918Z Build completed unsuccessfully in 0:56:37
2019-11-30T23:55:40.1118918Z Build completed unsuccessfully in 0:56:37
2019-11-30T23:55:40.1169396Z == clock drift check ==
2019-11-30T23:55:40.1187021Z   local time: Sat Nov 30 23:55:40 UTC 2019
2019-11-30T23:55:40.3960527Z   network time: Sat, 30 Nov 2019 23:55:40 GMT
2019-11-30T23:55:40.3963335Z == end clock drift check ==
2019-11-30T23:55:44.8985343Z 
2019-11-30T23:55:44.9071828Z ##[error]Bash exited with code '1'.
2019-11-30T23:55:44.9104793Z ##[section]Starting: Checkout
2019-11-30T23:55:44.9106320Z ==============================================================================
2019-11-30T23:55:44.9106389Z Task         : Get sources
2019-11-30T23:55:44.9106432Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
