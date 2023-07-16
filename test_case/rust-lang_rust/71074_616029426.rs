plain
2020-04-19T03:28:40.4746007Z ========================== Starting Command Output ===========================
2020-04-19T03:28:40.4750851Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/fd42af69-3f9c-4eac-b646-6fd7c0d33116.sh
2020-04-19T03:28:40.4751339Z 
2020-04-19T03:28:40.4756241Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-19T03:28:40.4775125Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71074/merge to s
2020-04-19T03:28:40.4778879Z Task         : Get sources
2020-04-19T03:28:40.4779163Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-19T03:28:40.4779424Z Version      : 1.0.0
2020-04-19T03:28:40.4779606Z Author       : Microsoft
---
2020-04-19T03:28:41.6352251Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-19T03:28:41.6360820Z ##[command]git config gc.auto 0
2020-04-19T03:28:41.6367224Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-19T03:28:41.6376328Z ##[command]git config --get-all http.proxy
2020-04-19T03:28:41.6388867Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71074/merge:refs/remotes/pull/71074/merge
---
2020-04-19T03:32:00.4629912Z  ---> 318032b5f0e2
2020-04-19T03:32:00.4630794Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-19T03:32:00.4637247Z  ---> Using cache
2020-04-19T03:32:00.4637681Z  ---> d44a858fd1ce
2020-04-19T03:32:00.4638746Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-19T03:32:00.4646356Z  ---> 58b910f50f5a
2020-04-19T03:32:00.4646569Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-19T03:32:00.4651735Z  ---> Using cache
2020-04-19T03:32:00.4652121Z  ---> ee7702aadba1
---
2020-04-19T03:32:00.5548404Z Looks like docker image is the same as before, not uploading
2020-04-19T03:32:08.9859200Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-19T03:32:09.0222790Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-19T03:32:09.0262240Z == clock drift check ==
2020-04-19T03:32:09.0271162Z   local time: Sun Apr 19 03:32:09 UTC 2020
2020-04-19T03:32:09.1182192Z   network time: Sun, 19 Apr 2020 03:32:09 GMT
2020-04-19T03:32:09.1208785Z Starting sccache server...
2020-04-19T03:32:09.2088682Z configure: processing command line
2020-04-19T03:32:09.2088963Z configure: 
2020-04-19T03:32:09.2089964Z configure: rust.dist-src        := False
---
2020-04-19T03:38:04.3539307Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-19T03:38:06.0742130Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-19T03:38:07.8525779Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-19T03:38:09.8851877Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-19T03:38:19.1949348Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-19T03:38:22.9919488Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-19T03:38:27.9810412Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-19T03:38:32.7426276Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-19T03:38:42.3408585Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-19T04:03:23.2614386Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-19T04:03:25.0563178Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-19T04:03:27.0153026Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-19T04:03:27.6288318Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-19T04:03:38.6347790Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-19T04:03:41.7656817Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-19T04:03:46.9612440Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-19T04:03:51.8091602Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-19T04:04:02.6170773Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-19T04:29:08.0449542Z .................................................................................................... 1700/9907
2020-04-19T04:29:12.2801009Z .................................................................................................... 1800/9907
2020-04-19T04:29:21.3110387Z ..................................................................................................i. 1900/9907
2020-04-19T04:29:29.4579356Z .................................................................................................... 2000/9907
2020-04-19T04:29:35.9044163Z ........................................................................................iiiii....... 2100/9907
2020-04-19T04:29:56.8412977Z .................................................................................................... 2300/9907
2020-04-19T04:29:59.0780345Z .................................................................................................... 2400/9907
2020-04-19T04:30:01.4133772Z .................................................................................................... 2500/9907
2020-04-19T04:30:07.5666669Z .................................................................................................... 2600/9907
---
2020-04-19T04:33:03.1083927Z ................................................................i...............i................... 5000/9907
2020-04-19T04:33:10.6896045Z .................................................................................................... 5100/9907
2020-04-19T04:33:17.8431640Z .................................................................................................... 5200/9907
2020-04-19T04:33:23.0487833Z ..........i......................................................................................... 5300/9907
2020-04-19T04:33:33.0424370Z i................................................................................................... 5400/9907
2020-04-19T04:33:38.5229425Z ii.ii........i...i.................................................................................. 5500/9907
2020-04-19T04:33:45.8229211Z ...............................................i.................................................... 5700/9907
2020-04-19T04:33:55.1396602Z ...............................................................................ii................... 5800/9907
2020-04-19T04:34:02.3522641Z ..................i................................................................................. 5900/9907
2020-04-19T04:34:07.8641431Z .................................................................................................... 6000/9907
2020-04-19T04:34:07.8641431Z .................................................................................................... 6000/9907
2020-04-19T04:34:18.7091381Z .................................................................................................... 6100/9907
2020-04-19T04:34:29.3541071Z ............ii...i...ii..........i.................................................................. 6200/9907
2020-04-19T04:34:45.3307513Z .................................................................................................... 6400/9907
2020-04-19T04:34:52.2063099Z .................................................................................................... 6500/9907
2020-04-19T04:34:52.2063099Z .................................................................................................... 6500/9907
2020-04-19T04:35:08.7771170Z ..........................................i..ii..................................................... 6600/9907
2020-04-19T04:35:31.5889034Z .................................................................................................... 6800/9907
2020-04-19T04:35:33.8132088Z ...........................................i........................................................ 6900/9907
2020-04-19T04:35:35.9573202Z .................................................................................................... 7000/9907
2020-04-19T04:35:38.1687558Z ...................................................................................i................ 7100/9907
---
2020-04-19T04:37:14.6760723Z .................................................................................................... 7800/9907
2020-04-19T04:37:19.3375235Z .................................................................................................... 7900/9907
2020-04-19T04:37:25.9675491Z .......................................................................F............................ 8000/9907
2020-04-19T04:37:31.8831621Z .................................................i.................................................. 8100/9907
2020-04-19T04:37:42.0225262Z ..................................................................................................ii 8200/9907
2020-04-19T04:37:47.4226481Z iiii.iiiii.i........................................................................................ 8300/9907
2020-04-19T04:38:01.3281168Z .................................................................................................... 8500/9907
2020-04-19T04:38:09.6724607Z .................................................................................................... 8600/9907
2020-04-19T04:38:23.8653010Z .................................................................................................... 8700/9907
2020-04-19T04:38:30.8689753Z .................................................................................................... 8800/9907
---
2020-04-19T04:40:23.1315476Z ---- [ui] ui/pattern/usefulness/match-privately-empty.rs stdout ----
2020-04-19T04:40:23.1315734Z 
2020-04-19T04:40:23.1315943Z error: ui test compiled successfully!
2020-04-19T04:40:23.1316162Z status: exit code: 0
2020-04-19T04:40:23.1318566Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/usefulness/match-privately-empty.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/usefulness/match-privately-empty" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/usefulness/match-privately-empty/auxiliary"
2020-04-19T04:40:23.1320493Z ------------------------------------------
2020-04-19T04:40:23.1320662Z 
2020-04-19T04:40:23.1321022Z ------------------------------------------
2020-04-19T04:40:23.1321216Z stderr:
---
2020-04-19T04:40:23.1322805Z ---- [ui] ui/rfc-2008-non-exhaustive/uninhabited/patterns.rs stdout ----
2020-04-19T04:40:23.1323014Z 
2020-04-19T04:40:23.1324144Z error: test compilation failed although it shouldn't!
2020-04-19T04:40:23.1324409Z status: exit code: 1
2020-04-19T04:40:23.1326426Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2008-non-exhaustive/uninhabited/patterns.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2008-non-exhaustive/uninhabited/patterns" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2008-non-exhaustive/uninhabited/patterns/auxiliary"
2020-04-19T04:40:23.1327977Z ------------------------------------------
2020-04-19T04:40:23.1328156Z 
2020-04-19T04:40:23.1328510Z ------------------------------------------
2020-04-19T04:40:23.1328699Z stderr:
2020-04-19T04:40:23.1328699Z stderr:
2020-04-19T04:40:23.1329275Z ------------------------------------------
2020-04-19T04:40:23.1329513Z error: unreachable pattern
2020-04-19T04:40:23.1330068Z   --> /checkout/src/test/ui/rfc-2008-non-exhaustive/uninhabited/patterns.rs:51:15
2020-04-19T04:40:23.1330351Z    |
2020-04-19T04:40:23.1330648Z LL |     while let PartiallyInhabitedVariants::Struct { x, .. } = partially_inhabited_variant() {
2020-04-19T04:40:23.1331261Z    |
2020-04-19T04:40:23.1331436Z note: the lint level is defined here
2020-04-19T04:40:23.1331958Z   --> /checkout/src/test/ui/rfc-2008-non-exhaustive/uninhabited/patterns.rs:3:9
2020-04-19T04:40:23.1332222Z    |
---
2020-04-19T04:40:23.1351244Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-19T04:40:23.1351646Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-19T04:40:23.1373113Z 
2020-04-19T04:40:23.1373270Z 
2020-04-19T04:40:23.1377455Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-19T04:40:23.1380114Z 
2020-04-19T04:40:23.1380221Z 
2020-04-19T04:40:23.1383564Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-19T04:40:23.1384024Z Build completed unsuccessfully in 1:06:21
2020-04-19T04:40:23.1384024Z Build completed unsuccessfully in 1:06:21
2020-04-19T04:40:23.1453044Z == clock drift check ==
2020-04-19T04:40:23.1471674Z   local time: Sun Apr 19 04:40:23 UTC 2020
2020-04-19T04:40:23.4764702Z   network time: Sun, 19 Apr 2020 04:40:23 GMT
2020-04-19T04:40:24.1095128Z 
2020-04-19T04:40:24.1095128Z 
2020-04-19T04:40:24.1170110Z ##[error]Bash exited with code '1'.
2020-04-19T04:40:24.1185179Z ##[section]Finishing: Run build
2020-04-19T04:40:24.1232174Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71074/merge to s
2020-04-19T04:40:24.1239317Z Task         : Get sources
2020-04-19T04:40:24.1239639Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-19T04:40:24.1239966Z Version      : 1.0.0
2020-04-19T04:40:24.1240195Z Author       : Microsoft
2020-04-19T04:40:24.1240195Z Author       : Microsoft
2020-04-19T04:40:24.1240528Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-19T04:40:24.1241028Z ==============================================================================
2020-04-19T04:40:24.4957352Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-19T04:40:24.5006324Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71074/merge to s
2020-04-19T04:40:24.5116175Z Cleaning up task key
2020-04-19T04:40:24.5117638Z Start cleaning up orphan processes.
2020-04-19T04:40:24.5343498Z Terminate orphan process: pid (3731) (python)
2020-04-19T04:40:24.5574555Z ##[section]Finishing: Finalize Job
