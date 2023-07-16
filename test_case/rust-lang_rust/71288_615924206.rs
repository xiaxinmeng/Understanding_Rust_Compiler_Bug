plain
2020-04-18T17:43:16.9573217Z ========================== Starting Command Output ===========================
2020-04-18T17:43:16.9575186Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/3c59b34f-0fb1-4a0f-8c67-163c32790bd3.sh
2020-04-18T17:43:16.9575366Z 
2020-04-18T17:43:16.9579127Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-18T17:43:16.9598119Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71288/merge to s
2020-04-18T17:43:16.9602118Z Task         : Get sources
2020-04-18T17:43:16.9602336Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-18T17:43:16.9602549Z Version      : 1.0.0
2020-04-18T17:43:16.9602739Z Author       : Microsoft
---
2020-04-18T17:43:18.2216267Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-18T17:43:18.2223560Z ##[command]git config gc.auto 0
2020-04-18T17:43:18.2229513Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-18T17:43:18.2234650Z ##[command]git config --get-all http.proxy
2020-04-18T17:43:18.2245119Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71288/merge:refs/remotes/pull/71288/merge
---
2020-04-18T17:45:21.4662255Z  ---> 318032b5f0e2
2020-04-18T17:45:21.4663670Z Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
2020-04-18T17:45:21.4667013Z  ---> Using cache
2020-04-18T17:45:21.4667519Z  ---> d44a858fd1ce
2020-04-18T17:45:21.4668980Z Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
2020-04-18T17:45:21.4671316Z  ---> 58b910f50f5a
2020-04-18T17:45:21.4671468Z Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
2020-04-18T17:45:21.4674509Z  ---> Using cache
2020-04-18T17:45:21.4674794Z  ---> ee7702aadba1
---
2020-04-18T17:45:21.5020709Z Looks like docker image is the same as before, not uploading
2020-04-18T17:45:29.2952176Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-18T17:45:29.3235679Z [CI_JOB_NAME=x86_64-gnu-llvm-8]
2020-04-18T17:45:29.3264450Z == clock drift check ==
2020-04-18T17:45:29.3271716Z   local time: Sat Apr 18 17:45:29 UTC 2020
2020-04-18T17:45:29.4047118Z   network time: Sat, 18 Apr 2020 17:45:29 GMT
2020-04-18T17:45:29.4071606Z Starting sccache server...
2020-04-18T17:45:29.4807510Z configure: processing command line
2020-04-18T17:45:29.4808073Z configure: 
2020-04-18T17:45:29.4809012Z configure: rust.dist-src        := False
---
2020-04-18T17:50:18.2560601Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-18T17:50:19.6538212Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-18T17:50:21.1133967Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-18T17:50:22.9051344Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-18T17:50:30.4267587Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-18T17:50:33.8473383Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-18T17:50:38.0268575Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-18T17:50:42.0295393Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-18T17:50:49.8682598Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-18T18:11:30.2312804Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-18T18:11:31.6928437Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-18T18:11:33.3379302Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-18T18:11:34.2889461Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-18T18:11:43.2610959Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-18T18:11:46.1636071Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-18T18:11:50.4126520Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-18T18:11:54.5534328Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-18T18:12:03.6122227Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-18T18:32:54.1359959Z .................................................................................................... 1700/9906
2020-04-18T18:32:57.7421683Z .................................................................................................... 1800/9906
2020-04-18T18:33:05.4283800Z .................................................................................................i.. 1900/9906
2020-04-18T18:33:12.6659070Z .................................................................................................... 2000/9906
2020-04-18T18:33:18.5090779Z .......................................................................................iiiii........ 2100/9906
2020-04-18T18:33:36.0050128Z .................................................................................................... 2300/9906
2020-04-18T18:33:37.9955929Z .................................................................................................... 2400/9906
2020-04-18T18:33:40.0805159Z .................................................................................................... 2500/9906
2020-04-18T18:33:45.4072613Z .................................................................................................... 2600/9906
---
2020-04-18T18:36:12.5516222Z .................................................................................................... 4900/9906
2020-04-18T18:36:17.0396521Z ...............................................................i...............i.................... 5000/9906
2020-04-18T18:36:23.5853676Z .................................................................................................... 5100/9906
2020-04-18T18:36:29.7422666Z .................................................................................................... 5200/9906
2020-04-18T18:36:34.2696549Z .........i.........................................................................................i 5300/9906
2020-04-18T18:36:42.8577397Z ...................................................................................................i 5400/9906
2020-04-18T18:36:47.0911421Z i.ii........i...i................................................................................... 5500/9906
2020-04-18T18:36:53.9520424Z ..............................................i..................................................... 5700/9906
2020-04-18T18:37:01.8899006Z ..............................................................................ii.................... 5800/9906
2020-04-18T18:37:08.0869220Z .................i.................................................................................. 5900/9906
2020-04-18T18:37:12.8981134Z .................................................................................................... 6000/9906
2020-04-18T18:37:12.8981134Z .................................................................................................... 6000/9906
2020-04-18T18:37:22.8236420Z .................................................................................................... 6100/9906
2020-04-18T18:37:32.5411918Z ...........ii...i..ii...........i................................................................... 6200/9906
2020-04-18T18:37:46.8793583Z .................................................................................................... 6400/9906
2020-04-18T18:37:52.6778908Z .................................................................................................... 6500/9906
2020-04-18T18:37:52.6778908Z .................................................................................................... 6500/9906
2020-04-18T18:38:09.0487255Z .........................................i..ii...................................................... 6600/9906
2020-04-18T18:38:28.3789567Z .................................................................................................... 6800/9906
2020-04-18T18:38:30.2414915Z ..........................................i......................................................... 6900/9906
2020-04-18T18:38:32.1007425Z .................................................................................................... 7000/9906
2020-04-18T18:38:33.9443297Z ..................................................................................i.F............... 7100/9906
---
2020-04-18T18:39:56.6638720Z .................................................................................................... 7800/9906
2020-04-18T18:40:00.5566380Z .................................................................................................... 7900/9906
2020-04-18T18:40:06.5203127Z .................................................................................................... 8000/9906
2020-04-18T18:40:11.7224632Z ................................................i................................................... 8100/9906
2020-04-18T18:40:20.5733538Z .................................................................................................iii 8200/9906
2020-04-18T18:40:25.4748943Z iii.iiiii.i......................................................................................... 8300/9906
2020-04-18T18:40:37.2210373Z .................................................................................................... 8500/9906
2020-04-18T18:40:44.4638799Z .................................................................................................... 8600/9906
2020-04-18T18:40:56.7619527Z .................................................................................................... 8700/9906
2020-04-18T18:41:02.6634747Z .................................................................................................... 8800/9906
---
2020-04-18T18:42:38.7287669Z failures:
2020-04-18T18:42:38.7337770Z 
2020-04-18T18:42:38.7338786Z ---- [ui] ui/parser/mod_file_not_exist.rs stdout ----
2020-04-18T18:42:38.7339337Z 
2020-04-18T18:42:38.7340246Z error: /checkout/src/test/ui/parser/mod_file_not_exist.rs:3: unexpected help message: '3:1: 3:21: if there is `mod not_a_real_file` elsewhere in the crate already, import it with `use crate::` instead'
2020-04-18T18:42:38.7341226Z error: 1 unexpected errors found, 0 expected errors not found
2020-04-18T18:42:38.7341601Z status: exit code: 1
2020-04-18T18:42:38.7341601Z status: exit code: 1
2020-04-18T18:42:38.7343466Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/mod_file_not_exist.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/mod_file_not_exist" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/mod_file_not_exist/auxiliary"
2020-04-18T18:42:38.7344907Z     Error {
2020-04-18T18:42:38.7345187Z         line_num: 3,
2020-04-18T18:42:38.7345492Z         kind: Some(
2020-04-18T18:42:38.7345769Z             Help,
2020-04-18T18:42:38.7345769Z             Help,
2020-04-18T18:42:38.7346041Z         ),
2020-04-18T18:42:38.7346535Z         msg: "3:1: 3:21: if there is `mod not_a_real_file` elsewhere in the crate already, import it with `use crate::` instead",
2020-04-18T18:42:38.7347489Z ]
2020-04-18T18:42:38.7347799Z 
2020-04-18T18:42:38.7348560Z thread '[ui] ui/parser/mod_file_not_exist.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1460:13
2020-04-18T18:42:38.7349300Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
---
2020-04-18T18:42:38.7351719Z 
2020-04-18T18:42:38.7365730Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-18T18:42:38.7383916Z 
2020-04-18T18:42:38.7385104Z 
2020-04-18T18:42:38.7389556Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-18T18:42:38.7393858Z 
2020-04-18T18:42:38.7394048Z 
2020-04-18T18:42:38.7394803Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-18T18:42:38.7395368Z Build completed unsuccessfully in 0:55:42
2020-04-18T18:42:38.7395368Z Build completed unsuccessfully in 0:55:42
2020-04-18T18:42:38.7439480Z == clock drift check ==
2020-04-18T18:42:38.7455821Z   local time: Sat Apr 18 18:42:38 UTC 2020
2020-04-18T18:42:38.8578061Z   network time: Sat, 18 Apr 2020 18:42:38 GMT
2020-04-18T18:42:39.2616512Z 
2020-04-18T18:42:39.2616512Z 
2020-04-18T18:42:39.2669042Z ##[error]Bash exited with code '1'.
2020-04-18T18:42:39.2680095Z ##[section]Finishing: Run build
2020-04-18T18:42:39.2722685Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71288/merge to s
2020-04-18T18:42:39.2727945Z Task         : Get sources
2020-04-18T18:42:39.2728186Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-18T18:42:39.2728420Z Version      : 1.0.0
2020-04-18T18:42:39.2728575Z Author       : Microsoft
2020-04-18T18:42:39.2728575Z Author       : Microsoft
2020-04-18T18:42:39.2728825Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-18T18:42:39.2729122Z ==============================================================================
2020-04-18T18:42:39.5696642Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-18T18:42:39.5748058Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71288/merge to s
2020-04-18T18:42:39.5828081Z Cleaning up task key
2020-04-18T18:42:39.5829100Z Start cleaning up orphan processes.
2020-04-18T18:42:39.6212952Z Terminate orphan process: pid (4064) (python)
2020-04-18T18:42:39.6253104Z ##[section]Finishing: Finalize Job
