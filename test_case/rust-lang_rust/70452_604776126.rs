plain
2020-03-27T00:54:34.1692580Z ========================== Starting Command Output ===========================
2020-03-27T00:54:34.1695964Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/e275a67f-073f-4567-ac0f-47f57698bee9.sh
2020-03-27T00:54:34.1696390Z 
2020-03-27T00:54:34.1700061Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-27T00:54:34.1718983Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70452/merge to s
2020-03-27T00:54:34.1722275Z Task         : Get sources
2020-03-27T00:54:34.1722593Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-27T00:54:34.1722919Z Version      : 1.0.0
2020-03-27T00:54:34.1723122Z Author       : Microsoft
---
2020-03-27T00:54:35.3749351Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-27T00:54:35.3757166Z ##[command]git config gc.auto 0
2020-03-27T00:54:35.3761470Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-27T00:54:35.3765302Z ##[command]git config --get-all http.proxy
2020-03-27T00:54:35.3773053Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70452/merge:refs/remotes/pull/70452/merge
---
2020-03-27T01:01:55.3278065Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-27T01:02:05.5173317Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-27T01:02:11.9007395Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-27T01:02:24.2958506Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-27T01:02:27.2570024Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-27T01:02:28.6485814Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-27T01:03:02.3070996Z    Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-03-27T01:03:10.5884843Z    Compiling rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-03-27T01:04:00.7634243Z    Compiling rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
---
2020-03-27T01:24:24.0780129Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-27T01:24:36.5208479Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-27T01:24:45.4331579Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-27T01:24:59.3099630Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-27T01:25:04.3303472Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-27T01:25:05.8815627Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-27T01:25:50.2641779Z    Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-03-27T01:26:00.3906457Z    Compiling rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-03-27T01:27:06.4196018Z    Compiling rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
---
2020-03-27T01:50:42.9961253Z .................................................................................................... 1700/9847
2020-03-27T01:50:47.0853630Z .................................................................................................... 1800/9847
2020-03-27T01:50:57.3981699Z .........................................................................................i.......... 1900/9847
2020-03-27T01:51:04.3141872Z .................................................................................................... 2000/9847
2020-03-27T01:51:10.7944809Z ...............................................................................iiiii................ 2100/9847
2020-03-27T01:51:32.2665153Z .................................................................................................... 2300/9847
2020-03-27T01:51:34.4734159Z .................................................................................................... 2400/9847
2020-03-27T01:51:37.0137352Z .................................................................................................... 2500/9847
2020-03-27T01:51:46.2052249Z .................................................................................................... 2600/9847
---
2020-03-27T01:54:38.9829977Z .....................................................i...............i.............................. 5000/9847
2020-03-27T01:54:47.1183641Z .................................................................................................... 5100/9847
2020-03-27T01:54:55.0018536Z ..................................................................................................i. 5200/9847
2020-03-27T01:55:00.3382705Z .................................................................................................... 5300/9847
2020-03-27T01:55:11.1280560Z .................................................................................ii.ii........i...i. 5400/9847
2020-03-27T01:55:18.6177765Z .....................i.............................................................................. 5600/9847
2020-03-27T01:55:26.0771515Z ..........................i......................................................................... 5700/9847
2020-03-27T01:55:34.0555822Z ...........................................ii....................................i.................. 5800/9847
2020-03-27T01:55:41.5708997Z .................................................................................................... 5900/9847
2020-03-27T01:55:41.5708997Z .................................................................................................... 5900/9847
2020-03-27T01:55:47.0266331Z .................................................................................................... 6000/9847
2020-03-27T01:55:56.3599787Z ...........................................................................ii...i..ii...........i... 6100/9847
2020-03-27T01:56:17.0556954Z .................................................................................................... 6300/9847
2020-03-27T01:56:21.2753882Z .................................................................................................... 6400/9847
2020-03-27T01:56:24.9068099Z .................................................................................................... 6500/9847
2020-03-27T01:56:24.9068099Z .................................................................................................... 6500/9847
2020-03-27T01:56:37.1537715Z .....i..ii.......................................................................................... 6600/9847
2020-03-27T01:56:57.3700024Z .................................................................................................... 6800/9847
2020-03-27T01:56:59.5042217Z .....i.............................................................................................. 6900/9847
2020-03-27T01:57:01.6490836Z .................................................................................................... 7000/9847
2020-03-27T01:57:04.0085849Z .........................................i.......................................................... 7100/9847
---
2020-03-27T01:58:44.7547423Z .................................................................................................... 7800/9847
2020-03-27T01:58:49.7695329Z .................................................................................................... 7900/9847
2020-03-27T01:58:56.7334910Z ..................................................................................................i. 8000/9847
2020-03-27T01:59:04.3855420Z .................................................................................................... 8100/9847
2020-03-27T01:59:12.1500307Z ...............................................iiiiiiiiii.i......................................... 8200/9847
2020-03-27T01:59:21.5866687Z ...........................................................................................i......i. 8300/9847
2020-03-27T01:59:31.3179524Z .................................................................................................... 8500/9847
2020-03-27T01:59:44.6857760Z .................................................................................................... 8600/9847
2020-03-27T01:59:53.9489750Z .................................................................................................... 8700/9847
2020-03-27T01:59:59.5437603Z .................................................................................................... 8800/9847
---
2020-03-27T02:01:49.2130814Z normalized stderr:
2020-03-27T02:01:49.2131215Z error: constant expression depends on a generic parameter
2020-03-27T02:01:49.2131821Z   --> $DIR/issue-66205.rs:5:12
2020-03-27T02:01:49.2132129Z    |
2020-03-27T02:01:49.2132623Z LL |     fact::<{ N - 1 }>();
2020-03-27T02:01:49.2133238Z    |
2020-03-27T02:01:49.2133624Z    = note: this may fail depending on what value the parameter takes
2020-03-27T02:01:49.2133954Z 
2020-03-27T02:01:49.2134249Z error: aborting due to previous error
2020-03-27T02:01:49.2134249Z error: aborting due to previous error
2020-03-27T02:01:49.2134757Z 
2020-03-27T02:01:49.2134957Z 
2020-03-27T02:01:49.2135153Z 
2020-03-27T02:01:49.2135432Z 
2020-03-27T02:01:49.2135795Z The actual stderr differed from the expected stderr.
2020-03-27T02:01:49.2136637Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-66205/issue-66205.stderr
2020-03-27T02:01:49.2137429Z To update references, rerun the tests and pass the `--bless` flag
2020-03-27T02:01:49.2138208Z To only update this specific test, also pass `--test-args const-generics/issues/issue-66205.rs`
2020-03-27T02:01:49.2138906Z error: 1 errors occurred comparing output.
2020-03-27T02:01:49.2139251Z status: exit code: 1
2020-03-27T02:01:49.2139251Z status: exit code: 1
2020-03-27T02:01:49.2141430Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-66205.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-66205" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-66205/auxiliary"
2020-03-27T02:01:49.2143364Z ------------------------------------------
2020-03-27T02:01:49.2143655Z 
2020-03-27T02:01:49.2144139Z ------------------------------------------
2020-03-27T02:01:49.2144476Z stderr:
2020-03-27T02:01:49.2144476Z stderr:
2020-03-27T02:01:49.2144972Z ------------------------------------------
2020-03-27T02:01:49.2146037Z error: constant expression depends on a generic parameter
2020-03-27T02:01:49.2148555Z   --> /checkout/src/test/ui/const-generics/issues/issue-66205.rs:5:12
2020-03-27T02:01:49.2148988Z    |
2020-03-27T02:01:49.2149492Z LL |     fact::<{ N - 1 }>();
2020-03-27T02:01:49.2151172Z    |
2020-03-27T02:01:49.2151456Z    = note: this may fail depending on what value the parameter takes
2020-03-27T02:01:49.2151682Z 
2020-03-27T02:01:49.2151866Z error: aborting due to previous error
---
2020-03-27T02:01:49.2153525Z normalized stderr:
2020-03-27T02:01:49.2153778Z error: constant expression depends on a generic parameter
2020-03-27T02:01:49.2154235Z   --> $DIR/issue-67739.rs:12:15
2020-03-27T02:01:49.2154446Z    |
2020-03-27T02:01:49.2154692Z LL |         [0u8; mem::size_of::<Self::Associated>()];
2020-03-27T02:01:49.2155253Z    |
2020-03-27T02:01:49.2155505Z    = note: this may fail depending on what value the parameter takes
2020-03-27T02:01:49.2155733Z 
2020-03-27T02:01:49.2155935Z error: aborting due to previous error
2020-03-27T02:01:49.2155935Z error: aborting due to previous error
2020-03-27T02:01:49.2156104Z 
2020-03-27T02:01:49.2156202Z 
2020-03-27T02:01:49.2156298Z 
2020-03-27T02:01:49.2156394Z 
2020-03-27T02:01:49.2156618Z The actual stderr differed from the expected stderr.
2020-03-27T02:01:49.2157314Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-67739/issue-67739.stderr
2020-03-27T02:01:49.2158194Z To update references, rerun the tests and pass the `--bless` flag
2020-03-27T02:01:49.2161948Z To only update this specific test, also pass `--test-args const-generics/issues/issue-67739.rs`
2020-03-27T02:01:49.2162442Z error: 1 errors occurred comparing output.
2020-03-27T02:01:49.2162702Z status: exit code: 1
2020-03-27T02:01:49.2162702Z status: exit code: 1
2020-03-27T02:01:49.2164989Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-67739.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-67739" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-67739/auxiliary"
2020-03-27T02:01:49.2167043Z ------------------------------------------
2020-03-27T02:01:49.2167225Z 
2020-03-27T02:01:49.2167620Z ------------------------------------------
2020-03-27T02:01:49.2167827Z stderr:
2020-03-27T02:01:49.2167827Z stderr:
2020-03-27T02:01:49.2168203Z ------------------------------------------
2020-03-27T02:01:49.2168515Z error: constant expression depends on a generic parameter
2020-03-27T02:01:49.2169077Z   --> /checkout/src/test/ui/const-generics/issues/issue-67739.rs:12:15
2020-03-27T02:01:49.2169344Z    |
2020-03-27T02:01:49.2170434Z LL |         [0u8; mem::size_of::<Self::Associated>()];
2020-03-27T02:01:49.2171008Z    |
2020-03-27T02:01:49.2171273Z    = note: this may fail depending on what value the parameter takes
2020-03-27T02:01:49.2171498Z 
2020-03-27T02:01:49.2171681Z error: aborting due to previous error
---
2020-03-27T02:01:49.2175278Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-27T02:01:49.2175715Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-27T02:01:49.2176631Z 
2020-03-27T02:01:49.2176739Z 
2020-03-27T02:01:49.2180546Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-27T02:01:49.2183275Z 
2020-03-27T02:01:49.2183376Z 
2020-03-27T02:01:49.2183907Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-27T02:01:49.2184284Z Build completed unsuccessfully in 1:03:17
2020-03-27T02:01:49.2184284Z Build completed unsuccessfully in 1:03:17
2020-03-27T02:01:49.6710439Z == clock drift check ==
2020-03-27T02:01:49.6710821Z   local time: Fri Mar 27 02:01:49 UTC 2020
2020-03-27T02:01:49.6711323Z   network time: Fri, 27 Mar 2020 02:01:49 GMT
2020-03-27T02:01:49.6711845Z == end clock drift check ==
2020-03-27T02:01:49.6712905Z 
2020-03-27T02:01:49.6759401Z ##[error]Bash exited with code '1'.
2020-03-27T02:01:49.6773801Z ##[section]Finishing: Run build
2020-03-27T02:01:49.6823576Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70452/merge to s
2020-03-27T02:01:49.6829094Z Task         : Get sources
2020-03-27T02:01:49.6829468Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-27T02:01:49.6829796Z Version      : 1.0.0
2020-03-27T02:01:49.6830026Z Author       : Microsoft
2020-03-27T02:01:49.6830026Z Author       : Microsoft
2020-03-27T02:01:49.6830411Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-27T02:01:49.6830830Z ==============================================================================
2020-03-27T02:01:50.0215045Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-27T02:01:50.0264124Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70452/merge to s
2020-03-27T02:01:50.0354207Z Cleaning up task key
2020-03-27T02:01:50.0355588Z Start cleaning up orphan processes.
2020-03-27T02:01:50.0556046Z Terminate orphan process: pid (8318) (python)
2020-03-27T02:01:50.0802009Z ##[section]Finishing: Finalize Job
