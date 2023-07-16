plain
2020-03-27T22:24:19.7790982Z ========================== Starting Command Output ===========================
2020-03-27T22:24:19.7793405Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c4ad6f10-7d30-40d4-a506-9e8054ea14d4.sh
2020-03-27T22:24:19.7793675Z 
2020-03-27T22:24:19.7797378Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-27T22:24:19.7816579Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70458/merge to s
2020-03-27T22:24:19.7819793Z Task         : Get sources
2020-03-27T22:24:19.7820098Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-27T22:24:19.7820411Z Version      : 1.0.0
2020-03-27T22:24:19.7820609Z Author       : Microsoft
---
2020-03-27T22:24:20.7758972Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-27T22:24:20.7770867Z ##[command]git config gc.auto 0
2020-03-27T22:24:20.7774732Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-27T22:24:20.7778016Z ##[command]git config --get-all http.proxy
2020-03-27T22:24:20.7784366Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70458/merge:refs/remotes/pull/70458/merge
---
2020-03-27T22:31:36.2323126Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-27T22:31:37.7829575Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-27T22:31:38.3701214Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-27T22:31:47.3944762Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-27T22:31:49.0760130Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-27T22:31:50.4753748Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-27T22:31:58.8488152Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-03-27T22:32:29.7449200Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-27T22:33:02.7089616Z    Compiling rustc_infer v0.0.0 (/checkout/src/librustc_infer)
2020-03-27T22:34:47.5960699Z    Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
---
2020-03-27T22:53:32.1237218Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-27T22:53:34.0984237Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-27T22:53:34.9899168Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-27T22:53:46.1841716Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-27T22:53:48.1276914Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-27T22:53:49.6192447Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-27T22:54:00.2181227Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-03-27T22:54:40.0136394Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-27T22:55:22.9689144Z    Compiling rustc_infer v0.0.0 (/checkout/src/librustc_infer)
2020-03-27T22:57:28.0815106Z    Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
---
2020-03-27T23:19:07.8950792Z .................................................................................................... 1700/9849
2020-03-27T23:19:11.9951843Z .................................................................................................... 1800/9849
2020-03-27T23:19:22.2620234Z .........................................................................................i.......... 1900/9849
2020-03-27T23:19:29.2585118Z .................................................................................................... 2000/9849
2020-03-27T23:19:35.7730764Z ...............................................................................iiiii................ 2100/9849
2020-03-27T23:19:57.0546911Z .................................................................................................... 2300/9849
2020-03-27T23:19:59.2570467Z .................................................................................................... 2400/9849
2020-03-27T23:20:01.7669891Z .................................................................................................... 2500/9849
2020-03-27T23:20:11.2221730Z .................................................................................................... 2600/9849
---
2020-03-27T23:23:04.0000299Z .....................................................i...............i.............................. 5000/9849
2020-03-27T23:23:12.6822657Z .................................................................................................... 5100/9849
2020-03-27T23:23:19.7185175Z ..................................................................................................i. 5200/9849
2020-03-27T23:23:25.0360062Z .................................................................................................... 5300/9849
2020-03-27T23:23:35.7624691Z ...................................................................................ii.ii........i... 5400/9849
2020-03-27T23:23:39.4471447Z i................................................................................................... 5500/9849
2020-03-27T23:23:43.2028878Z .......................i............................................................................ 5600/9849
2020-03-27T23:23:49.1443400Z ............................i......................................................................1 warning: Linking globals named 'foo': symbol multiply defined!
2020-03-27T23:23:49.1448169Z 2 
2020-03-27T23:23:49.1449135Z - error: failed to load bc of "lto_duplicate_symbols2.3a1fbbbh-cgu.0": 
2020-03-27T23:23:49.1450020Z + error: failed to load bc of "lto-duplicate-symbols2.lto_duplicate_symbols2.3a1fbbbh-cgu.0.rcgu.o": 
2020-03-27T23:23:49.1451600Z 5 error: aborting due to previous error
2020-03-27T23:23:49.1452254Z 6 
2020-03-27T23:23:49.1452383Z 
2020-03-27T23:23:49.1476868Z F 5700/9849
2020-03-27T23:23:49.1476868Z F 5700/9849
2020-03-27T23:23:58.4855899Z .............................................ii....................................i................ 5800/9849
2020-03-27T23:24:05.9161742Z .................................................................................................... 5900/9849
2020-03-27T23:24:11.2265833Z .................................................................................................... 6000/9849
2020-03-27T23:24:20.3150677Z .............................................................................ii...i..ii...........i. 6100/9849
2020-03-27T23:24:41.1070858Z .................................................................................................... 6300/9849
2020-03-27T23:24:45.2180445Z .................................................................................................... 6400/9849
2020-03-27T23:24:48.7024638Z .................................................................................................... 6500/9849
2020-03-27T23:24:48.7024638Z .................................................................................................... 6500/9849
2020-03-27T23:25:00.5634119Z .......i..ii........................................................................................ 6600/9849
2020-03-27T23:25:20.1655442Z .................................................................................................... 6800/9849
2020-03-27T23:25:22.2552383Z .......i............................................................................................ 6900/9849
2020-03-27T23:25:24.4817022Z .................................................................................................... 7000/9849
2020-03-27T23:25:26.8109992Z ...........................................i........................................................ 7100/9849
---
2020-03-27T23:26:59.7940756Z .................................................................................................... 7700/9849
2020-03-27T23:27:04.7586364Z .................................................................................................... 7800/9849
2020-03-27T23:27:09.6002677Z .................................................................................................... 7900/9849
2020-03-27T23:27:16.6593641Z .................................................................................................... 8000/9849
2020-03-27T23:27:24.0962824Z i................................................................................................... 8100/9849
2020-03-27T23:27:31.8278845Z .................................................iiiiiiiiii.i....................................... 8200/9849
2020-03-27T23:27:45.5287548Z i................................................................................................... 8400/9849
2020-03-27T23:27:50.5685129Z .................................................................................................... 8500/9849
2020-03-27T23:28:03.4762624Z .................................................................................................... 8600/9849
2020-03-27T23:28:12.6048347Z .................................................................................................... 8700/9849
---
2020-03-27T23:30:04.2819173Z 
2020-03-27T23:30:04.2819377Z 
2020-03-27T23:30:04.2819714Z The actual stderr differed from the expected stderr.
2020-03-27T23:30:04.2820544Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto-duplicate-symbols/lto-duplicate-symbols.stderr
2020-03-27T23:30:04.2821324Z To update references, rerun the tests and pass the `--bless` flag
2020-03-27T23:30:04.2822076Z To only update this specific test, also pass `--test-args lto-duplicate-symbols.rs`
2020-03-27T23:30:04.2822735Z error: 1 errors occurred comparing output.
2020-03-27T23:30:04.2823513Z status: exit code: 1
2020-03-27T23:30:04.2823513Z status: exit code: 1
2020-03-27T23:30:04.2825934Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lto-duplicate-symbols.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto-duplicate-symbols" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-C" "lto" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto-duplicate-symbols/auxiliary"
2020-03-27T23:30:04.2828202Z ------------------------------------------
2020-03-27T23:30:04.2828573Z 
2020-03-27T23:30:04.2828949Z ------------------------------------------
2020-03-27T23:30:04.2829153Z stderr:
2020-03-27T23:30:04.2829153Z stderr:
2020-03-27T23:30:04.2829537Z ------------------------------------------
2020-03-27T23:30:04.2830128Z warning: Linking globals named 'foo': symbol multiply defined!
2020-03-27T23:30:04.2830356Z 
2020-03-27T23:30:04.2830947Z error: failed to load bc of "lto-duplicate-symbols2.lto_duplicate_symbols2.3a1fbbbh-cgu.0.rcgu.o": 
2020-03-27T23:30:04.2831452Z error: aborting due to previous error
2020-03-27T23:30:04.2831618Z 
2020-03-27T23:30:04.2831730Z 
2020-03-27T23:30:04.2832088Z ------------------------------------------
---
2020-03-27T23:30:04.2843474Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-27T23:30:04.2843908Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-27T23:30:04.2854984Z 
2020-03-27T23:30:04.2855133Z 
2020-03-27T23:30:04.2859022Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-27T23:30:04.2863987Z 
2020-03-27T23:30:04.2864089Z 
2020-03-27T23:30:04.2865086Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-27T23:30:04.2865474Z Build completed unsuccessfully in 1:01:57
2020-03-27T23:30:04.2865474Z Build completed unsuccessfully in 1:01:57
2020-03-27T23:30:04.2917938Z == clock drift check ==
2020-03-27T23:30:04.2934218Z   local time: Fri Mar 27 23:30:04 UTC 2020
2020-03-27T23:30:04.3841917Z   network time: Fri, 27 Mar 2020 23:30:04 GMT
2020-03-27T23:30:04.3842457Z == end clock drift check ==
2020-03-27T23:30:04.8530358Z 
2020-03-27T23:30:04.8570844Z ##[error]Bash exited with code '1'.
2020-03-27T23:30:04.8586755Z ##[section]Finishing: Run build
2020-03-27T23:30:04.8638329Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70458/merge to s
2020-03-27T23:30:04.8643344Z Task         : Get sources
2020-03-27T23:30:04.8643688Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-27T23:30:04.8644018Z Version      : 1.0.0
2020-03-27T23:30:04.8644247Z Author       : Microsoft
2020-03-27T23:30:04.8644247Z Author       : Microsoft
2020-03-27T23:30:04.8644601Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-27T23:30:04.8645019Z ==============================================================================
2020-03-27T23:30:05.1960423Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-27T23:30:05.1964545Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70458/merge to s
2020-03-27T23:30:05.2060016Z Cleaning up task key
2020-03-27T23:30:05.2061336Z Start cleaning up orphan processes.
2020-03-27T23:30:05.2237551Z Terminate orphan process: pid (3481) (python)
2020-03-27T23:30:05.2405756Z ##[section]Finishing: Finalize Job
