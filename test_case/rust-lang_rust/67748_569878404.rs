plain
2019-12-31T05:54:55.0110225Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-31T05:54:55.5338099Z ##[command]git config gc.auto 0
2019-12-31T05:54:55.5341733Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-31T05:54:55.5344103Z ##[command]git config --get-all http.proxy
2019-12-31T05:54:55.5347143Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67748/merge:refs/remotes/pull/67748/merge
---
2019-12-31T06:56:43.8772755Z .................................................................................................... 1500/9464
2019-12-31T06:56:49.9401832Z .................................................................................................... 1600/9464
2019-12-31T06:56:55.0617895Z .................................................................................................... 1700/9464
2019-12-31T06:57:04.6458697Z .................................................................................................... 1800/9464
2019-12-31T06:57:12.8514572Z i................................................................................................... 1900/9464
2019-12-31T06:57:19.6709884Z ......................................................................................iiiii......... 2000/9464
2019-12-31T06:57:41.9642172Z .................................................................................................... 2200/9464
2019-12-31T06:57:44.4166358Z .................................................................................................... 2300/9464
2019-12-31T06:57:46.9551083Z .................................................................................................... 2400/9464
2019-12-31T06:57:53.1359612Z .................................................................................................... 2500/9464
---
2019-12-31T07:00:54.9323584Z .................i...............i.................................................................. 4900/9464
2019-12-31T07:01:05.0204295Z .................................................................................................... 5000/9464
2019-12-31T07:01:10.9245691Z ..............................................................i..................................... 5100/9464
2019-12-31T07:01:19.3764255Z .................................................................................................... 5200/9464
2019-12-31T07:01:27.1439038Z .............................ii.ii...........i...................................................... 5300/9464
2019-12-31T07:01:36.6209804Z .................................................................................................... 5500/9464
2019-12-31T07:01:46.9341690Z .................................................................................................... 5600/9464
2019-12-31T07:01:54.1328535Z ............i....................................................................................... 5700/9464
2019-12-31T07:02:00.4006748Z .................................................................................................... 5800/9464
2019-12-31T07:02:00.4006748Z .................................................................................................... 5800/9464
2019-12-31T07:02:11.4444752Z .................................................................................................... 5900/9464
2019-12-31T07:02:23.2059162Z ii...i..ii...........i.............................................................................. 6000/9464
2019-12-31T07:02:41.5187804Z .................................................................................................... 6200/9464
2019-12-31T07:02:48.9911252Z .................................................................................................... 6300/9464
2019-12-31T07:02:48.9911252Z .................................................................................................... 6300/9464
2019-12-31T07:03:07.3768527Z ...........................i..ii.................................................................... 6400/9464
2019-12-31T07:03:27.8669594Z .................................................................................................... 6600/9464
2019-12-31T07:03:30.0599820Z ...i................................................................................................ 6700/9464
2019-12-31T07:03:32.4682625Z .................................................................................................... 6800/9464
2019-12-31T07:03:35.1342597Z ..i................................................................................................. 6900/9464
---
2019-12-31T07:05:14.7682645Z .................................................................................................... 7500/9464
2019-12-31T07:05:20.0153992Z .................................................................................................... 7600/9464
2019-12-31T07:05:25.6975722Z .................................................................................................... 7700/9464
2019-12-31T07:05:36.0097810Z .................................................................................................... 7800/9464
2019-12-31T07:05:43.8437234Z ..................................iiii.............................................................. 7900/9464
2019-12-31T07:05:59.2934889Z .................................................................................................... 8100/9464
2019-12-31T07:06:08.2256503Z .................................................................................................... 8200/9464
2019-12-31T07:06:22.7605807Z .................................................................................................... 8300/9464
2019-12-31T07:06:30.5996774Z .................................................................................................... 8400/9464
---
2019-12-31T07:08:53.2625045Z  finished in 6.704
2019-12-31T07:08:53.2831970Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-31T07:08:53.4762296Z 
2019-12-31T07:08:53.4767346Z running 166 tests
2019-12-31T07:08:56.6747401Z iiii......i........ii..iiii...i.....F...........F...........i..i..................i....i............ 100/166
2019-12-31T07:08:58.8442248Z i.i.i...iii..iiiiiii.......................iii............ii......
2019-12-31T07:08:58.8442666Z 
2019-12-31T07:08:58.8443050Z ---- [codegen] codegen/force-frame-pointers.rs stdout ----
2019-12-31T07:08:58.8443088Z 
2019-12-31T07:08:58.8443088Z 
2019-12-31T07:08:58.8443360Z error: verification with 'FileCheck' failed
2019-12-31T07:08:58.8443411Z status: exit code: 1
2019-12-31T07:08:58.8443836Z command: "/usr/lib/llvm-7/bin/FileCheck" "/usr/lib/llvm-7/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/force-frame-pointers/force-frame-pointers.ll" "/checkout/src/test/codegen/force-frame-pointers.rs"
2019-12-31T07:08:58.8444138Z ------------------------------------------
2019-12-31T07:08:58.8444170Z 
2019-12-31T07:08:58.8444682Z ------------------------------------------
2019-12-31T07:08:58.8444739Z stderr:
2019-12-31T07:08:58.8444739Z stderr:
2019-12-31T07:08:58.8444997Z ------------------------------------------
2019-12-31T07:08:58.8445306Z /checkout/src/test/codegen/force-frame-pointers.rs:6:11: error: CHECK: expected string not found in input
2019-12-31T07:08:58.8445564Z // CHECK: attributes #{{.*}} "frame-pointer"="all"
2019-12-31T07:08:58.8445781Z           ^
2019-12-31T07:08:58.8446194Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/force-frame-pointers/force-frame-pointers.ll:1:1: note: scanning from here
2019-12-31T07:08:58.8446624Z ; ModuleID = 'force_frame_pointers.3a1fbbbh-cgu.0'
2019-12-31T07:08:58.8446680Z ^
2019-12-31T07:08:58.8447061Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/force-frame-pointers/force-frame-pointers.ll:13:1: note: possible intended match here
2019-12-31T07:08:58.8447574Z attributes #0 = { nonlazybind uwtable "no-frame-pointer-elim"="true" "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
2019-12-31T07:08:58.8447660Z 
2019-12-31T07:08:58.8447899Z ------------------------------------------
2019-12-31T07:08:58.8447931Z 
2019-12-31T07:08:58.8447957Z 
2019-12-31T07:08:58.8447957Z 
2019-12-31T07:08:58.8448388Z ---- [codegen] codegen/instrument-mcount.rs stdout ----
2019-12-31T07:08:58.8448465Z 
2019-12-31T07:08:58.8448883Z error: verification with 'FileCheck' failed
2019-12-31T07:08:58.8448939Z status: exit code: 1
2019-12-31T07:08:58.8449536Z command: "/usr/lib/llvm-7/bin/FileCheck" "/usr/lib/llvm-7/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/instrument-mcount/instrument-mcount.ll" "/checkout/src/test/codegen/instrument-mcount.rs"
2019-12-31T07:08:58.8450259Z ------------------------------------------
2019-12-31T07:08:58.8450302Z 
2019-12-31T07:08:58.8451364Z ------------------------------------------
2019-12-31T07:08:58.8451425Z stderr:
2019-12-31T07:08:58.8451425Z stderr:
2019-12-31T07:08:58.8451641Z ------------------------------------------
2019-12-31T07:08:58.8451947Z /checkout/src/test/codegen/instrument-mcount.rs:6:11: error: CHECK: expected string not found in input
2019-12-31T07:08:58.8452232Z // CHECK: attributes #{{.*}} "frame-pointer"="all" "instrument-function-entry-inlined"="{{.*}}mcount{{.*}}"
2019-12-31T07:08:58.8452464Z           ^
2019-12-31T07:08:58.8452825Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/instrument-mcount/instrument-mcount.ll:1:1: note: scanning from here
2019-12-31T07:08:58.8453062Z ; ModuleID = 'instrument_mcount.3a1fbbbh-cgu.0'
2019-12-31T07:08:58.8453193Z ^
2019-12-31T07:08:58.8453559Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/instrument-mcount/instrument-mcount.ll:13:22: note: possible intended match here
2019-12-31T07:08:58.8453946Z attributes #0 = { norecurse nounwind nonlazybind readnone uwtable "instrument-function-entry-inlined"="mcount" "no-frame-pointer-elim"="true" "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
2019-12-31T07:08:58.8454050Z 
2019-12-31T07:08:58.8454265Z ------------------------------------------
2019-12-31T07:08:58.8454296Z 
2019-12-31T07:08:58.8454321Z 
---
2019-12-31T07:08:58.8458593Z test result: FAILED. 130 passed; 2 failed; 34 ignored; 0 measured; 0 filtered out
2019-12-31T07:08:58.8458655Z 
2019-12-31T07:08:58.8458682Z 
2019-12-31T07:08:58.8458706Z 
2019-12-31T07:08:58.8461032Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-31T07:08:58.8461345Z 
2019-12-31T07:08:58.8461552Z 
2019-12-31T07:08:58.8465552Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2019-12-31T07:08:58.8465645Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-31T07:08:58.8465645Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-31T07:08:58.8468884Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-31T07:08:58.8468945Z Build completed unsuccessfully in 1:07:13
2019-12-31T07:08:58.8519063Z == clock drift check ==
2019-12-31T07:08:58.8542712Z   local time: Tue Dec 31 07:08:58 UTC 2019
2019-12-31T07:08:59.3948443Z   network time: Tue, 31 Dec 2019 07:08:59 GMT
2019-12-31T07:08:59.3949187Z == end clock drift check ==
2019-12-31T07:09:05.0505775Z 
2019-12-31T07:09:05.0608475Z ##[error]Bash exited with code '1'.
2019-12-31T07:09:05.0669301Z ##[section]Starting: Checkout
2019-12-31T07:09:05.0671206Z ==============================================================================
2019-12-31T07:09:05.0671278Z Task         : Get sources
2019-12-31T07:09:05.0671321Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
