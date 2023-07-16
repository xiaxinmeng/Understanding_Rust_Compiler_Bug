plain
2020-04-13T01:34:23.9404747Z ========================== Starting Command Output ===========================
2020-04-13T01:34:23.9407700Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/789bc8bf-54cb-43ce-a872-188f02600e80.sh
2020-04-13T01:34:23.9408105Z 
2020-04-13T01:34:23.9412416Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-13T01:34:23.9435260Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71074/merge to s
2020-04-13T01:34:23.9438715Z Task         : Get sources
2020-04-13T01:34:23.9439028Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-13T01:34:23.9439342Z Version      : 1.0.0
2020-04-13T01:34:23.9439549Z Author       : Microsoft
---
2020-04-13T01:34:25.1890733Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-13T01:34:25.1900208Z ##[command]git config gc.auto 0
2020-04-13T01:34:25.1905119Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-13T01:34:25.1909894Z ##[command]git config --get-all http.proxy
2020-04-13T01:34:25.1919743Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71074/merge:refs/remotes/pull/71074/merge
---
2020-04-13T01:37:59.7669828Z Looks like docker image is the same as before, not uploading
2020-04-13T01:38:05.4204746Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-13T01:38:05.4515168Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-13T01:38:05.4547632Z == clock drift check ==
2020-04-13T01:38:05.4557358Z   local time: Mon Apr 13 01:38:05 UTC 2020
2020-04-13T01:38:05.5226629Z   network time: Mon, 13 Apr 2020 01:38:05 GMT
2020-04-13T01:38:05.5251835Z Starting sccache server...
2020-04-13T01:38:05.6108194Z configure: processing command line
2020-04-13T01:38:05.6109504Z configure: 
2020-04-13T01:38:05.6110353Z configure: rust.dist-src        := False
---
2020-04-13T01:43:05.8014270Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-13T01:43:07.2023288Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-13T01:43:08.7279820Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-13T01:43:09.7862679Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-13T01:43:18.4441049Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-13T01:43:20.6864284Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-13T01:43:24.9069347Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-13T01:43:28.8292321Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-13T01:43:38.3385682Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-13T02:05:21.1458611Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-13T02:05:22.8496327Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-13T02:05:24.7869271Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-13T02:05:25.6765167Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-13T02:05:36.9157458Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-13T02:05:38.9771870Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-13T02:05:44.1404049Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-13T02:05:49.3657516Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-13T02:06:01.2455186Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-13T02:30:59.7051459Z .................................................................................................... 1700/9891
2020-04-13T02:31:03.9381274Z .................................................................................................... 1800/9891
2020-04-13T02:31:12.4267596Z .................................................................................................... 1900/9891
2020-04-13T02:31:20.6416530Z ....i............................................................................................... 2000/9891
2020-04-13T02:31:27.0051308Z ..............................................................................................iiiii. 2100/9891
2020-04-13T02:31:47.6436193Z .................................................................................................... 2300/9891
2020-04-13T02:31:49.7954820Z .................................................................................................... 2400/9891
2020-04-13T02:31:52.0737117Z .................................................................................................... 2500/9891
2020-04-13T02:31:57.8221360Z .................................................................................................... 2600/9891
---
2020-04-13T02:35:00.1696599Z .................................................................................................... 5100/9891
2020-04-13T02:35:07.5939414Z .................................................................................................... 5200/9891
2020-04-13T02:35:12.6209696Z ..............i..................................................................................... 5300/9891
2020-04-13T02:35:22.1276051Z .................................................................................................... 5400/9891
2020-04-13T02:35:27.3072290Z ....ii.ii........i...i.............................................................................. 5500/9891
2020-04-13T02:35:34.9131997Z ..................................................i................................................. 5700/9891
2020-04-13T02:35:44.7367631Z ......................................................................ii............................ 5800/9891
2020-04-13T02:35:51.1963482Z .........i.......................................................................................... 5900/9891
2020-04-13T02:35:56.6072052Z .................................................................................................... 6000/9891
2020-04-13T02:35:56.6072052Z .................................................................................................... 6000/9891
2020-04-13T02:36:06.7670523Z .................................................................................................... 6100/9891
2020-04-13T02:36:17.5404544Z ...ii...i..ii...........i........................................................................... 6200/9891
2020-04-13T02:36:33.0398443Z .................................................................................................... 6400/9891
2020-04-13T02:36:38.6139233Z .................................................................................................... 6500/9891
2020-04-13T02:36:38.6139233Z .................................................................................................... 6500/9891
2020-04-13T02:36:50.4561828Z .................................i..ii.............................................................. 6600/9891
2020-04-13T02:37:11.2703110Z .................................................................................................... 6800/9891
2020-04-13T02:37:13.2581884Z .................................i.................................................................. 6900/9891
2020-04-13T02:37:15.3261672Z .................................................................................................... 7000/9891
2020-04-13T02:37:17.4629299Z ........................................................................i........................... 7100/9891
---
2020-04-13T02:38:52.9442082Z .................................................................................................... 7800/9891
2020-04-13T02:38:56.8537795Z .................................................................................................... 7900/9891
2020-04-13T02:39:03.7530157Z .................................................................................................... 8000/9891
2020-04-13T02:39:10.0325454Z ......................................i............................................................. 8100/9891
2020-04-13T02:39:19.4221237Z ......................................................................................iiiiii.iiiii.i 8200/9891
2020-04-13T02:39:36.0707843Z ................................i......i............................................................ 8400/9891
2020-04-13T02:39:39.9152588Z .................................................................................................... 8500/9891
2020-04-13T02:39:50.7579958Z .................................................................................................... 8600/9891
2020-04-13T02:40:04.4126841Z .................................................................................................... 8700/9891
---
2020-04-13T02:42:00.4165055Z ---- [ui] ui/issues/issue-46964.rs stdout ----
2020-04-13T02:42:00.4165293Z 
2020-04-13T02:42:00.4165742Z error: test compilation failed although it shouldn't!
2020-04-13T02:42:00.4166015Z status: exit code: 101
2020-04-13T02:42:00.4167976Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-46964.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46964" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46964/auxiliary"
2020-04-13T02:42:00.4169596Z ------------------------------------------
2020-04-13T02:42:00.4169890Z 
2020-04-13T02:42:00.4170236Z ------------------------------------------
2020-04-13T02:42:00.4170426Z stderr:
2020-04-13T02:42:00.4170426Z stderr:
2020-04-13T02:42:00.4170796Z ------------------------------------------
2020-04-13T02:42:00.4171490Z thread 'rustc' panicked at 'assertion failed: rows.iter().all(|r| r.len() == v.len())', src/librustc_mir_build/hair/pattern/_match.rs:1652:5
2020-04-13T02:42:00.4172235Z 
2020-04-13T02:42:00.4172437Z error: internal compiler error: unexpected panic
2020-04-13T02:42:00.4172621Z 
2020-04-13T02:42:00.4172827Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-13T02:42:00.4172827Z note: the compiler unexpectedly panicked. this is a bug.
2020-04-13T02:42:00.4173027Z 
2020-04-13T02:42:00.4173785Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-04-13T02:42:00.4174534Z note: rustc 1.44.0-nightly (8a3c83803 2020-04-13) running on x86_64-unknown-linux-gnu
2020-04-13T02:42:00.4174769Z 
2020-04-13T02:42:00.4174769Z 
2020-04-13T02:42:00.4175537Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-04-13T02:42:00.4175992Z 
2020-04-13T02:42:00.4176363Z ------------------------------------------
2020-04-13T02:42:00.4176536Z 
2020-04-13T02:42:00.4176634Z 
---
2020-04-13T02:42:00.4193393Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-13T02:42:00.4193848Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-13T02:42:00.4211043Z 
2020-04-13T02:42:00.4211254Z 
2020-04-13T02:42:00.4215809Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-13T02:42:00.4218666Z 
2020-04-13T02:42:00.4218774Z 
2020-04-13T02:42:00.4222954Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-13T02:42:00.4223359Z Build completed unsuccessfully in 1:02:22
2020-04-13T02:42:00.4223359Z Build completed unsuccessfully in 1:02:22
2020-04-13T02:42:00.4275866Z == clock drift check ==
2020-04-13T02:42:00.4295573Z   local time: Mon Apr 13 02:42:00 UTC 2020
2020-04-13T02:42:00.5003404Z   network time: Mon, 13 Apr 2020 02:42:00 GMT
2020-04-13T02:42:01.1993011Z 
2020-04-13T02:42:01.1993011Z 
2020-04-13T02:42:01.2077507Z ##[error]Bash exited with code '1'.
2020-04-13T02:42:01.2091871Z ##[section]Finishing: Run build
2020-04-13T02:42:01.2144262Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71074/merge to s
2020-04-13T02:42:01.2149479Z Task         : Get sources
2020-04-13T02:42:01.2149832Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-13T02:42:01.2150170Z Version      : 1.0.0
2020-04-13T02:42:01.2150410Z Author       : Microsoft
2020-04-13T02:42:01.2150410Z Author       : Microsoft
2020-04-13T02:42:01.2150766Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-13T02:42:01.2151194Z ==============================================================================
2020-04-13T02:42:01.5402837Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-13T02:42:01.5467997Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71074/merge to s
2020-04-13T02:42:01.5553562Z Cleaning up task key
2020-04-13T02:42:01.5554821Z Start cleaning up orphan processes.
2020-04-13T02:42:01.5732926Z Terminate orphan process: pid (3673) (python)
2020-04-13T02:42:01.5898923Z ##[section]Finishing: Finalize Job
