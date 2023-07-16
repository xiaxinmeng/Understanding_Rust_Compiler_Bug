plain
2020-04-01T18:16:52.1693803Z ========================== Starting Command Output ===========================
2020-04-01T18:16:52.1696857Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d1cb5b58-8730-43c6-b0f0-c1c2d8deb718.sh
2020-04-01T18:16:52.1697071Z 
2020-04-01T18:16:52.1700243Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-01T18:16:52.1726628Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70566/merge to s
2020-04-01T18:16:52.1732024Z Task         : Get sources
2020-04-01T18:16:52.1732471Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-01T18:16:52.1732909Z Version      : 1.0.0
2020-04-01T18:16:52.1733406Z Author       : Microsoft
---
2020-04-01T18:16:53.1807537Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-01T18:16:53.1812561Z ##[command]git config gc.auto 0
2020-04-01T18:16:53.1815518Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-01T18:16:53.1819518Z ##[command]git config --get-all http.proxy
2020-04-01T18:16:53.1824787Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70566/merge:refs/remotes/pull/70566/merge
---
2020-04-01T18:19:15.5862915Z Looks like docker image is the same as before, not uploading
2020-04-01T18:19:23.5741918Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-01T18:19:23.6109795Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-01T18:19:23.6141201Z == clock drift check ==
2020-04-01T18:19:23.6160287Z   local time: Wed Apr  1 18:19:23 UTC 2020
2020-04-01T18:19:23.9066162Z   network time: Wed, 01 Apr 2020 18:19:23 GMT
2020-04-01T18:19:23.9082091Z Starting sccache server...
2020-04-01T18:19:23.9903492Z configure: processing command line
2020-04-01T18:19:23.9903768Z configure: 
2020-04-01T18:19:23.9904548Z configure: rust.dist-src        := False
---
2020-04-01T18:24:37.6027655Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-01T18:24:39.1261051Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-01T18:24:40.7600510Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-01T18:24:42.3818250Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-01T18:24:51.3535131Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-01T18:24:54.4915832Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-01T18:24:59.2035813Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-01T18:25:03.6289935Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-01T18:25:12.4716618Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-01T18:47:54.3676839Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-01T18:47:56.1524104Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-01T18:47:58.1013342Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-01T18:48:00.3341362Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-01T18:48:10.2506906Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-01T18:48:14.1919685Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-01T18:48:19.5081436Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-01T18:48:24.9596997Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-01T18:48:34.9300448Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-01T19:14:01.5832236Z .................................................................................................... 1700/9869
2020-04-01T19:14:05.6140429Z .................................................................................................... 1800/9869
2020-04-01T19:14:14.5102989Z .................................................................................................i.. 1900/9869
2020-04-01T19:14:22.4386774Z .................................................................................................... 2000/9869
2020-04-01T19:14:28.7736849Z .......................................................................................iiiii........ 2100/9869
2020-04-01T19:14:49.5671678Z .................................................................................................... 2300/9869
2020-04-01T19:14:51.7501370Z .................................................................................................... 2400/9869
2020-04-01T19:14:54.0235360Z .................................................................................................... 2500/9869
2020-04-01T19:14:59.9549755Z .................................................................................................... 2600/9869
---
2020-04-01T19:17:48.9148465Z .............................................................i...............i...................... 5000/9869
2020-04-01T19:17:56.2354274Z .................................................................................................... 5100/9869
2020-04-01T19:18:03.8888459Z .................................................................................................... 5200/9869
2020-04-01T19:18:08.9464934Z ......i............................................................................................. 5300/9869
2020-04-01T19:18:18.8339092Z ............................................................................................ii.ii... 5400/9869
2020-04-01T19:18:23.2747889Z .....i...i.......................................................................................... 5500/9869
2020-04-01T19:18:26.7907705Z ................................i.....................................FF...F........................ 5600/9869
2020-04-01T19:18:41.4372221Z .........................................................ii....................................i.... 5800/9869
2020-04-01T19:18:48.8119893Z .................................................................................................... 5900/9869
2020-04-01T19:18:53.5511808Z .................................................................................................... 6000/9869
2020-04-01T19:18:53.5511808Z .................................................................................................... 6000/9869
2020-04-01T19:19:03.2492060Z .........................................................................................ii...i..ii. 6100/9869
2020-04-01T19:19:22.3816350Z .................................................................................................... 6300/9869
2020-04-01T19:19:29.9767971Z .................................................................................................... 6400/9869
2020-04-01T19:19:35.7974620Z .................................................................................................... 6500/9869
2020-04-01T19:19:35.7974620Z .................................................................................................... 6500/9869
2020-04-01T19:19:52.9383540Z ...................i..ii............................................................................ 6600/9869
2020-04-01T19:20:13.6373602Z .................................................................................................... 6800/9869
2020-04-01T19:20:15.6991999Z ...................i................................................................................ 6900/9869
2020-04-01T19:20:17.7148950Z .................................................................................................... 7000/9869
2020-04-01T19:20:19.9438058Z ..........................................................i......................................... 7100/9869
---
2020-04-01T19:22:00.5416484Z .................................................................................................... 7800/9869
2020-04-01T19:22:05.2955107Z .................................................................................................... 7900/9869
2020-04-01T19:22:11.3971215Z .................................................................................................... 8000/9869
2020-04-01T19:22:19.7570658Z ....................i............................................................................... 8100/9869
2020-04-01T19:22:28.0864167Z .....................................................................iiiiiiiiii.i................... 8200/9869
2020-04-01T19:22:44.0964432Z .............i......i............................................................................... 8400/9869
2020-04-01T19:22:48.9133672Z .................................................................................................... 8500/9869
2020-04-01T19:23:00.3162841Z .................................................................................................... 8600/9869
2020-04-01T19:23:11.8613923Z .................................................................................................... 8700/9869
---
2020-04-01T19:25:06.5314865Z failures:
2020-04-01T19:25:06.5340051Z 
2020-04-01T19:25:06.5341351Z ---- [ui] ui/lint/lint-exceeding-bitshifts.rs#noopt stdout ----
2020-04-01T19:25:06.5341780Z 
2020-04-01T19:25:06.5342418Z error in revision `noopt`: /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:18: expected error not found: arithmetic operation will overflow
2020-04-01T19:25:06.5342756Z 
2020-04-01T19:25:06.5342976Z error in revision `noopt`: 0 unexpected errors found, 1 expected errors not found
2020-04-01T19:25:06.5343243Z status: exit code: 1
2020-04-01T19:25:06.5344912Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "noopt" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts.noopt" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-C" "opt-level=0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts.noopt/auxiliary"
2020-04-01T19:25:06.5346496Z     Error {
2020-04-01T19:25:06.5346657Z         line_num: 18,
2020-04-01T19:25:06.5346815Z         kind: Some(
2020-04-01T19:25:06.5346961Z             Error,
---
2020-04-01T19:25:06.5352152Z error in revision `opt`: /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:18: expected error not found: arithmetic operation will overflow
2020-04-01T19:25:06.5352489Z 
2020-04-01T19:25:06.5352703Z error in revision `opt`: 0 unexpected errors found, 1 expected errors not found
2020-04-01T19:25:06.5352949Z status: exit code: 1
2020-04-01T19:25:06.5354586Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "opt" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts.opt" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts.opt/auxiliary"
2020-04-01T19:25:06.5361431Z     Error {
2020-04-01T19:25:06.5361756Z         line_num: 18,
2020-04-01T19:25:06.5361932Z         kind: Some(
2020-04-01T19:25:06.5362089Z             Error,
---
2020-04-01T19:25:06.5364044Z ---- [ui] ui/lint/lint-exceeding-bitshifts.rs#opt_with_overflow_checks stdout ----
2020-04-01T19:25:06.5364234Z 
2020-04-01T19:25:06.5364841Z error in revision `opt_with_overflow_checks`: /checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs:18: expected error not found: arithmetic operation will overflow
2020-04-01T19:25:06.5365187Z 
2020-04-01T19:25:06.5365432Z error in revision `opt_with_overflow_checks`: 0 unexpected errors found, 1 expected errors not found
2020-04-01T19:25:06.5365726Z status: exit code: 1
2020-04-01T19:25:06.5368098Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-exceeding-bitshifts.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "opt_with_overflow_checks" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts.opt_with_overflow_checks" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-C" "overflow-checks=on" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-exceeding-bitshifts.opt_with_overflow_checks/auxiliary"
2020-04-01T19:25:06.5369575Z     Error {
2020-04-01T19:25:06.5369730Z         line_num: 18,
2020-04-01T19:25:06.5369901Z         kind: Some(
2020-04-01T19:25:06.5370079Z             Error,
---
2020-04-01T19:25:06.5378731Z 
2020-04-01T19:25:06.5379213Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-01T19:25:06.5379443Z 
2020-04-01T19:25:06.5379541Z 
2020-04-01T19:25:06.5383428Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-01T19:25:06.5388498Z 
2020-04-01T19:25:06.5388592Z 
2020-04-01T19:25:06.5389220Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-01T19:25:06.5389592Z Build completed unsuccessfully in 1:04:04
2020-04-01T19:25:06.5389592Z Build completed unsuccessfully in 1:04:04
2020-04-01T19:25:06.5446721Z == clock drift check ==
2020-04-01T19:25:06.5466378Z   local time: Wed Apr  1 19:25:06 UTC 2020
2020-04-01T19:25:06.7105904Z   network time: Wed, 01 Apr 2020 19:25:06 GMT
2020-04-01T19:25:07.1356427Z 
2020-04-01T19:25:07.1356427Z 
2020-04-01T19:25:07.1441580Z ##[error]Bash exited with code '1'.
2020-04-01T19:25:07.1454510Z ##[section]Finishing: Run build
2020-04-01T19:25:07.1499992Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70566/merge to s
2020-04-01T19:25:07.1504667Z Task         : Get sources
2020-04-01T19:25:07.1504997Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-01T19:25:07.1505292Z Version      : 1.0.0
2020-04-01T19:25:07.1505490Z Author       : Microsoft
2020-04-01T19:25:07.1505490Z Author       : Microsoft
2020-04-01T19:25:07.1505820Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-01T19:25:07.1506193Z ==============================================================================
2020-04-01T19:25:07.4787739Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-01T19:25:07.4833639Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70566/merge to s
2020-04-01T19:25:07.4935423Z Cleaning up task key
2020-04-01T19:25:07.4936603Z Start cleaning up orphan processes.
2020-04-01T19:25:07.5127416Z Terminate orphan process: pid (3926) (python)
2020-04-01T19:25:07.5310558Z ##[section]Finishing: Finalize Job
