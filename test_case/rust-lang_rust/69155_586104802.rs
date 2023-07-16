plain
2020-02-14T04:18:15.5014286Z ========================== Starting Command Output ===========================
2020-02-14T04:18:15.5035325Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/90d2d012-7af0-4f24-ab8d-fb6c5513eebb.sh
2020-02-14T04:18:15.5219259Z 
2020-02-14T04:18:15.5282162Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-14T04:18:15.5287499Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69155/merge to s
2020-02-14T04:18:15.5289003Z Task         : Get sources
2020-02-14T04:18:15.5289074Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-14T04:18:15.5289104Z Version      : 1.0.0
2020-02-14T04:18:15.5289132Z Author       : Microsoft
---
2020-02-14T04:18:16.4474047Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-14T04:18:16.4621314Z ##[command]git config gc.auto 0
2020-02-14T04:18:16.4639842Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-14T04:18:16.4689572Z ##[command]git config --get-all http.proxy
2020-02-14T04:18:16.4844701Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69155/merge:refs/remotes/pull/69155/merge
---
2020-02-14T05:17:44.7435066Z .................................................................................................... 1700/9637
2020-02-14T05:17:49.6236555Z .................................................................................................... 1800/9637
2020-02-14T05:18:01.5131163Z ..............................i..................................................................... 1900/9637
2020-02-14T05:18:09.2997447Z .................................................................................................... 2000/9637
2020-02-14T05:18:23.8340730Z ....................iiiii........................................................................... 2100/9637
2020-02-14T05:18:33.7410239Z .................................................................................................... 2300/9637
2020-02-14T05:18:36.2407919Z .................................................................................................... 2400/9637
2020-02-14T05:18:41.0181968Z .................................................................................................... 2500/9637
2020-02-14T05:19:02.1137991Z .................................................................................................... 2600/9637
---
2020-02-14T05:21:40.8729983Z ........................................................................i...............i........... 4900/9637
2020-02-14T05:21:48.7788118Z .................................................................................................... 5000/9637
2020-02-14T05:21:57.2437472Z .................................................................................................... 5100/9637
2020-02-14T05:22:02.2211091Z ..............i..................................................................................... 5200/9637
2020-02-14T05:22:13.8708133Z ........................................................................................ii.ii....... 5300/9637
2020-02-14T05:22:17.9968800Z .i...i.............................................................................................. 5400/9637
2020-02-14T05:22:28.7798616Z .................................................................................................... 5600/9637
2020-02-14T05:22:39.1601180Z .............................................................................i...................... 5700/9637
2020-02-14T05:22:46.9228513Z .................................................................................................... 5800/9637
2020-02-14T05:22:53.4493103Z ...........................................................................i........................ 5900/9637
2020-02-14T05:22:53.4493103Z ...........................................................................i........................ 5900/9637
2020-02-14T05:23:03.3939132Z .....................................................................ii...i..ii...........i......... 6000/9637
2020-02-14T05:23:25.2889100Z .................................................................................................... 6200/9637
2020-02-14T05:23:32.6017254Z .................................................................................................... 6300/9637
2020-02-14T05:23:40.7919816Z .................................................................................................i.. 6400/9637
2020-02-14T05:23:54.1084681Z ii.................................................................................................. 6500/9637
---
2020-02-14T05:25:59.5265673Z .................................................................................................... 7600/9637
2020-02-14T05:26:03.8897353Z .................................................................................................... 7700/9637
2020-02-14T05:26:10.1388297Z .................................................................................................... 7800/9637
2020-02-14T05:26:18.1376945Z .................................................................................................... 7900/9637
2020-02-14T05:26:26.9629859Z ............................................................................iiiiiii.i............... 8000/9637
2020-02-14T05:26:43.3124836Z ................i......i............................................................................ 8200/9637
2020-02-14T05:26:48.8200612Z .................................................................................................... 8300/9637
2020-02-14T05:27:01.9662032Z .................................................................................................... 8400/9637
2020-02-14T05:27:11.8960713Z .................................................................................................... 8500/9637
---
2020-02-14T05:29:39.2335181Z  finished in 7.274
2020-02-14T05:29:39.2523120Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-14T05:29:39.4959332Z 
2020-02-14T05:29:39.4959593Z running 178 tests
2020-02-14T05:29:42.3906459Z iiii......i...........ii.Fiiii...i....i...........i............i..i..................i....i......... 100/178
2020-02-14T05:29:44.7049704Z ...i.i.i...iii..iiiiiiiiiiiiiiii.......................iii............ii......
2020-02-14T05:29:44.7051774Z 
2020-02-14T05:29:44.7052270Z ---- [codegen] codegen/consts.rs stdout ----
2020-02-14T05:29:44.7052373Z 
2020-02-14T05:29:44.7052373Z 
2020-02-14T05:29:44.7052648Z error: verification with 'FileCheck' failed
2020-02-14T05:29:44.7052700Z status: exit code: 1
2020-02-14T05:29:44.7053109Z command: "/usr/lib/llvm-7/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/consts/consts.ll" "/checkout/src/test/codegen/consts.rs"
2020-02-14T05:29:44.7053418Z ------------------------------------------
2020-02-14T05:29:44.7053470Z 
2020-02-14T05:29:44.7053716Z ------------------------------------------
2020-02-14T05:29:44.7053778Z stderr:
2020-02-14T05:29:44.7053778Z stderr:
2020-02-14T05:29:44.7054043Z ------------------------------------------
2020-02-14T05:29:44.7054107Z /checkout/src/test/codegen/consts.rs:46:12: error: CHECK: expected string not found in input
2020-02-14T05:29:44.7054172Z  // CHECK: load %"E<i16, [i16; 3]>"*, %"E<i16, [i16; 3]>"** bitcast (<{ i8*, [0 x i8] }>* @1 to %"E<i16, [i16; 3]>"**), align 8, !nonnull !1
2020-02-14T05:29:44.7054241Z            ^
2020-02-14T05:29:44.7054685Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/consts/consts.ll:41:28: note: scanning from here
2020-02-14T05:29:44.7054740Z define i64 @low_align_const() unnamed_addr #0 {
2020-02-14T05:29:44.7054803Z                            ^
2020-02-14T05:29:44.7055248Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/consts/consts.ll:44:8: note: possible intended match here
2020-02-14T05:29:44.7055316Z  %_2 = load %"E<i16, [i16; 3]>"*, %"E<i16, [i16; 3]>"** bitcast (<{ i8*, [0 x i8] }>* @1 to %"E<i16, [i16; 3]>"**), align 8, !nonnull !2
2020-02-14T05:29:44.7055443Z /checkout/src/test/codegen/consts.rs:54:12: error: CHECK: expected string not found in input
2020-02-14T05:29:44.7055443Z /checkout/src/test/codegen/consts.rs:54:12: error: CHECK: expected string not found in input
2020-02-14T05:29:44.7055511Z  // CHECK: load %"E<i16, i32>"*, %"E<i16, i32>"** bitcast (<{ i8*, [0 x i8] }>* @1 to %"E<i16, i32>"**), align 8, !nonnull !1
2020-02-14T05:29:44.7055579Z            ^
2020-02-14T05:29:44.7055900Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/consts/consts.ll:54:29: note: scanning from here
2020-02-14T05:29:44.7055956Z define i64 @high_align_const() unnamed_addr #0 {
2020-02-14T05:29:44.7056020Z                             ^
2020-02-14T05:29:44.7056345Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/consts/consts.ll:57:8: note: possible intended match here
2020-02-14T05:29:44.7056409Z  %_2 = load %"E<i16, i32>"*, %"E<i16, i32>"** bitcast (<{ i8*, [0 x i8] }>* @1 to %"E<i16, i32>"**), align 8, !nonnull !2
2020-02-14T05:29:44.7056504Z 
2020-02-14T05:29:44.7056967Z ------------------------------------------
2020-02-14T05:29:44.7057001Z 
2020-02-14T05:29:44.7057028Z 
---
2020-02-14T05:29:44.7059860Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-14T05:29:44.7060276Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-14T05:29:44.7063277Z 
2020-02-14T05:29:44.7063489Z 
2020-02-14T05:29:44.7068827Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-14T05:29:44.7070911Z 
2020-02-14T05:29:44.7070987Z 
2020-02-14T05:29:44.7079242Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-14T05:29:44.7080223Z Build completed unsuccessfully in 1:05:19
2020-02-14T05:29:44.7080223Z Build completed unsuccessfully in 1:05:19
2020-02-14T05:29:44.7139716Z == clock drift check ==
2020-02-14T05:29:44.7165955Z   local time: Fri Feb 14 05:29:44 UTC 2020
2020-02-14T05:29:45.0068115Z   network time: Fri, 14 Feb 2020 05:29:44 GMT
2020-02-14T05:29:45.0068912Z == end clock drift check ==
2020-02-14T05:29:47.2077230Z 
2020-02-14T05:29:47.2168823Z ##[error]Bash exited with code '1'.
2020-02-14T05:29:47.2183052Z ##[section]Finishing: Run build
2020-02-14T05:29:47.2207811Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69155/merge to s
2020-02-14T05:29:47.2209667Z Task         : Get sources
2020-02-14T05:29:47.2209730Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-14T05:29:47.2209791Z Version      : 1.0.0
2020-02-14T05:29:47.2209832Z Author       : Microsoft
2020-02-14T05:29:47.2209832Z Author       : Microsoft
2020-02-14T05:29:47.2209895Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-14T05:29:47.2209946Z ==============================================================================
2020-02-14T05:29:47.6721190Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-14T05:29:47.6762607Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69155/merge to s
2020-02-14T05:29:47.6873964Z Cleaning up task key
2020-02-14T05:29:47.6874695Z Start cleaning up orphan processes.
2020-02-14T05:29:47.6992405Z Terminate orphan process: pid (3364) (python)
2020-02-14T05:29:47.7223705Z ##[section]Finishing: Finalize Job
