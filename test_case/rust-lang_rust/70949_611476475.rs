plain
2020-04-09T10:12:40.2145894Z ========================== Starting Command Output ===========================
2020-04-09T10:12:40.2149099Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/512f25f3-e2da-4d12-830e-1bc0474c77c6.sh
2020-04-09T10:12:40.2149371Z 
2020-04-09T10:12:40.2152858Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-09T10:12:40.2171754Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70949/merge to s
2020-04-09T10:12:40.2175492Z Task         : Get sources
2020-04-09T10:12:40.2175784Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-09T10:12:40.2176065Z Version      : 1.0.0
2020-04-09T10:12:40.2176308Z Author       : Microsoft
---
2020-04-09T10:12:41.4518525Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-09T10:12:41.4529164Z ##[command]git config gc.auto 0
2020-04-09T10:12:41.4535500Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-09T10:12:41.4542001Z ##[command]git config --get-all http.proxy
2020-04-09T10:12:41.4554809Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70949/merge:refs/remotes/pull/70949/merge
---
2020-04-09T10:15:04.8331081Z Looks like docker image is the same as before, not uploading
2020-04-09T10:15:12.5474940Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-09T10:15:12.5755825Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-09T10:15:12.5788213Z == clock drift check ==
2020-04-09T10:15:12.5798303Z   local time: Thu Apr  9 10:15:12 UTC 2020
2020-04-09T10:15:12.7318137Z   network time: Thu, 09 Apr 2020 10:15:12 GMT
2020-04-09T10:15:12.7345087Z Starting sccache server...
2020-04-09T10:15:12.8168993Z configure: processing command line
2020-04-09T10:15:12.8169662Z configure: 
2020-04-09T10:15:12.8171289Z configure: rust.dist-src        := False
---
2020-04-09T10:20:35.1757448Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-09T10:20:36.8130467Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-09T10:20:38.5198799Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-09T10:20:40.2457121Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-09T10:20:49.1900896Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-09T10:20:52.5098534Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-09T10:20:57.4107283Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-09T10:21:02.1402020Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-09T10:21:11.3531637Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-09T10:44:59.7587958Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-09T10:45:01.7110873Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-09T10:45:03.8446060Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-09T10:45:06.0737151Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-09T10:45:17.1274634Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-09T10:45:20.5090268Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-09T10:45:26.1903503Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-09T10:45:31.9067848Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-09T10:45:42.9883879Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-09T11:12:28.7498377Z .................................................................................................... 1700/9879
2020-04-09T11:12:32.7909776Z .................................................................................................... 1800/9879
2020-04-09T11:12:42.1202409Z ..................................................................................................i. 1900/9879
2020-04-09T11:12:50.3642236Z .................................................................................................... 2000/9879
2020-04-09T11:12:56.9599131Z ........................................................................................iiiii....... 2100/9879
2020-04-09T11:13:18.8447237Z .................................................................................................... 2300/9879
2020-04-09T11:13:21.0476778Z .................................................................................................... 2400/9879
2020-04-09T11:13:23.2929469Z .................................................................................................... 2500/9879
2020-04-09T11:13:29.6103218Z .................................................................................................... 2600/9879
---
2020-04-09T11:16:28.2244274Z ..............................................................i...............i..................... 5000/9879
2020-04-09T11:16:35.6571418Z .................................................................................................... 5100/9879
2020-04-09T11:16:43.4896484Z .................................................................................................... 5200/9879
2020-04-09T11:16:49.0394974Z .......i............................................................................................ 5300/9879
2020-04-09T11:16:59.0632627Z ................................................................................................ii.i 5400/9879
2020-04-09T11:17:04.1822778Z i........i...i...................................................................................... 5500/9879
2020-04-09T11:17:12.9473899Z .........................................i.......................................................... 5700/9879
2020-04-09T11:17:22.9916616Z .............................................................ii..................................... 5800/9879
2020-04-09T11:17:30.1122857Z i................................................................................................... 5900/9879
2020-04-09T11:17:35.3487414Z .................................................................................................... 6000/9879
2020-04-09T11:17:35.3487414Z .................................................................................................... 6000/9879
2020-04-09T11:17:45.1400253Z ..............................................................................................ii...i 6100/9879
2020-04-09T11:17:57.1284752Z ..ii...........i.................................................................................... 6200/9879
2020-04-09T11:18:13.3142845Z .................................................................................................... 6400/9879
2020-04-09T11:18:19.3768529Z .................................................................................................... 6500/9879
2020-04-09T11:18:19.3768529Z .................................................................................................... 6500/9879
2020-04-09T11:18:40.6520065Z ........................i..ii....................................................................... 6600/9879
2020-04-09T11:19:01.8433175Z .................................................................................................... 6800/9879
2020-04-09T11:19:04.0865565Z ........................i........................................................................... 6900/9879
2020-04-09T11:19:06.2441863Z .................................................................................................... 7000/9879
2020-04-09T11:19:08.5133109Z ...............................................................i.................................... 7100/9879
---
2020-04-09T11:20:50.9928506Z .................................................................................................... 7800/9879
2020-04-09T11:20:55.4880379Z .................................................................................................... 7900/9879
2020-04-09T11:21:01.6079061Z .................................................................................................... 8000/9879
2020-04-09T11:21:09.3133447Z ............................i....................................................................... 8100/9879
2020-04-09T11:21:17.9386589Z ............................................................................iiiiii.iiii.i........... 8200/9879
2020-04-09T11:21:34.2622543Z .....................i......i....................................................................... 8400/9879
2020-04-09T11:21:38.9689263Z .................................................................................................... 8500/9879
2020-04-09T11:21:50.0378795Z .................................................................................................... 8600/9879
2020-04-09T11:22:02.6796022Z .................................................................................................... 8700/9879
---
2020-04-09T11:24:03.0129811Z 9 
2020-04-09T11:24:03.0130027Z 
2020-04-09T11:24:03.0130234Z 
2020-04-09T11:24:03.0130566Z The actual stderr differed from the expected stderr.
2020-04-09T11:24:03.0131474Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/vec/vec-macro-with-comma-only/vec-macro-with-comma-only.stderr
2020-04-09T11:24:03.0132526Z To update references, rerun the tests and pass the `--bless` flag
2020-04-09T11:24:03.0133457Z To only update this specific test, also pass `--test-args vec/vec-macro-with-comma-only.rs`
2020-04-09T11:24:03.0134370Z error: 1 errors occurred comparing output.
2020-04-09T11:24:03.0135041Z status: exit code: 1
2020-04-09T11:24:03.0135041Z status: exit code: 1
2020-04-09T11:24:03.0137376Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/vec/vec-macro-with-comma-only.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/vec/vec-macro-with-comma-only" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/vec/vec-macro-with-comma-only/auxiliary"
2020-04-09T11:24:03.0139297Z ------------------------------------------
2020-04-09T11:24:03.0139626Z 
2020-04-09T11:24:03.0140117Z ------------------------------------------
2020-04-09T11:24:03.0140448Z stderr:
2020-04-09T11:24:03.0140448Z stderr:
2020-04-09T11:24:03.0140966Z ------------------------------------------
2020-04-09T11:24:03.0141531Z error[E0282]: type annotations needed
2020-04-09T11:24:03.0142366Z   --> /checkout/src/test/ui/vec/vec-macro-with-comma-only.rs:2:5
2020-04-09T11:24:03.0142945Z    |
2020-04-09T11:24:03.0143279Z LL |     vec![,]; //~ ERROR no rules expected the token `,`
2020-04-09T11:24:03.0144190Z    |
2020-04-09T11:24:03.0144873Z    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
2020-04-09T11:24:03.0145480Z 
2020-04-09T11:24:03.0145773Z error: aborting due to previous error
---
2020-04-09T11:24:03.0150280Z test result: FAILED. 9818 passed; 1 failed; 60 ignored; 0 measured; 0 filtered out
2020-04-09T11:24:03.0150695Z 
2020-04-09T11:24:03.0157733Z 
2020-04-09T11:24:03.0158025Z 
2020-04-09T11:24:03.0165627Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-09T11:24:03.0170871Z 
2020-04-09T11:24:03.0171535Z 
2020-04-09T11:24:03.0172484Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-09T11:24:03.0174422Z Build completed unsuccessfully in 1:07:14
2020-04-09T11:24:03.0174422Z Build completed unsuccessfully in 1:07:14
2020-04-09T11:24:03.0177293Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-09T11:24:03.0177745Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-09T11:24:03.0207994Z == clock drift check ==
2020-04-09T11:24:03.0231113Z   local time: Thu Apr  9 11:24:03 UTC 2020
2020-04-09T11:24:03.1222627Z   network time: Thu, 09 Apr 2020 11:24:03 GMT
2020-04-09T11:24:03.7501591Z 
2020-04-09T11:24:03.7501591Z 
2020-04-09T11:24:03.7569362Z ##[error]Bash exited with code '1'.
2020-04-09T11:24:03.7582506Z ##[section]Finishing: Run build
2020-04-09T11:24:03.7636208Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70949/merge to s
2020-04-09T11:24:03.7642564Z Task         : Get sources
2020-04-09T11:24:03.7642865Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-09T11:24:03.7643518Z Version      : 1.0.0
2020-04-09T11:24:03.7643929Z Author       : Microsoft
2020-04-09T11:24:03.7643929Z Author       : Microsoft
2020-04-09T11:24:03.7644248Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-09T11:24:03.7644635Z ==============================================================================
2020-04-09T11:24:04.1018397Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-09T11:24:04.1069221Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70949/merge to s
2020-04-09T11:24:04.1165764Z Cleaning up task key
2020-04-09T11:24:04.1167125Z Start cleaning up orphan processes.
2020-04-09T11:24:04.1369252Z Terminate orphan process: pid (3854) (python)
2020-04-09T11:24:04.1598614Z ##[section]Finishing: Finalize Job
