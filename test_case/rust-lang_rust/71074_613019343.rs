plain
2020-04-13T17:05:54.2433408Z ========================== Starting Command Output ===========================
2020-04-13T17:05:54.2436269Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/fb5d6635-bbd1-4ba6-ad4b-7e5717e9facd.sh
2020-04-13T17:05:54.2436459Z 
2020-04-13T17:05:54.2439899Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-13T17:05:54.2454063Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71074/merge to s
2020-04-13T17:05:54.2456510Z Task         : Get sources
2020-04-13T17:05:54.2456716Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-13T17:05:54.2456952Z Version      : 1.0.0
2020-04-13T17:05:54.2457091Z Author       : Microsoft
---
2020-04-13T17:05:55.5849489Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-13T17:05:55.5855227Z ##[command]git config gc.auto 0
2020-04-13T17:05:55.5860468Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-13T17:05:55.5863556Z ##[command]git config --get-all http.proxy
2020-04-13T17:05:55.5869616Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71074/merge:refs/remotes/pull/71074/merge
---
2020-04-13T17:09:32.6991709Z Looks like docker image is the same as before, not uploading
2020-04-13T17:09:35.8171786Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-13T17:09:35.8432132Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-13T17:09:35.8459245Z == clock drift check ==
2020-04-13T17:09:35.8480772Z   local time: Mon Apr 13 17:09:35 UTC 2020
2020-04-13T17:09:36.1182915Z   network time: Mon, 13 Apr 2020 17:09:36 GMT
2020-04-13T17:09:36.1209602Z Starting sccache server...
2020-04-13T17:09:36.1903708Z configure: processing command line
2020-04-13T17:09:36.1903955Z configure: 
2020-04-13T17:09:36.1908676Z configure: rust.dist-src        := False
---
2020-04-13T17:14:02.2045550Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-13T17:14:03.4398994Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-13T17:14:04.7933380Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-13T17:14:06.5416891Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-13T17:14:13.6647127Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-13T17:14:16.3255594Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-13T17:14:20.1828684Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-13T17:14:23.8114342Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-13T17:14:31.3900189Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-13T17:33:45.3329942Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-13T17:33:46.8617086Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-13T17:33:48.5600952Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-13T17:33:51.1968076Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-13T17:33:59.2286043Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-13T17:34:03.1276616Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-13T17:34:07.8279436Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-13T17:34:12.4790706Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-13T17:34:20.4688833Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-13T17:56:02.9998727Z .................................................................................................... 1700/9891
2020-04-13T17:56:06.5348969Z .................................................................................................... 1800/9891
2020-04-13T17:56:13.6776746Z .................................................................................................... 1900/9891
2020-04-13T17:56:20.6377787Z ....i............................................................................................... 2000/9891
2020-04-13T17:56:25.9201877Z ..............................................................................................iiiii. 2100/9891
2020-04-13T17:56:43.4047985Z .................................................................................................... 2300/9891
2020-04-13T17:56:45.1182681Z .................................................................................................... 2400/9891
2020-04-13T17:56:46.9190439Z .................................................................................................... 2500/9891
2020-04-13T17:56:51.5753328Z .................................................................................................... 2600/9891
---
2020-04-13T17:59:21.5131786Z .................................................................................................... 5100/9891
2020-04-13T17:59:27.9097117Z .................................................................................................... 5200/9891
2020-04-13T17:59:32.1305737Z ..............i..................................................................................... 5300/9891
2020-04-13T17:59:40.2249383Z .................................................................................................... 5400/9891
2020-04-13T17:59:44.5864492Z ....ii.ii........i...i.............................................................................. 5500/9891
2020-04-13T17:59:50.8965920Z ..................................................i................................................. 5700/9891
2020-04-13T17:59:59.3547799Z ......................................................................ii............................ 5800/9891
2020-04-13T18:00:04.7715546Z .........i.......................................................................................... 5900/9891
2020-04-13T18:00:09.3837229Z .................................................................................................... 6000/9891
2020-04-13T18:00:09.3837229Z .................................................................................................... 6000/9891
2020-04-13T18:00:17.8444942Z .................................................................................................... 6100/9891
2020-04-13T18:00:27.3413092Z ...ii...i..ii............i.......................................................................... 6200/9891
2020-04-13T18:00:39.6704079Z .................................................................................................... 6400/9891
2020-04-13T18:00:44.6678348Z .................................................................................................... 6500/9891
2020-04-13T18:00:44.6678348Z .................................................................................................... 6500/9891
2020-04-13T18:00:59.1773591Z .................................i..ii.............................................................. 6600/9891
2020-04-13T18:01:16.8740512Z .................................................................................................... 6800/9891
2020-04-13T18:01:18.4961456Z .................................i.................................................................. 6900/9891
2020-04-13T18:01:20.5220097Z .................................................................................................... 7000/9891
2020-04-13T18:01:22.2607070Z ........................................................................i........................... 7100/9891
---
2020-04-13T18:02:44.8437871Z .................................................................................................... 7800/9891
2020-04-13T18:02:48.2824892Z .................................................................................................... 7900/9891
2020-04-13T18:02:53.6664895Z ............................................................F....................................... 8000/9891
2020-04-13T18:02:59.1164839Z ......................................i............................................................. 8100/9891
2020-04-13T18:03:06.6835380Z ......................................................................................iiiiii.iiiii.i 8200/9891
2020-04-13T18:03:19.7020025Z ................................i......i............................................................ 8400/9891
2020-04-13T18:03:22.4837661Z .................................................................................................... 8500/9891
2020-04-13T18:03:31.2568950Z .................................................................................................... 8600/9891
2020-04-13T18:03:42.1567260Z .................................................................................................... 8700/9891
---
2020-04-13T18:05:22.6743193Z ---- [ui] ui/pattern/usefulness/match-privately-empty.rs stdout ----
2020-04-13T18:05:22.6743537Z 
2020-04-13T18:05:22.6743816Z error: ui test compiled successfully!
2020-04-13T18:05:22.6744089Z status: exit code: 0
2020-04-13T18:05:22.6745744Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/usefulness/match-privately-empty.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/usefulness/match-privately-empty" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/usefulness/match-privately-empty/auxiliary"
2020-04-13T18:05:22.6747476Z ------------------------------------------
2020-04-13T18:05:22.6747744Z 
2020-04-13T18:05:22.6748170Z ------------------------------------------
2020-04-13T18:05:22.6748475Z stderr:
---
2020-04-13T18:05:22.6750644Z ---- [ui] ui/rfc-2008-non-exhaustive/uninhabited/patterns.rs stdout ----
2020-04-13T18:05:22.6750962Z 
2020-04-13T18:05:22.6751405Z error: test compilation failed although it shouldn't!
2020-04-13T18:05:22.6751722Z status: exit code: 1
2020-04-13T18:05:22.6753327Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2008-non-exhaustive/uninhabited/patterns.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2008-non-exhaustive/uninhabited/patterns" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2008-non-exhaustive/uninhabited/patterns/auxiliary"
2020-04-13T18:05:22.6754827Z ------------------------------------------
2020-04-13T18:05:22.6755087Z 
2020-04-13T18:05:22.6755506Z ------------------------------------------
2020-04-13T18:05:22.6755792Z stderr:
2020-04-13T18:05:22.6755792Z stderr:
2020-04-13T18:05:22.6757123Z ------------------------------------------
2020-04-13T18:05:22.6757953Z error: unreachable pattern
2020-04-13T18:05:22.6758708Z   --> /checkout/src/test/ui/rfc-2008-non-exhaustive/uninhabited/patterns.rs:51:15
2020-04-13T18:05:22.6759231Z    |
2020-04-13T18:05:22.6759583Z LL |     while let PartiallyInhabitedVariants::Struct { x, .. } = partially_inhabited_variant() {
2020-04-13T18:05:22.6760285Z    |
2020-04-13T18:05:22.6760523Z note: the lint level is defined here
2020-04-13T18:05:22.6761105Z   --> /checkout/src/test/ui/rfc-2008-non-exhaustive/uninhabited/patterns.rs:3:9
2020-04-13T18:05:22.6761455Z    |
---
2020-04-13T18:05:22.6769655Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-13T18:05:22.6770211Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-13T18:05:22.6784008Z 
2020-04-13T18:05:22.6784545Z 
2020-04-13T18:05:22.6795996Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-13T18:05:22.6799065Z 
2020-04-13T18:05:22.6799168Z 
2020-04-13T18:05:22.6799754Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-13T18:05:22.6800311Z Build completed unsuccessfully in 0:54:26
2020-04-13T18:05:22.6800311Z Build completed unsuccessfully in 0:54:26
2020-04-13T18:05:22.6847656Z == clock drift check ==
2020-04-13T18:05:22.6862397Z   local time: Mon Apr 13 18:05:22 UTC 2020
2020-04-13T18:05:23.2347842Z   network time: Mon, 13 Apr 2020 18:05:23 GMT
2020-04-13T18:05:23.9580776Z 
2020-04-13T18:05:23.9580776Z 
2020-04-13T18:05:23.9653671Z ##[error]Bash exited with code '1'.
2020-04-13T18:05:23.9665534Z ##[section]Finishing: Run build
2020-04-13T18:05:23.9701824Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71074/merge to s
2020-04-13T18:05:23.9706012Z Task         : Get sources
2020-04-13T18:05:23.9706251Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-13T18:05:23.9706499Z Version      : 1.0.0
2020-04-13T18:05:23.9706655Z Author       : Microsoft
2020-04-13T18:05:23.9706655Z Author       : Microsoft
2020-04-13T18:05:23.9706901Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-13T18:05:23.9707200Z ==============================================================================
2020-04-13T18:05:24.2497432Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-13T18:05:24.2538784Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71074/merge to s
2020-04-13T18:05:24.2614980Z Cleaning up task key
2020-04-13T18:05:24.2615999Z Start cleaning up orphan processes.
2020-04-13T18:05:24.2881517Z Terminate orphan process: pid (3746) (python)
2020-04-13T18:05:24.2928266Z ##[section]Finishing: Finalize Job
