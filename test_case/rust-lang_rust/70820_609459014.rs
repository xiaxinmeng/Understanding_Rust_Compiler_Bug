plain
2020-04-05T17:11:48.1554234Z ========================== Starting Command Output ===========================
2020-04-05T17:11:48.1556348Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/480b2068-de31-4cda-8f0b-db748f4fb453.sh
2020-04-05T17:11:48.1556552Z 
2020-04-05T17:11:48.1560020Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-05T17:11:48.1582523Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70820/merge to s
2020-04-05T17:11:48.1585203Z Task         : Get sources
2020-04-05T17:11:48.1585531Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-05T17:11:48.1585757Z Version      : 1.0.0
2020-04-05T17:11:48.1585907Z Author       : Microsoft
---
2020-04-05T17:11:49.1634834Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-05T17:11:49.1643647Z ##[command]git config gc.auto 0
2020-04-05T17:11:49.1652124Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-05T17:11:49.1661789Z ##[command]git config --get-all http.proxy
2020-04-05T17:11:49.1672855Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70820/merge:refs/remotes/pull/70820/merge
---
2020-04-05T17:13:53.5273328Z Looks like docker image is the same as before, not uploading
2020-04-05T17:14:01.7159671Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-05T17:14:01.7532511Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-05T17:14:01.7559645Z == clock drift check ==
2020-04-05T17:14:01.7566860Z   local time: Sun Apr  5 17:14:01 UTC 2020
2020-04-05T17:14:02.0466858Z   network time: Sun, 05 Apr 2020 17:14:02 GMT
2020-04-05T17:14:02.0491868Z Starting sccache server...
2020-04-05T17:14:02.1207582Z configure: processing command line
2020-04-05T17:14:02.1207810Z configure: 
2020-04-05T17:14:02.1208604Z configure: rust.dist-src        := False
---
2020-04-05T17:19:01.1866190Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-05T17:19:02.5894455Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-05T17:19:04.0255918Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-05T17:19:05.6365268Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-05T17:19:13.3541104Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-05T17:19:16.2739498Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-05T17:19:20.4862980Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-05T17:19:24.3430824Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-05T17:19:32.9632932Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-05T17:40:18.6710406Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-05T17:40:20.3110060Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-05T17:40:22.0846373Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-05T17:40:24.3854103Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-05T17:40:33.7487701Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-05T17:40:37.0958950Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-05T17:40:42.0183409Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-05T17:40:47.1886467Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-05T17:40:56.7681891Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-05T18:04:14.5922851Z .................................................................................................... 1700/9877
2020-04-05T18:04:18.2032609Z .................................................................................................... 1800/9877
2020-04-05T18:04:25.9909481Z .................................................................................................i.. 1900/9877
2020-04-05T18:04:32.9961258Z .................................................................................................... 2000/9877
2020-04-05T18:04:38.6330939Z .......................................................................................iiiii........ 2100/9877
2020-04-05T18:04:57.3108668Z .................................................................................................... 2300/9877
2020-04-05T18:04:59.1142423Z .................................................................................................... 2400/9877
2020-04-05T18:05:01.0236619Z .................................................................................................... 2500/9877
2020-04-05T18:05:06.4919900Z .................................................................................................... 2600/9877
---
2020-04-05T18:07:41.0351080Z .............................................................i...............i...................... 5000/9877
2020-04-05T18:07:48.0007596Z .................................................................................................... 5100/9877
2020-04-05T18:07:54.8251660Z .................................................................................................... 5200/9877
2020-04-05T18:07:59.5670201Z ......i............................................................................................. 5300/9877
2020-04-05T18:08:08.7051566Z ...............................................................................................ii.ii 5400/9877
2020-04-05T18:08:13.1944997Z ........i...i....................................................................................... 5500/9877
2020-04-05T18:08:21.1356152Z ........................................i........................................................... 5700/9877
2020-04-05T18:08:21.1356152Z ........................................i........................................................... 5700/9877
2020-04-05T18:08:29.9919515Z ............................................................ii.....................................i 5800/9877
2020-04-05T18:08:41.1714160Z .................................................................................................... 6000/9877
2020-04-05T18:08:41.1714160Z .................................................................................................... 6000/9877
2020-04-05T18:08:49.7525942Z .............................................................................................ii...i. 6100/9877
2020-04-05T18:09:00.3939951Z .ii...........i..................................................................................... 6200/9877
2020-04-05T18:09:15.4813019Z .................................................................................................... 6400/9877
2020-04-05T18:09:20.3857660Z .................................................................................................... 6500/9877
2020-04-05T18:09:20.3857660Z .................................................................................................... 6500/9877
2020-04-05T18:09:34.4517092Z .......................i..ii........................................................................ 6600/9877
2020-04-05T18:09:53.0429114Z .................................................................................................... 6800/9877
2020-04-05T18:09:54.8228616Z .......................i............................................................................ 6900/9877
2020-04-05T18:09:56.5964376Z .................................................................................................... 7000/9877
2020-04-05T18:09:58.4897285Z ..............................................................i..................................... 7100/9877
---
2020-04-05T18:11:26.9071254Z .................................................................................................... 7800/9877
2020-04-05T18:11:30.8332989Z .................................................................................................... 7900/9877
2020-04-05T18:11:36.1107163Z .................................................................................................... 8000/9877
2020-04-05T18:11:42.9234335Z ..........................i......................................................................... 8100/9877
2020-04-05T18:11:50.8324241Z ...........................................................................iiiiiiiiii.i............. 8200/9877
2020-04-05T18:12:05.7387064Z ...................i......i......................................................................... 8400/9877
2020-04-05T18:12:10.4260128Z .................................................................................................... 8500/9877
2020-04-05T18:12:20.7466999Z .................................................................................................... 8600/9877
2020-04-05T18:12:32.5664532Z .................................................................................................... 8700/9877
---
2020-04-05T18:14:18.2655776Z 
2020-04-05T18:14:18.2656577Z ---- [ui] ui/consts/assoc_const_generic_impl.rs stdout ----
2020-04-05T18:14:18.2657059Z diff of stderr:
2020-04-05T18:14:18.2657570Z 
2020-04-05T18:14:18.2657926Z 18 LL |         let () = Self::I_AM_ZERO_SIZED;
2020-04-05T18:14:18.2658686Z 20 
2020-04-05T18:14:18.2659374Z - error: aborting due to previous error
2020-04-05T18:14:18.2659871Z + error: erroneous constant encountered
2020-04-05T18:14:18.2660515Z +   --> $DIR/assoc_const_generic_impl.rs:13:18
2020-04-05T18:14:18.2660515Z +   --> $DIR/assoc_const_generic_impl.rs:13:18
2020-04-05T18:14:18.2661071Z +    |
2020-04-05T18:14:18.2661650Z + LL |         let () = Self::I_AM_ZERO_SIZED;
2020-04-05T18:14:18.2662629Z + 
2020-04-05T18:14:18.2662983Z + error: aborting due to 2 previous errors
2020-04-05T18:14:18.2663323Z 22 
2020-04-05T18:14:18.2663588Z 23 
2020-04-05T18:14:18.2663588Z 23 
2020-04-05T18:14:18.2663850Z 
2020-04-05T18:14:18.2664092Z 
2020-04-05T18:14:18.2664441Z The actual stderr differed from the expected stderr.
2020-04-05T18:14:18.2665547Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/assoc_const_generic_impl/assoc_const_generic_impl.stderr
2020-04-05T18:14:18.2666748Z To update references, rerun the tests and pass the `--bless` flag
2020-04-05T18:14:18.2667717Z To only update this specific test, also pass `--test-args consts/assoc_const_generic_impl.rs`
2020-04-05T18:14:18.2668514Z error: 1 errors occurred comparing output.
2020-04-05T18:14:18.2668881Z status: exit code: 1
2020-04-05T18:14:18.2668881Z status: exit code: 1
2020-04-05T18:14:18.2672573Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/assoc_const_generic_impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/assoc_const_generic_impl" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/assoc_const_generic_impl/auxiliary"
2020-04-05T18:14:18.2675636Z ------------------------------------------
2020-04-05T18:14:18.2675946Z 
2020-04-05T18:14:18.2679413Z ------------------------------------------
2020-04-05T18:14:18.2681464Z stderr:
2020-04-05T18:14:18.2681464Z stderr:
2020-04-05T18:14:18.2682200Z ------------------------------------------
2020-04-05T18:14:18.2683385Z warning: any use of this value will cause an error
2020-04-05T18:14:18.2685223Z   --> /checkout/src/test/ui/consts/assoc_const_generic_impl.rs:11:34
2020-04-05T18:14:18.2686040Z    |
2020-04-05T18:14:18.2686541Z LL |     const I_AM_ZERO_SIZED: ()  = [()][std::mem::size_of::<Self>()]; //~ WARN any use of this value
2020-04-05T18:14:18.2688547Z    |                                  |
2020-04-05T18:14:18.2689452Z    |                                  index out of bounds: the len is 1 but the index is 4
2020-04-05T18:14:18.2689909Z    |
2020-04-05T18:14:18.2690222Z note: the lint level is defined here
---
2020-04-05T18:14:18.2692269Z 
2020-04-05T18:14:18.2692881Z error: erroneous constant encountered
2020-04-05T18:14:18.2694423Z   --> /checkout/src/test/ui/consts/assoc_const_generic_impl.rs:13:18
2020-04-05T18:14:18.2694689Z    |
2020-04-05T18:14:18.2694957Z LL |         let () = Self::I_AM_ZERO_SIZED; //~ ERROR erroneous constant encountered
2020-04-05T18:14:18.2696067Z 
2020-04-05T18:14:18.2696294Z error: erroneous constant encountered
2020-04-05T18:14:18.2699460Z   --> /checkout/src/test/ui/consts/assoc_const_generic_impl.rs:13:18
2020-04-05T18:14:18.2700375Z    |
2020-04-05T18:14:18.2700375Z    |
2020-04-05T18:14:18.2701224Z LL |         let () = Self::I_AM_ZERO_SIZED; //~ ERROR erroneous constant encountered
2020-04-05T18:14:18.2702590Z 
2020-04-05T18:14:18.2703018Z error: aborting due to 2 previous errors
2020-04-05T18:14:18.2703379Z 
2020-04-05T18:14:18.2703482Z 
2020-04-05T18:14:18.2703482Z 
2020-04-05T18:14:18.2704065Z ------------------------------------------
2020-04-05T18:14:18.2704251Z 
2020-04-05T18:14:18.2704351Z 
2020-04-05T18:14:18.2704823Z ---- [ui] ui/consts/const-eval/index-out-of-bounds-never-type.rs stdout ----
2020-04-05T18:14:18.2705136Z diff of stderr:
2020-04-05T18:14:18.2705266Z 
2020-04-05T18:14:18.2705486Z 18 LL |     let _ = PrintName::<T>::VOID;
2020-04-05T18:14:18.2709533Z 20 
2020-04-05T18:14:18.2710045Z - error: aborting due to previous error
2020-04-05T18:14:18.2710313Z + error: erroneous constant encountered
2020-04-05T18:14:18.2711134Z +   --> $DIR/index-out-of-bounds-never-type.rs:15:13
2020-04-05T18:14:18.2711134Z +   --> $DIR/index-out-of-bounds-never-type.rs:15:13
2020-04-05T18:14:18.2711374Z +    |
2020-04-05T18:14:18.2711580Z + LL |     let _ = PrintName::<T>::VOID;
2020-04-05T18:14:18.2712029Z + 
2020-04-05T18:14:18.2712219Z + error: aborting due to 2 previous errors
2020-04-05T18:14:18.2712562Z 22 
2020-04-05T18:14:18.2712692Z 23 
2020-04-05T18:14:18.2712692Z 23 
2020-04-05T18:14:18.2712788Z 
2020-04-05T18:14:18.2712875Z 
2020-04-05T18:14:18.2713063Z The actual stderr differed from the expected stderr.
2020-04-05T18:14:18.2713787Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/index-out-of-bounds-never-type/index-out-of-bounds-never-type.stderr
2020-04-05T18:14:18.2714425Z To update references, rerun the tests and pass the `--bless` flag
2020-04-05T18:14:18.2715035Z To only update this specific test, also pass `--test-args consts/const-eval/index-out-of-bounds-never-type.rs`
2020-04-05T18:14:18.2715477Z error: 1 errors occurred comparing output.
2020-04-05T18:14:18.2715693Z status: exit code: 1
2020-04-05T18:14:18.2715693Z status: exit code: 1
2020-04-05T18:14:18.2719461Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/index-out-of-bounds-never-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/index-out-of-bounds-never-type" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/index-out-of-bounds-never-type/auxiliary"
2020-04-05T18:14:18.2721196Z ------------------------------------------
2020-04-05T18:14:18.2721367Z 
2020-04-05T18:14:18.2721725Z ------------------------------------------
2020-04-05T18:14:18.2721916Z stderr:
2020-04-05T18:14:18.2721916Z stderr:
2020-04-05T18:14:18.2722256Z ------------------------------------------
2020-04-05T18:14:18.2722531Z warning: any use of this value will cause an error
2020-04-05T18:14:18.2723095Z   --> /checkout/src/test/ui/consts/const-eval/index-out-of-bounds-never-type.rs:10:61
2020-04-05T18:14:18.2723376Z    |
2020-04-05T18:14:18.2723666Z LL |     const VOID: ! = { let x = 0 * std::mem::size_of::<T>(); [][x] };
2020-04-05T18:14:18.2724839Z    |                                                             |
2020-04-05T18:14:18.2725307Z    |                                                             index out of bounds: the len is 0 but the index is 0
2020-04-05T18:14:18.2725652Z    |
2020-04-05T18:14:18.2725834Z note: the lint level is defined here
---
2020-04-05T18:14:18.2727280Z 
2020-04-05T18:14:18.2727451Z error: erroneous constant encountered
2020-04-05T18:14:18.2728080Z   --> /checkout/src/test/ui/consts/const-eval/index-out-of-bounds-never-type.rs:15:13
2020-04-05T18:14:18.2728382Z    |
2020-04-05T18:14:18.2728578Z LL |     let _ = PrintName::<T>::VOID;
2020-04-05T18:14:18.2729155Z 
2020-04-05T18:14:18.2729320Z error: erroneous constant encountered
2020-04-05T18:14:18.2729850Z   --> /checkout/src/test/ui/consts/const-eval/index-out-of-bounds-never-type.rs:15:13
2020-04-05T18:14:18.2730114Z    |
2020-04-05T18:14:18.2730114Z    |
2020-04-05T18:14:18.2730321Z LL |     let _ = PrintName::<T>::VOID;
2020-04-05T18:14:18.2731117Z 
2020-04-05T18:14:18.2731330Z error: aborting due to 2 previous errors
2020-04-05T18:14:18.2731508Z 
2020-04-05T18:14:18.2731609Z 
---
2020-04-05T18:14:18.2736844Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-05T18:14:18.2737368Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-05T18:14:18.2737588Z 
2020-04-05T18:14:18.2739487Z 
2020-04-05T18:14:18.2744039Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-05T18:14:18.2746683Z 
2020-04-05T18:14:18.2746776Z 
2020-04-05T18:14:18.2747297Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-05T18:14:18.2747652Z Build completed unsuccessfully in 0:58:35
2020-04-05T18:14:18.2747652Z Build completed unsuccessfully in 0:58:35
2020-04-05T18:14:18.2779064Z == clock drift check ==
2020-04-05T18:14:18.2804037Z   local time: Sun Apr  5 18:14:18 UTC 2020
2020-04-05T18:14:18.5752207Z   network time: Sun, 05 Apr 2020 18:14:18 GMT
2020-04-05T18:14:19.0394123Z 
2020-04-05T18:14:19.0394123Z 
2020-04-05T18:14:19.0461837Z ##[error]Bash exited with code '1'.
2020-04-05T18:14:19.0473977Z ##[section]Finishing: Run build
2020-04-05T18:14:19.0524125Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70820/merge to s
2020-04-05T18:14:19.0529712Z Task         : Get sources
2020-04-05T18:14:19.0530312Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-05T18:14:19.0530594Z Version      : 1.0.0
2020-04-05T18:14:19.0530954Z Author       : Microsoft
2020-04-05T18:14:19.0530954Z Author       : Microsoft
2020-04-05T18:14:19.0531254Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-05T18:14:19.0531621Z ==============================================================================
2020-04-05T18:14:19.3547049Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-05T18:14:19.3594122Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70820/merge to s
2020-04-05T18:14:19.3676041Z Cleaning up task key
2020-04-05T18:14:19.3677572Z Start cleaning up orphan processes.
2020-04-05T18:14:19.3836080Z Terminate orphan process: pid (3942) (python)
2020-04-05T18:14:19.4026672Z ##[section]Finishing: Finalize Job
