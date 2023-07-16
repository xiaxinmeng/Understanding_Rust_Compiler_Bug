plain
2020-03-24T22:58:38.1320504Z ========================== Starting Command Output ===========================
2020-03-24T22:58:38.1322255Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/44d4995f-4352-4b51-902f-dd516ceaa03c.sh
2020-03-24T22:58:38.1322444Z 
2020-03-24T22:58:38.1325685Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-24T22:58:38.1339705Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70107/merge to s
2020-03-24T22:58:38.1342434Z Task         : Get sources
2020-03-24T22:58:38.1342704Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-24T22:58:38.1342952Z Version      : 1.0.0
2020-03-24T22:58:38.1343116Z Author       : Microsoft
---
2020-03-24T22:58:39.1329569Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-24T22:58:39.1334657Z ##[command]git config gc.auto 0
2020-03-24T22:58:39.1337901Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-24T22:58:39.1340940Z ##[command]git config --get-all http.proxy
2020-03-24T22:58:39.1346397Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70107/merge:refs/remotes/pull/70107/merge
---
2020-03-24T23:05:07.6698703Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-24T23:05:08.0801842Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-24T23:05:15.3197713Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-24T23:05:22.3518936Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-24T23:05:25.4233316Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-24T23:05:26.9850859Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-24T23:05:53.0274788Z    Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-03-24T23:05:59.3144258Z    Compiling rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-03-24T23:06:37.6150806Z    Compiling rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
---
2020-03-24T23:22:53.1246372Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-24T23:22:53.8291863Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-24T23:23:03.1177869Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-24T23:23:12.1450911Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-24T23:23:16.5627550Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-24T23:23:17.9969581Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-24T23:23:53.1762196Z    Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-03-24T23:24:01.1884120Z    Compiling rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-03-24T23:24:50.9778716Z    Compiling rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
---
2020-03-24T23:42:47.7752151Z .................................................................................................... 1700/9833
2020-03-24T23:42:51.0444522Z .................................................................................................... 1800/9833
2020-03-24T23:42:58.9744226Z ........................................................................................i........... 1900/9833
2020-03-24T23:43:04.5968410Z .................................................................................................... 2000/9833
2020-03-24T23:43:09.6703434Z ..............................................................................iiiii................. 2100/9833
2020-03-24T23:43:26.6102390Z .................................................................................................... 2300/9833
2020-03-24T23:43:28.4147853Z .................................................................................................... 2400/9833
2020-03-24T23:43:30.3933359Z .................................................................................................... 2500/9833
2020-03-24T23:43:38.3117541Z .................................................................................................... 2600/9833
---
2020-03-24T23:45:54.5197019Z .....................................................i...............i.............................. 5000/9833
2020-03-24T23:46:00.6225542Z .................................................................................................... 5100/9833
2020-03-24T23:46:06.3272243Z ..................................................................................................i. 5200/9833
2020-03-24T23:46:10.6042933Z .................................................................................................... 5300/9833
2020-03-24T23:46:18.9030236Z .................................................................................ii.ii........i...i. 5400/9833
2020-03-24T23:46:24.6863803Z .....................i.............................................................................. 5600/9833
2020-03-24T23:46:30.4905813Z ..........................i......................................................................... 5700/9833
2020-03-24T23:46:36.6010820Z ...........................................ii....................................i.................. 5800/9833
2020-03-24T23:46:42.4543396Z .................................................................................................... 5900/9833
2020-03-24T23:46:42.4543396Z .................................................................................................... 5900/9833
2020-03-24T23:46:46.7990351Z .................................................................................................... 6000/9833
2020-03-24T23:46:54.3120796Z ...........................................................................ii...i..ii...........i... 6100/9833
2020-03-24T23:47:10.4433494Z .................................................................................................... 6300/9833
2020-03-24T23:47:13.8381485Z .................................................................................................... 6400/9833
2020-03-24T23:47:16.6285742Z .................................................................................................... 6500/9833
2020-03-24T23:47:16.6285742Z .................................................................................................... 6500/9833
2020-03-24T23:47:26.1927233Z .....i..ii.......................................................................................... 6600/9833
2020-03-24T23:47:41.7620203Z .................................................................................................... 6800/9833
2020-03-24T23:47:43.4403404Z ....i............................................................................................... 6900/9833
2020-03-24T23:47:45.0624441Z .................................................................................................... 7000/9833
2020-03-24T23:47:46.9606869Z .......................................i............................................................ 7100/9833
---
2020-03-24T23:49:03.8581598Z .................................................................................................... 7800/9833
2020-03-24T23:49:07.5484788Z .................................................................................................... 7900/9833
2020-03-24T23:49:12.6311152Z ...............................................................................................i.... 8000/9833
2020-03-24T23:49:18.7000373Z .................................................................................................... 8100/9833
2020-03-24T23:49:24.4542583Z .............................................iiiiiiiiiii............................................ 8200/9833
2020-03-24T23:49:35.3292492Z .................................................................................................... 8400/9833
2020-03-24T23:49:39.2907628Z .................................................................................................... 8500/9833
2020-03-24T23:49:49.6818542Z .................................................................................................... 8600/9833
2020-03-24T23:49:56.7737565Z .................................................................................................... 8700/9833
---
2020-03-24T23:51:27.1657927Z ---- [ui] ui/consts/associated_const_generic.rs stdout ----
2020-03-24T23:51:27.1658276Z 
2020-03-24T23:51:27.1658718Z error: test compilation failed although it shouldn't!
2020-03-24T23:51:27.1659031Z status: exit code: 1
2020-03-24T23:51:27.1660579Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/associated_const_generic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/associated_const_generic" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/associated_const_generic/auxiliary"
2020-03-24T23:51:27.1662013Z ------------------------------------------
2020-03-24T23:51:27.1662256Z 
2020-03-24T23:51:27.1662658Z ------------------------------------------
2020-03-24T23:51:27.1662927Z stderr:
2020-03-24T23:51:27.1662927Z stderr:
2020-03-24T23:51:27.1663330Z ------------------------------------------
2020-03-24T23:51:27.1663680Z error: constant expression depends on a generic parameter
2020-03-24T23:51:27.1664200Z   --> /checkout/src/test/ui/consts/associated_const_generic.rs:24:17
2020-03-24T23:51:27.1664530Z    |
2020-03-24T23:51:27.1664842Z LL |     let _ = [0; B::VALUE]; // Indirectly refers to `A::VALUE`
2020-03-24T23:51:27.1665419Z    |
2020-03-24T23:51:27.1665708Z    = note: this may fail depending on what value the parameter takes
2020-03-24T23:51:27.1665973Z 
2020-03-24T23:51:27.1666233Z error: aborting due to previous error
---
2020-03-24T23:51:27.1671578Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-24T23:51:27.1672128Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-24T23:51:27.1675481Z 
2020-03-24T23:51:27.1675733Z 
2020-03-24T23:51:27.1680737Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-24T23:51:27.1682752Z 
2020-03-24T23:51:27.1682826Z 
2020-03-24T23:51:27.1686829Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-24T23:51:27.1687110Z Build completed unsuccessfully in 0:49:02
2020-03-24T23:51:27.1687110Z Build completed unsuccessfully in 0:49:02
2020-03-24T23:51:27.1729467Z == clock drift check ==
2020-03-24T23:51:27.1755311Z   local time: Tue Mar 24 23:51:27 UTC 2020
2020-03-24T23:51:27.4671754Z   network time: Tue, 24 Mar 2020 23:51:27 GMT
2020-03-24T23:51:27.4676100Z == end clock drift check ==
2020-03-24T23:51:27.9588662Z 
2020-03-24T23:51:27.9660992Z ##[error]Bash exited with code '1'.
2020-03-24T23:51:27.9671203Z ##[section]Finishing: Run build
2020-03-24T23:51:27.9715823Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70107/merge to s
2020-03-24T23:51:27.9720884Z Task         : Get sources
2020-03-24T23:51:27.9721220Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-24T23:51:27.9721534Z Version      : 1.0.0
2020-03-24T23:51:27.9721747Z Author       : Microsoft
2020-03-24T23:51:27.9721747Z Author       : Microsoft
2020-03-24T23:51:27.9722082Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-24T23:51:27.9722480Z ==============================================================================
2020-03-24T23:51:28.2413153Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-24T23:51:28.2454529Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70107/merge to s
2020-03-24T23:51:28.2518234Z Cleaning up task key
2020-03-24T23:51:28.2519395Z Start cleaning up orphan processes.
2020-03-24T23:51:28.2757205Z Terminate orphan process: pid (4024) (python)
2020-03-24T23:51:28.2790804Z ##[section]Finishing: Finalize Job
