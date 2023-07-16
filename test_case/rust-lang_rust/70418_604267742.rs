plain
2020-03-26T06:10:07.7882529Z ========================== Starting Command Output ===========================
2020-03-26T06:10:07.7885655Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b29bcebd-7af9-4744-a566-08e5c006fb7c.sh
2020-03-26T06:10:07.7885986Z 
2020-03-26T06:10:07.7889775Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-26T06:10:07.7906059Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70418/merge to s
2020-03-26T06:10:07.7908905Z Task         : Get sources
2020-03-26T06:10:07.7909122Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-26T06:10:07.7909542Z Version      : 1.0.0
2020-03-26T06:10:07.7909705Z Author       : Microsoft
---
2020-03-26T06:10:10.5321403Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-26T06:10:10.5330736Z ##[command]git config gc.auto 0
2020-03-26T06:10:10.5336981Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-26T06:10:10.5340268Z ##[command]git config --get-all http.proxy
2020-03-26T06:10:10.5349329Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70418/merge:refs/remotes/pull/70418/merge
---
2020-03-26T06:17:56.9836373Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-26T06:17:57.0575092Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-26T06:18:05.8155783Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-26T06:18:13.8269927Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-26T06:18:17.4980486Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-26T06:18:19.3350867Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-26T06:18:48.5788647Z    Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-03-26T06:18:55.9469154Z    Compiling rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-03-26T06:19:41.0956282Z    Compiling rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
---
2020-03-26T06:38:01.2619154Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-26T06:38:12.2007540Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-26T06:38:19.1359471Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-26T06:38:33.1524770Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-26T06:38:35.5455657Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-26T06:38:38.0031470Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-26T06:39:17.2166716Z    Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-03-26T06:39:26.0805497Z    Compiling rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-03-26T06:40:23.9081683Z    Compiling rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
---
2020-03-26T07:01:17.0166378Z .................................................................................................... 1700/9835
2020-03-26T07:01:20.6298489Z .................................................................................................... 1800/9835
2020-03-26T07:01:29.6006807Z .........................................................................................i.......... 1900/9835
2020-03-26T07:01:35.7602045Z .................................................................................................... 2000/9835
2020-03-26T07:01:41.5465612Z ...............................................................................iiiii................ 2100/9835
2020-03-26T07:02:00.3771129Z .................................................................................................... 2300/9835
2020-03-26T07:02:02.3316759Z .................................................................................................... 2400/9835
2020-03-26T07:02:04.5960547Z .................................................................................................... 2500/9835
2020-03-26T07:02:10.7483417Z .................................................................................................... 2600/9835
---
2020-03-26T07:04:53.6968962Z ......................................................i...............i............................. 5000/9835
2020-03-26T07:05:01.1918010Z .................................................................................................... 5100/9835
2020-03-26T07:05:08.0721307Z ...................................................................................................i 5200/9835
2020-03-26T07:05:12.8310284Z .................................................................................................... 5300/9835
2020-03-26T07:05:22.6706263Z ..................................................................................ii.ii........i...i 5400/9835
2020-03-26T07:05:29.9346495Z ......................i............................................................................. 5600/9835
2020-03-26T07:05:37.1443319Z ...........................i........................................................................ 5700/9835
2020-03-26T07:05:44.6790565Z ............................................ii....................................i................. 5800/9835
2020-03-26T07:05:51.6783077Z .................................................................................................... 5900/9835
2020-03-26T07:05:51.6783077Z .................................................................................................... 5900/9835
2020-03-26T07:05:56.6262218Z .................................................................................................... 6000/9835
2020-03-26T07:06:05.4670932Z ............................................................................ii...i..ii...........i.. 6100/9835
2020-03-26T07:06:24.9490702Z .................................................................................................... 6300/9835
2020-03-26T07:06:28.7952226Z .................................................................................................... 6400/9835
2020-03-26T07:06:32.1226073Z .................................................................................................... 6500/9835
2020-03-26T07:06:32.1226073Z .................................................................................................... 6500/9835
2020-03-26T07:06:43.3263184Z ......i..ii......................................................................................... 6600/9835
2020-03-26T07:07:01.2413665Z .................................................................................................... 6800/9835
2020-03-26T07:07:03.2451925Z .....i.............................................................................................. 6900/9835
2020-03-26T07:07:05.0860475Z .....................................................F.............................................. 7000/9835
2020-03-26T07:07:07.1585832Z ........................................i........................................................... 7100/9835
---
2020-03-26T07:08:36.2448330Z .................................................................................................... 7800/9835
2020-03-26T07:08:40.5063736Z .................................................................................................... 7900/9835
2020-03-26T07:08:46.8999564Z .................................................................................................i.. 8000/9835
2020-03-26T07:08:53.7350177Z .................................................................................................... 8100/9835
2020-03-26T07:09:00.4726019Z ..............................................iiiiiiiiii.i.......................................... 8200/9835
2020-03-26T07:09:09.9470630Z ..........................................................................................i......i.. 8300/9835
2020-03-26T07:09:18.6801579Z .................................................................................................... 8500/9835
2020-03-26T07:09:30.9724321Z .................................................................................................... 8600/9835
2020-03-26T07:09:39.4742742Z .................................................................................................... 8700/9835
2020-03-26T07:09:44.7758616Z .................................................................................................... 8800/9835
---
2020-03-26T07:11:25.2307129Z 
2020-03-26T07:11:25.2307227Z 
2020-03-26T07:11:25.2307431Z The actual stderr differed from the expected stderr.
2020-03-26T07:11:25.2308076Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-8537/issue-8537.stderr
2020-03-26T07:11:25.2308682Z To update references, rerun the tests and pass the `--bless` flag
2020-03-26T07:11:25.2309248Z To only update this specific test, also pass `--test-args parser/issue-8537.rs`
2020-03-26T07:11:25.2309871Z error: 1 errors occurred comparing output.
2020-03-26T07:11:25.2310189Z status: exit code: 1
2020-03-26T07:11:25.2310189Z status: exit code: 1
2020-03-26T07:11:25.2312136Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-8537.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-8537" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-8537/auxiliary"
2020-03-26T07:11:25.2313664Z ------------------------------------------
2020-03-26T07:11:25.2313837Z 
2020-03-26T07:11:25.2314190Z ------------------------------------------
2020-03-26T07:11:25.2314405Z stderr:
2020-03-26T07:11:25.2314405Z stderr:
2020-03-26T07:11:25.2314773Z ------------------------------------------
2020-03-26T07:11:25.2315242Z error[E0703]: invalid ABI: found `invalid-ab_isize`
2020-03-26T07:11:25.2315978Z    |
2020-03-26T07:11:25.2315978Z    |
2020-03-26T07:11:25.2316353Z LL |   "invalid-ab_isize" //~ ERROR invalid ABI
2020-03-26T07:11:25.2316630Z    |   ^^^^^^^^^^^^^^^^^^ invalid ABI
2020-03-26T07:11:25.2316814Z    |
2020-03-26T07:11:25.2317689Z    = help: valid ABIs: cdecl, stdcall, fastcall, vectorcall, thiscall, aapcs, win64, sysv64, ptx-kernel, msp430-interrupt, x86-interrupt, amdgpu-kernel, efiapi, Rust, C, system, rust-intrinsic, rust-call, platform-intrinsic, unadjusted
2020-03-26T07:11:25.2318449Z error: aborting due to previous error
2020-03-26T07:11:25.2318614Z 
2020-03-26T07:11:25.2319040Z For more information about this error, try `rustc --explain E0703`.
2020-03-26T07:11:25.2319268Z 
---
2020-03-26T07:11:25.2333832Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-26T07:11:25.2334571Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-26T07:11:25.2345715Z 
2020-03-26T07:11:25.2346004Z 
2020-03-26T07:11:25.2349818Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-26T07:11:25.2352754Z 
2020-03-26T07:11:25.2352946Z 
2020-03-26T07:11:25.2356456Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-26T07:11:25.2356779Z Build completed unsuccessfully in 0:56:30
2020-03-26T07:11:25.2356779Z Build completed unsuccessfully in 0:56:30
2020-03-26T07:11:25.2406676Z == clock drift check ==
2020-03-26T07:11:25.2423703Z   local time: Thu Mar 26 07:11:25 UTC 2020
2020-03-26T07:11:25.5355036Z   network time: Thu, 26 Mar 2020 07:11:25 GMT
2020-03-26T07:11:25.5356905Z == end clock drift check ==
2020-03-26T07:11:25.9966403Z 
2020-03-26T07:11:26.0001781Z ##[error]Bash exited with code '1'.
2020-03-26T07:11:26.0014482Z ##[section]Finishing: Run build
2020-03-26T07:11:26.0058319Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70418/merge to s
2020-03-26T07:11:26.0062984Z Task         : Get sources
2020-03-26T07:11:26.0063226Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-26T07:11:26.0063460Z Version      : 1.0.0
2020-03-26T07:11:26.0063628Z Author       : Microsoft
2020-03-26T07:11:26.0063628Z Author       : Microsoft
2020-03-26T07:11:26.0063874Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-26T07:11:26.0064172Z ==============================================================================
2020-03-26T07:11:26.3072594Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-26T07:11:26.3117564Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70418/merge to s
2020-03-26T07:11:26.3199714Z Cleaning up task key
2020-03-26T07:11:26.3200949Z Start cleaning up orphan processes.
2020-03-26T07:11:26.3383416Z Terminate orphan process: pid (4140) (python)
2020-03-26T07:11:26.6492832Z ##[section]Finishing: Finalize Job
