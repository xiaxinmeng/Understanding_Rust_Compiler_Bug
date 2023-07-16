plain
2020-04-15T21:34:55.0023914Z ========================== Starting Command Output ===========================
2020-04-15T21:34:55.0026332Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/005aa340-4f51-4db2-83ea-f14106950d59.sh
2020-04-15T21:34:55.0026583Z 
2020-04-15T21:34:55.0030238Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-15T21:34:55.0047926Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71174/merge to s
2020-04-15T21:34:55.0050891Z Task         : Get sources
2020-04-15T21:34:55.0051165Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-15T21:34:55.0051566Z Version      : 1.0.0
2020-04-15T21:34:55.0051738Z Author       : Microsoft
---
2020-04-15T21:34:55.9897508Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-15T21:34:55.9907292Z ##[command]git config gc.auto 0
2020-04-15T21:34:55.9913433Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-15T21:34:55.9919186Z ##[command]git config --get-all http.proxy
2020-04-15T21:34:55.9929336Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71174/merge:refs/remotes/pull/71174/merge
---
2020-04-15T21:37:04.4489973Z  ---> f58a2bb1e753
2020-04-15T21:37:04.4499737Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-7       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-15T21:37:04.4500318Z  ---> Using cache
2020-04-15T21:37:04.4500634Z  ---> d079cc6b6db8
2020-04-15T21:37:04.4501458Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-15T21:37:04.4502372Z  ---> 4183ca46ee56
2020-04-15T21:37:04.4502556Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-15T21:37:04.4502874Z  ---> Using cache
2020-04-15T21:37:04.4503175Z  ---> 69e7f8a2a2fb
---
2020-04-15T21:37:04.4875413Z Looks like docker image is the same as before, not uploading
2020-04-15T21:37:12.4984396Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-15T21:37:12.5244816Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-15T21:37:12.5271903Z == clock drift check ==
2020-04-15T21:37:12.5286786Z   local time: Wed Apr 15 21:37:12 UTC 2020
2020-04-15T21:37:12.7521539Z   network time: Wed, 15 Apr 2020 21:37:12 GMT
2020-04-15T21:37:12.7545001Z Starting sccache server...
2020-04-15T21:37:12.8343257Z configure: processing command line
2020-04-15T21:37:12.8343611Z configure: 
2020-04-15T21:37:12.8344595Z configure: rust.dist-src        := False
---
2020-04-15T21:42:03.8018122Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-15T21:42:05.1765445Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-15T21:42:06.6319658Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-15T21:42:07.3880638Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-15T21:42:15.9547011Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-15T21:42:17.8821998Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-15T21:42:21.9734434Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-15T21:42:25.8609363Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-15T21:42:34.7506620Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-15T22:03:25.4618109Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-15T22:03:27.0791310Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-15T22:03:28.9200799Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-15T22:03:30.1134966Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-15T22:03:40.6212349Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-15T22:03:42.8774441Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-15T22:03:47.8375123Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-15T22:03:52.8344543Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-15T22:04:03.9303840Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-15T22:27:58.3312997Z .................................................................................................... 1700/9898
2020-04-15T22:28:02.5161700Z .................................................................................................... 1800/9898
2020-04-15T22:28:10.1161043Z .................................................................................................... 1900/9898
2020-04-15T22:28:17.9592312Z ........i........................................................................................... 2000/9898
2020-04-15T22:28:23.9117197Z ..................................................................................................ii 2100/9898
2020-04-15T22:28:37.3491356Z iii................................................................................................. 2200/9898
2020-04-15T22:28:45.0735546Z .................................................................................................... 2400/9898
2020-04-15T22:28:47.1072305Z .................................................................................................... 2500/9898
2020-04-15T22:28:52.3863605Z .................................................................................................... 2600/9898
2020-04-15T22:29:10.7342807Z .................................................................................................... 2700/9898
---
2020-04-15T22:31:45.2721613Z .................................................................................................... 5100/9898
2020-04-15T22:31:52.5299653Z .................................................................................................... 5200/9898
2020-04-15T22:31:56.8424388Z ...................i................................................................................ 5300/9898
2020-04-15T22:32:06.2351384Z .........i.......................................................................................... 5400/9898
2020-04-15T22:32:11.4979150Z .........ii.ii........i...i......................................................................... 5500/9898
2020-04-15T22:32:18.6493748Z .......................................................i............................................ 5700/9898
2020-04-15T22:32:27.6521779Z ...........................................................................ii....................... 5800/9898
2020-04-15T22:32:33.8909237Z ..............i..................................................................................... 5900/9898
2020-04-15T22:32:38.9689040Z .................................................................................................... 6000/9898
2020-04-15T22:32:38.9689040Z .................................................................................................... 6000/9898
2020-04-15T22:32:49.0043089Z .................................................................................................... 6100/9898
2020-04-15T22:32:59.2140716Z ........ii...i..ii...........i...................................................................... 6200/9898
2020-04-15T22:33:13.3751639Z .................................................................................................... 6400/9898
2020-04-15T22:33:16.6207519Z .................................................................................................... 6500/9898
2020-04-15T22:33:16.6207519Z .................................................................................................... 6500/9898
2020-04-15T22:33:27.9750049Z ......................................i..ii......................................................... 6600/9898
2020-04-15T22:33:49.1571418Z .................................................................................................... 6800/9898
2020-04-15T22:33:51.1471541Z ......................................i............................................................. 6900/9898
2020-04-15T22:33:53.1064242Z .................................................................................................... 7000/9898
2020-04-15T22:33:55.1187885Z .............................................................................i...................... 7100/9898
---
2020-04-15T22:35:26.6120686Z .................................................................................................... 7800/9898
2020-04-15T22:35:30.6519530Z .................................................................................................... 7900/9898
2020-04-15T22:35:36.7620826Z .................................................................................................... 8000/9898
2020-04-15T22:35:42.5294856Z ...........................................i........................................................ 8100/9898
2020-04-15T22:35:51.5123156Z ...........................................................................................iiiiii.ii 8200/9898
2020-04-15T22:35:57.2234842Z iii.i............................................................................................... 8300/9898
2020-04-15T22:36:09.4257582Z .................................................................................................... 8500/9898
2020-04-15T22:36:18.0817520Z .................................................................................................... 8600/9898
2020-04-15T22:36:30.6296713Z .................................................................................................... 8700/9898
2020-04-15T22:36:36.7176358Z .................................................................................................... 8800/9898
---
2020-04-15T22:38:20.3751457Z failures:
2020-04-15T22:38:20.3784732Z 
2020-04-15T22:38:20.3785502Z ---- [ui] ui/async-await/issue-68523-start.rs stdout ----
2020-04-15T22:38:20.3785842Z 
2020-04-15T22:38:20.3786490Z error: /checkout/src/test/ui/async-await/issue-68523-start.rs:6: unexpected error: '6:5: 6:10: start is not allowed to be `async` [E0752]'
2020-04-15T22:38:20.3787041Z error: 1 unexpected errors found, 0 expected errors not found
2020-04-15T22:38:20.3787284Z status: exit code: 1
2020-04-15T22:38:20.3787284Z status: exit code: 1
2020-04-15T22:38:20.3789141Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-68523-start.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-68523-start" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-68523-start/auxiliary"
2020-04-15T22:38:20.3790500Z     Error {
2020-04-15T22:38:20.3790667Z         line_num: 6,
2020-04-15T22:38:20.3790848Z         kind: Some(
2020-04-15T22:38:20.3791030Z             Error,
2020-04-15T22:38:20.3791030Z             Error,
2020-04-15T22:38:20.3791179Z         ),
2020-04-15T22:38:20.3791421Z         msg: "6:5: 6:10: start is not allowed to be `async` [E0752]",
2020-04-15T22:38:20.3791786Z ]
2020-04-15T22:38:20.3791878Z 
2020-04-15T22:38:20.3792576Z thread '[ui] ui/async-await/issue-68523-start.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1460:13
2020-04-15T22:38:20.3792969Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-15T22:38:20.3792969Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-15T22:38:20.3793159Z 
2020-04-15T22:38:20.3793516Z ---- [ui] ui/async-await/issue-68523.rs stdout ----
2020-04-15T22:38:20.3793670Z 
2020-04-15T22:38:20.3794294Z error: /checkout/src/test/ui/async-await/issue-68523.rs:3: unexpected error: '3:20: 3:35: `main` has invalid return type `impl std::future::Future` [E0277]'
2020-04-15T22:38:20.3794637Z 
2020-04-15T22:38:20.3795214Z error: /checkout/src/test/ui/async-await/issue-68523.rs:3: unexpected error: '3:1: 3:6: `main` function is not allowed to be `async` [E0752]'
2020-04-15T22:38:20.3795702Z error: 2 unexpected errors found, 0 expected errors not found
2020-04-15T22:38:20.3796197Z status: exit code: 1
2020-04-15T22:38:20.3796197Z status: exit code: 1
2020-04-15T22:38:20.3797864Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-68523.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-68523" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-68523/auxiliary"
2020-04-15T22:38:20.3799326Z     Error {
2020-04-15T22:38:20.3799504Z         line_num: 3,
2020-04-15T22:38:20.3799684Z         kind: Some(
2020-04-15T22:38:20.3799855Z             Error,
2020-04-15T22:38:20.3799855Z             Error,
2020-04-15T22:38:20.3800003Z         ),
2020-04-15T22:38:20.3800315Z         msg: "3:20: 3:35: `main` has invalid return type `impl std::future::Future` [E0277]",
2020-04-15T22:38:20.3800737Z     Error {
2020-04-15T22:38:20.3800914Z         line_num: 3,
2020-04-15T22:38:20.3801182Z         kind: Some(
2020-04-15T22:38:20.3801351Z             Error,
2020-04-15T22:38:20.3801351Z             Error,
2020-04-15T22:38:20.3801514Z         ),
2020-04-15T22:38:20.3801766Z         msg: "3:1: 3:6: `main` function is not allowed to be `async` [E0752]",
2020-04-15T22:38:20.3805253Z ]
2020-04-15T22:38:20.3805376Z 
2020-04-15T22:38:20.3806197Z thread '[ui] ui/async-await/issue-68523.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1460:13
2020-04-15T22:38:20.3806475Z 
---
2020-04-15T22:38:20.3808351Z 
2020-04-15T22:38:20.3808862Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-15T22:38:20.3819477Z 
2020-04-15T22:38:20.3819648Z 
2020-04-15T22:38:20.3823287Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-15T22:38:20.3825591Z 
2020-04-15T22:38:20.3825684Z 
2020-04-15T22:38:20.3829114Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-15T22:38:20.3829465Z Build completed unsuccessfully in 0:59:36
2020-04-15T22:38:20.3829465Z Build completed unsuccessfully in 0:59:36
2020-04-15T22:38:20.3881654Z == clock drift check ==
2020-04-15T22:38:20.3897291Z   local time: Wed Apr 15 22:38:20 UTC 2020
2020-04-15T22:38:20.4785692Z   network time: Wed, 15 Apr 2020 22:38:20 GMT
2020-04-15T22:38:20.9552790Z 
2020-04-15T22:38:20.9552790Z 
2020-04-15T22:38:20.9642962Z ##[error]Bash exited with code '1'.
2020-04-15T22:38:20.9655709Z ##[section]Finishing: Run build
2020-04-15T22:38:20.9699164Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71174/merge to s
2020-04-15T22:38:20.9703694Z Task         : Get sources
2020-04-15T22:38:20.9703975Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-15T22:38:20.9704250Z Version      : 1.0.0
2020-04-15T22:38:20.9704438Z Author       : Microsoft
2020-04-15T22:38:20.9704438Z Author       : Microsoft
2020-04-15T22:38:20.9704736Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-15T22:38:20.9705083Z ==============================================================================
2020-04-15T22:38:21.2872975Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-15T22:38:21.2917789Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71174/merge to s
2020-04-15T22:38:21.3025389Z Cleaning up task key
2020-04-15T22:38:21.3026567Z Start cleaning up orphan processes.
2020-04-15T22:38:21.3192994Z Terminate orphan process: pid (3421) (python)
2020-04-15T22:38:21.3416348Z ##[section]Finishing: Finalize Job
