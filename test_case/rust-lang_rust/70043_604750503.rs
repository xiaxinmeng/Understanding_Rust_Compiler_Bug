plain
2020-03-26T23:13:35.9653787Z ========================== Starting Command Output ===========================
2020-03-26T23:13:35.9657144Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ec7e534b-9687-4e48-968f-5d239990c47a.sh
2020-03-26T23:13:35.9657494Z 
2020-03-26T23:13:35.9661290Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-26T23:13:35.9679455Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70043/merge to s
2020-03-26T23:13:35.9682603Z Task         : Get sources
2020-03-26T23:13:35.9682891Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-26T23:13:35.9683173Z Version      : 1.0.0
2020-03-26T23:13:35.9683546Z Author       : Microsoft
---
2020-03-26T23:13:37.2992675Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-26T23:13:37.2999240Z ##[command]git config gc.auto 0
2020-03-26T23:13:37.3002940Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-26T23:13:37.3006542Z ##[command]git config --get-all http.proxy
2020-03-26T23:13:37.3014350Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70043/merge:refs/remotes/pull/70043/merge
---
2020-03-26T23:20:33.7908902Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-26T23:20:34.4649570Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-26T23:20:42.8295824Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-26T23:20:51.0449990Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-26T23:20:54.6882988Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-26T23:20:56.4610440Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-26T23:21:26.7662415Z    Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-03-26T23:21:34.2822864Z    Compiling rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-03-26T23:22:18.9957494Z    Compiling rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
---
2020-03-26T23:40:33.8135258Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-26T23:40:34.9810852Z    Compiling rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-26T23:40:44.8088572Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-26T23:40:55.3785537Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-26T23:41:00.3162168Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-26T23:41:01.7359707Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-26T23:41:41.6386676Z    Compiling rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-03-26T23:41:50.6884504Z    Compiling rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-03-26T23:42:49.7427169Z    Compiling rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
---
2020-03-27T00:03:58.1972453Z .................................................................................................... 1700/9846
2020-03-27T00:04:01.8244834Z .................................................................................................... 1800/9846
2020-03-27T00:04:10.9137337Z .........................................................................................i.......... 1900/9846
2020-03-27T00:04:17.0161870Z .................................................................................................... 2000/9846
2020-03-27T00:04:22.8263453Z ...............................................................................iiiii................ 2100/9846
2020-03-27T00:04:42.3104141Z .................................................................................................... 2300/9846
2020-03-27T00:04:44.3570715Z .................................................................................................... 2400/9846
2020-03-27T00:04:46.6181477Z .................................................................................................... 2500/9846
2020-03-27T00:04:55.0799343Z .................................................................................................... 2600/9846
---
2020-03-27T00:07:32.2482629Z .....................................................i...............i.............................. 5000/9846
2020-03-27T00:07:39.3920872Z .................................................................................................... 5100/9846
2020-03-27T00:07:46.3309837Z ..................................................................................................i. 5200/9846
2020-03-27T00:07:51.1858746Z .................................................................................................... 5300/9846
2020-03-27T00:08:01.1511110Z .................................................................................ii.ii........i...i. 5400/9846
2020-03-27T00:08:08.0170211Z .....................i.............................................................................. 5600/9846
2020-03-27T00:08:14.9855542Z ..........................i......................................................................... 5700/9846
2020-03-27T00:08:22.1746504Z ...........................................ii....................................i.................. 5800/9846
2020-03-27T00:08:28.9233641Z .................................................................................................... 5900/9846
2020-03-27T00:08:28.9233641Z .................................................................................................... 5900/9846
2020-03-27T00:08:33.8809902Z .................................................................................................... 6000/9846
2020-03-27T00:08:42.6555520Z ...........................................................................ii...i..ii...........i... 6100/9846
2020-03-27T00:09:02.0422132Z .................................................................................................... 6300/9846
2020-03-27T00:09:05.7847327Z .................................................................................................... 6400/9846
2020-03-27T00:09:09.1761219Z .................................................................................................... 6500/9846
2020-03-27T00:09:09.1761219Z .................................................................................................... 6500/9846
2020-03-27T00:09:20.4995853Z .....i..ii.......................................................................................... 6600/9846
2020-03-27T00:09:39.0084095Z .................................................................................................... 6800/9846
2020-03-27T00:09:40.9342459Z ....i............................................................................................... 6900/9846
2020-03-27T00:09:42.8513094Z .................................................................................................... 7000/9846
2020-03-27T00:09:44.9987851Z ........................................i........................................................... 7100/9846
---
2020-03-27T00:11:17.2423552Z .................................................................................................... 7800/9846
2020-03-27T00:11:21.6728310Z .................................................................................................... 7900/9846
2020-03-27T00:11:28.2551899Z .................................................................................................i.. 8000/9846
2020-03-27T00:11:35.5230977Z .................................................................................................... 8100/9846
2020-03-27T00:11:42.5948569Z ..............................................iiiiiiiiii.i.......F.................................. 8200/9846
2020-03-27T00:11:51.4534683Z ..........................................................................................i......i.. 8300/9846
2020-03-27T00:12:00.3595192Z .................................................................................................... 8500/9846
2020-03-27T00:12:12.6971197Z .................................................................................................... 8600/9846
2020-03-27T00:12:21.2188574Z .................................................................................................... 8700/9846
2020-03-27T00:12:26.5681923Z .................................................................................................... 8800/9846
---
2020-03-27T00:14:07.9870040Z failures:
2020-03-27T00:14:07.9904606Z 
2020-03-27T00:14:07.9907297Z ---- [ui] ui/save-analysis/issue-68621.rs stdout ----
2020-03-27T00:14:07.9907628Z 
2020-03-27T00:14:07.9907934Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-27T00:14:07.9908249Z status: exit code: 101
2020-03-27T00:14:07.9910343Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/save-analysis/issue-68621.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/save-analysis/issue-68621" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-Zsave-analysis" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/save-analysis/issue-68621/auxiliary"
2020-03-27T00:14:07.9912040Z ------------------------------------------
2020-03-27T00:14:07.9912219Z 
2020-03-27T00:14:07.9912694Z ------------------------------------------
2020-03-27T00:14:07.9912887Z stderr:
---
2020-03-27T00:14:07.9916456Z error: internal compiler error: unexpected panic
2020-03-27T00:14:07.9916640Z 
2020-03-27T00:14:07.9916845Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-27T00:14:07.9917047Z 
2020-03-27T00:14:07.9917692Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-27T00:14:07.9918427Z note: rustc 1.44.0-nightly (7d9bda899 2020-03-26) running on x86_64-unknown-linux-gnu
2020-03-27T00:14:07.9918665Z 
2020-03-27T00:14:07.9918665Z 
2020-03-27T00:14:07.9919301Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z save-analysis -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-27T00:14:07.9920004Z error: aborting due to previous error
2020-03-27T00:14:07.9920226Z 
2020-03-27T00:14:07.9920318Z 
2020-03-27T00:14:07.9920674Z ------------------------------------------
2020-03-27T00:14:07.9920674Z ------------------------------------------
2020-03-27T00:14:07.9920854Z 
2020-03-27T00:14:07.9920944Z 
2020-03-27T00:14:07.9921338Z ---- [ui] ui/type-alias-impl-trait/issue-63279.rs stdout ----
2020-03-27T00:14:07.9921532Z 
2020-03-27T00:14:07.9921797Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-27T00:14:07.9922085Z status: exit code: 101
2020-03-27T00:14:07.9925361Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/issue-63279.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-63279" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-Zsave-analysis" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-63279/auxiliary"
2020-03-27T00:14:07.9926957Z ------------------------------------------
2020-03-27T00:14:07.9927139Z 
2020-03-27T00:14:07.9927483Z ------------------------------------------
2020-03-27T00:14:07.9927674Z stderr:
2020-03-27T00:14:07.9927674Z stderr:
2020-03-27T00:14:07.9928042Z ------------------------------------------
2020-03-27T00:14:07.9928823Z error[E0271]: type mismatch resolving `<[closure@/checkout/src/test/ui/type-alias-impl-trait/issue-63279.rs:8:5: 8:28] as std::ops::FnOnce<()>>::Output == ()`
2020-03-27T00:14:07.9929802Z    |
2020-03-27T00:14:07.9930052Z LL | type Closure = impl FnOnce(); //~ ERROR: type mismatch resolving
2020-03-27T00:14:07.9930391Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected opaque type, found `()`
2020-03-27T00:14:07.9930621Z    |
---
2020-03-27T00:14:07.9933061Z error: internal compiler error: unexpected panic
2020-03-27T00:14:07.9933244Z 
2020-03-27T00:14:07.9933463Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-27T00:14:07.9933649Z 
2020-03-27T00:14:07.9934215Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-27T00:14:07.9934956Z note: rustc 1.44.0-nightly (7d9bda899 2020-03-26) running on x86_64-unknown-linux-gnu
2020-03-27T00:14:07.9935197Z 
2020-03-27T00:14:07.9935197Z 
2020-03-27T00:14:07.9935830Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z save-analysis -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-27T00:14:07.9936378Z error: aborting due to previous error
2020-03-27T00:14:07.9936532Z 
2020-03-27T00:14:07.9936938Z For more information about this error, try `rustc --explain E0271`.
2020-03-27T00:14:07.9937159Z 
2020-03-27T00:14:07.9937159Z 
2020-03-27T00:14:07.9937502Z ------------------------------------------
2020-03-27T00:14:07.9937664Z 
2020-03-27T00:14:07.9937755Z 
2020-03-27T00:14:07.9938413Z ---- [ui] ui/type-alias-impl-trait/issue-65679-inst-opaque-ty-from-val-twice.rs stdout ----
2020-03-27T00:14:07.9938766Z 
2020-03-27T00:14:07.9939332Z error: test compilation failed although it shouldn't!
2020-03-27T00:14:07.9939613Z status: exit code: 101
2020-03-27T00:14:07.9942003Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/issue-65679-inst-opaque-ty-from-val-twice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-65679-inst-opaque-ty-from-val-twice" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-Zsave-analysis" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-65679-inst-opaque-ty-from-val-twice/auxiliary"
2020-03-27T00:14:07.9943930Z ------------------------------------------
2020-03-27T00:14:07.9944107Z 
2020-03-27T00:14:07.9944493Z ------------------------------------------
2020-03-27T00:14:07.9944699Z stderr:
---
2020-03-27T00:14:07.9946792Z error: internal compiler error: unexpected panic
2020-03-27T00:14:07.9946991Z 
2020-03-27T00:14:07.9947210Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-27T00:14:07.9947412Z 
2020-03-27T00:14:07.9948026Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-27T00:14:07.9948801Z note: rustc 1.44.0-nightly (7d9bda899 2020-03-26) running on x86_64-unknown-linux-gnu
2020-03-27T00:14:07.9949073Z 
2020-03-27T00:14:07.9949073Z 
2020-03-27T00:14:07.9949757Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z save-analysis -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-27T00:14:07.9950256Z 
2020-03-27T00:14:07.9950644Z ------------------------------------------
2020-03-27T00:14:07.9950818Z 
2020-03-27T00:14:07.9950915Z 
---
2020-03-27T00:14:07.9953374Z test result: FAILED. 9783 passed; 3 failed; 60 ignored; 0 measured; 0 filtered out
2020-03-27T00:14:07.9953627Z 
2020-03-27T00:14:07.9958726Z 
2020-03-27T00:14:07.9958990Z 
2020-03-27T00:14:07.9963071Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-27T00:14:07.9966752Z 
2020-03-27T00:14:07.9966854Z 
2020-03-27T00:14:07.9967537Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-27T00:14:07.9967963Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-27T00:14:07.9967963Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-27T00:14:07.9968612Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-27T00:14:07.9969010Z Build completed unsuccessfully in 0:56:40
2020-03-27T00:14:08.0020659Z == clock drift check ==
2020-03-27T00:14:08.0041982Z   local time: Fri Mar 27 00:14:08 UTC 2020
2020-03-27T00:14:08.1661663Z   network time: Fri, 27 Mar 2020 00:14:08 GMT
2020-03-27T00:14:08.1672757Z == end clock drift check ==
2020-03-27T00:14:08.6003867Z 
2020-03-27T00:14:08.6039670Z ##[error]Bash exited with code '1'.
2020-03-27T00:14:08.6053061Z ##[section]Finishing: Run build
2020-03-27T00:14:08.6100795Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70043/merge to s
2020-03-27T00:14:08.6105956Z Task         : Get sources
2020-03-27T00:14:08.6106311Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-27T00:14:08.6106648Z Version      : 1.0.0
2020-03-27T00:14:08.6106883Z Author       : Microsoft
2020-03-27T00:14:08.6106883Z Author       : Microsoft
2020-03-27T00:14:08.6107246Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-27T00:14:08.6107682Z ==============================================================================
2020-03-27T00:14:08.9373233Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-27T00:14:08.9417978Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70043/merge to s
2020-03-27T00:14:08.9497666Z Cleaning up task key
2020-03-27T00:14:08.9498833Z Start cleaning up orphan processes.
2020-03-27T00:14:08.9771770Z Terminate orphan process: pid (4200) (python)
2020-03-27T00:14:08.9815574Z ##[section]Finishing: Finalize Job
