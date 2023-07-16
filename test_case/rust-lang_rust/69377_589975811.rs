plain
2020-02-22T15:48:26.1043835Z ========================== Starting Command Output ===========================
2020-02-22T15:48:26.1048183Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/45a2541a-d2db-4ece-b9d4-fa1302d67113.sh
2020-02-22T15:48:26.1048645Z 
2020-02-22T15:48:26.1054861Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-22T15:48:26.1077161Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69377/merge to s
2020-02-22T15:48:26.1081004Z Task         : Get sources
2020-02-22T15:48:26.1081269Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-22T15:48:26.1081644Z Version      : 1.0.0
2020-02-22T15:48:26.1081819Z Author       : Microsoft
---
2020-02-22T15:48:27.3743238Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-22T15:48:27.3752997Z ##[command]git config gc.auto 0
2020-02-22T15:48:27.3759271Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-22T15:48:27.3766558Z ##[command]git config --get-all http.proxy
2020-02-22T15:48:27.3778067Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69377/merge:refs/remotes/pull/69377/merge
---
2020-02-22T16:48:30.8720690Z .................................................................................................... 1700/9695
2020-02-22T16:48:35.2857675Z .................................................................................................... 1800/9695
2020-02-22T16:48:46.3662627Z ..........................................i......................................................... 1900/9695
2020-02-22T16:48:53.8849767Z .................................................................................................... 2000/9695
2020-02-22T16:49:07.2697963Z ................................iiiii............................................................... 2100/9695
2020-02-22T16:49:16.9240488Z .................................................................................................... 2300/9695
2020-02-22T16:49:19.1643516Z .................................................................................................... 2400/9695
2020-02-22T16:49:23.3551059Z .................................................................................................... 2500/9695
2020-02-22T16:49:43.1133464Z .................................................................................................... 2600/9695
---
2020-02-22T16:52:15.3896076Z ........i........................................................................................... 5000/9695
2020-02-22T16:52:23.9064421Z .................................................................................................... 5100/9695
2020-02-22T16:52:28.3969228Z ...................................i................................................................ 5200/9695
2020-02-22T16:52:37.7452861Z .................................................................................................... 5300/9695
2020-02-22T16:52:43.2135451Z ...........ii.ii........i...i....................................................................... 5400/9695
2020-02-22T16:52:50.9969265Z .................................................................................................... 5600/9695
2020-02-22T16:53:01.1170505Z .................................................................................................... 5700/9695
2020-02-22T16:53:08.1624672Z ..i................................................................................................. 5800/9695
2020-02-22T16:53:13.3172829Z .................................................................................................... 5900/9695
2020-02-22T16:53:13.3172829Z .................................................................................................... 5900/9695
2020-02-22T16:53:22.7762875Z .............................................................................................ii...i. 6000/9695
2020-02-22T16:53:33.9392318Z .ii...........i..................................................................................... 6100/9695
2020-02-22T16:53:49.4800460Z .................................................................................................... 6300/9695
2020-02-22T16:53:55.6133086Z .................................................................................................... 6400/9695
2020-02-22T16:53:55.6133086Z .................................................................................................... 6400/9695
2020-02-22T16:54:13.0792751Z ........................i..ii....................................................................... 6500/9695
2020-02-22T16:54:33.1921858Z .................................................................................................... 6700/9695
2020-02-22T16:54:35.2737345Z ................i................................................................................... 6800/9695
2020-02-22T16:54:37.4037683Z .................................................................................................... 6900/9695
2020-02-22T16:54:39.7252435Z ......................................i............................................................. 7000/9695
---
2020-02-22T16:56:15.6975584Z .................................................................................................... 7700/9695
2020-02-22T16:56:20.4856461Z .................................................................................................... 7800/9695
2020-02-22T16:56:26.4305312Z ..................................................................................i................. 7900/9695
2020-02-22T16:56:34.9047064Z .................................................................................................... 8000/9695
2020-02-22T16:56:41.3928863Z ...............................iiiiiii.i............................................................ 8100/9695
2020-02-22T16:56:55.0460431Z .................................................................................................... 8300/9695
2020-02-22T16:57:03.1370638Z .................................................................................................... 8400/9695
2020-02-22T16:57:16.0752188Z .................................................................................................... 8500/9695
2020-02-22T16:57:22.6297878Z .................................................................................................... 8600/9695
---
2020-02-22T16:59:11.6416194Z ---- [ui] ui/impl-trait/bound-normalization-pass.rs stdout ----
2020-02-22T16:59:11.6416766Z 
2020-02-22T16:59:11.6417256Z error: test compilation failed although it shouldn't!
2020-02-22T16:59:11.6417536Z status: exit code: 101
2020-02-22T16:59:11.6420060Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/bound-normalization-pass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/bound-normalization-pass" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/bound-normalization-pass/auxiliary"
2020-02-22T16:59:11.6422182Z ------------------------------------------
2020-02-22T16:59:11.6422368Z 
2020-02-22T16:59:11.6422754Z ------------------------------------------
2020-02-22T16:59:11.6422988Z stderr:
---
2020-02-22T16:59:11.6425198Z    |            ^^^^^^^^^^^^^^^^^^^^^^
2020-02-22T16:59:11.6425392Z    |
2020-02-22T16:59:11.6425778Z    = note: `#[warn(incomplete_features)]` on by default
2020-02-22T16:59:11.6425998Z 
2020-02-22T16:59:11.6427303Z error: internal compiler error: src/librustc/ty/context.rs:210: node type T::Assoc (hir_id=HirId { owner: DefIndex(43), local_id: 1 }) with HirId::owner DefId(0:43 ~ bound_normalization_pass[317d]::impl_trait_in_bindings[0]::foo[0]::{{opaque}}[0]) cannot be placed in TypeckTables with local_id_root DefId(0:41 ~ bound_normalization_pass[317d]::impl_trait_in_bindings[0]::foo[0])
2020-02-22T16:59:11.6428838Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:881:9
2020-02-22T16:59:11.6429243Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-22T16:59:11.6429489Z 
2020-02-22T16:59:11.6429733Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-22T16:59:11.6429733Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-22T16:59:11.6429939Z 
2020-02-22T16:59:11.6430646Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-22T16:59:11.6430946Z 
2020-02-22T16:59:11.6431473Z note: rustc 1.43.0-nightly (33b44d242 2020-02-22) running on x86_64-unknown-linux-gnu
2020-02-22T16:59:11.6431734Z 
2020-02-22T16:59:11.6432393Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-22T16:59:11.6433418Z error: aborting due to previous error
2020-02-22T16:59:11.6433594Z 
2020-02-22T16:59:11.6433694Z 
2020-02-22T16:59:11.6434214Z ------------------------------------------
---
2020-02-22T16:59:11.6461198Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-22T16:59:11.6461654Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-22T16:59:11.6473049Z 
2020-02-22T16:59:11.6473488Z 
2020-02-22T16:59:11.6477646Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-22T16:59:11.6480963Z 
2020-02-22T16:59:11.6481067Z 
2020-02-22T16:59:11.6485641Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-22T16:59:11.6486197Z Build completed unsuccessfully in 1:04:52
2020-02-22T16:59:11.6486197Z Build completed unsuccessfully in 1:04:52
2020-02-22T16:59:11.6538259Z == clock drift check ==
2020-02-22T16:59:11.6557906Z   local time: Sat Feb 22 16:59:11 UTC 2020
2020-02-22T16:59:11.9483865Z   network time: Sat, 22 Feb 2020 16:59:11 GMT
2020-02-22T16:59:11.9487087Z == end clock drift check ==
2020-02-22T16:59:12.4227900Z 
2020-02-22T16:59:12.4311837Z ##[error]Bash exited with code '1'.
2020-02-22T16:59:12.4325627Z ##[section]Finishing: Run build
2020-02-22T16:59:12.4372701Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69377/merge to s
2020-02-22T16:59:12.4377548Z Task         : Get sources
2020-02-22T16:59:12.4377877Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-22T16:59:12.4378186Z Version      : 1.0.0
2020-02-22T16:59:12.4378420Z Author       : Microsoft
2020-02-22T16:59:12.4378420Z Author       : Microsoft
2020-02-22T16:59:12.4378930Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-22T16:59:12.4379336Z ==============================================================================
2020-02-22T16:59:12.7525224Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-22T16:59:12.7568102Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69377/merge to s
2020-02-22T16:59:12.7663019Z Cleaning up task key
2020-02-22T16:59:12.7664434Z Start cleaning up orphan processes.
2020-02-22T16:59:12.7829587Z Terminate orphan process: pid (3655) (python)
2020-02-22T16:59:12.8048699Z ##[section]Finishing: Finalize Job
