plain
2020-03-18T17:42:01.0656115Z ========================== Starting Command Output ===========================
2020-03-18T17:42:01.0660934Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/0b333c1e-53fb-4908-b9f6-20ccb482a321.sh
2020-03-18T17:42:01.0661493Z 
2020-03-18T17:42:01.0666179Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-18T17:42:01.0687034Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70107/merge to s
2020-03-18T17:42:01.0690740Z Task         : Get sources
2020-03-18T17:42:01.0691027Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-18T17:42:01.0691368Z Version      : 1.0.0
2020-03-18T17:42:01.0691559Z Author       : Microsoft
---
2020-03-18T17:42:02.0610339Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-18T17:42:02.0615671Z ##[command]git config gc.auto 0
2020-03-18T17:42:02.0619860Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-18T17:42:02.0623645Z ##[command]git config --get-all http.proxy
2020-03-18T17:42:02.0630097Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70107/merge:refs/remotes/pull/70107/merge
---
2020-03-18T18:44:18.5227449Z .................................................................................................... 1700/9797
2020-03-18T18:44:22.9673621Z .................................................................................................... 1800/9797
2020-03-18T18:44:34.7817488Z ...........................................................................i........................ 1900/9797
2020-03-18T18:44:41.7062279Z .................................................................................................... 2000/9797
2020-03-18T18:44:50.0736917Z .................................................................iiiii.............................. 2100/9797
2020-03-18T18:45:09.0647163Z .................................................................................................... 2300/9797
2020-03-18T18:45:11.3830890Z .................................................................................................... 2400/9797
2020-03-18T18:45:14.5174727Z .................................................................................................... 2500/9797
2020-03-18T18:45:35.9399935Z .................................................................................................... 2600/9797
---
2020-03-18T18:48:24.7282489Z .....................................i...............i.............................................. 5000/9797
2020-03-18T18:48:34.2291789Z .................................................................................................... 5100/9797
2020-03-18T18:48:40.9886610Z ................................................................................i................... 5200/9797
2020-03-18T18:48:46.7006929Z .................................................................................................... 5300/9797
2020-03-18T18:48:57.6228694Z .............................................................ii.ii........i...i..................... 5400/9797
2020-03-18T18:49:06.3218120Z i................................................................................................... 5600/9797
2020-03-18T18:49:16.9928064Z .................................................................................................... 5700/9797
2020-03-18T18:49:23.7676625Z ......................................................i............................................. 5800/9797
2020-03-18T18:49:30.5381415Z .................................................................................................... 5900/9797
2020-03-18T18:49:30.5381415Z .................................................................................................... 5900/9797
2020-03-18T18:49:40.2351169Z .................................................................................................... 6000/9797
2020-03-18T18:49:47.2297264Z ................................................ii...i..ii...........i.............................. 6100/9797
2020-03-18T18:50:08.6500548Z .................................................................................................... 6300/9797
2020-03-18T18:50:16.3570111Z .................................................................................................... 6400/9797
2020-03-18T18:50:16.3570111Z .................................................................................................... 6400/9797
2020-03-18T18:50:24.9560263Z ..............................................................................i..ii................. 6500/9797
2020-03-18T18:50:48.8328037Z .................................................................................................... 6700/9797
2020-03-18T18:50:58.4679947Z ............................................................................i....................... 6800/9797
2020-03-18T18:51:00.6303673Z .................................................................................................... 6900/9797
2020-03-18T18:51:02.9144857Z .................................................................................................... 7000/9797
---
2020-03-18T18:52:55.0329245Z .................................................................................................... 7800/9797
2020-03-18T18:53:00.7255518Z .................................................................................................... 7900/9797
2020-03-18T18:53:07.2387768Z ..............................................................i..................................... 8000/9797
2020-03-18T18:53:17.8030479Z .................................................................................................... 8100/9797
2020-03-18T18:53:24.3624602Z ...........iiiiiiiiii.i............................................................................. 8200/9797
2020-03-18T18:53:38.1829922Z .................................................................................................... 8400/9797
2020-03-18T18:53:46.5087684Z .................................................................................................... 8500/9797
2020-03-18T18:54:01.3696520Z .................................................................................................... 8600/9797
2020-03-18T18:54:07.7972493Z .................................................................................................... 8700/9797
---
2020-03-18T18:56:08.9049213Z ---- [ui] ui/const-generics/issues/issue-66205.rs stdout ----
2020-03-18T18:56:08.9049422Z 
2020-03-18T18:56:08.9049825Z error: test compilation failed although it shouldn't!
2020-03-18T18:56:08.9050100Z status: exit code: 101
2020-03-18T18:56:08.9052058Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-66205.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-66205" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-66205/auxiliary"
2020-03-18T18:56:08.9053665Z ------------------------------------------
2020-03-18T18:56:08.9053835Z 
2020-03-18T18:56:08.9054201Z ------------------------------------------
2020-03-18T18:56:08.9054397Z stderr:
2020-03-18T18:56:08.9054397Z stderr:
2020-03-18T18:56:08.9054749Z ------------------------------------------
2020-03-18T18:56:08.9061079Z error: internal compiler error: broken MIR in DefId(0:3 ~ issue_66205[317d]::fact[0]) (CanonicalUserTypeAnnotation { user_ty: Canonical { max_universe: U0, variables: [], value: TypeOf(DefId(0:3 ~ issue_66205[317d]::fact[0]), UserSubsts { substs: [Const { ty: usize, val: Unevaluated(DefId(0:5 ~ issue_66205[317d]::fact[0]::{{constant}}[0]), [Const { ty: usize, val: Param(N/#0) }], None) }], user_self_ty: None }) }, span: /checkout/src/test/ui/const-generics/issues/issue-66205.rs:7:5: 7:22, inferred_ty: fn() {fact::<{ N - 1 }>} }): bad user type AscribeUserType(fn() {fact::<{ N - 1 }>}, DefId(0:3 ~ issue_66205[317d]::fact[0]) UserSubsts { substs: [Const { ty: usize, val: Unevaluated(DefId(0:5 ~ issue_66205[317d]::fact[0]::{{constant}}[0]), [Const { ty: usize, val: Param(N/#0) }], None) }], user_self_ty: None }): NoSolution
2020-03-18T18:56:08.9068326Z 
2020-03-18T18:56:08.9069065Z thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:360:17
2020-03-18T18:56:08.9069779Z 
2020-03-18T18:56:08.9069983Z error: internal compiler error: unexpected panic
2020-03-18T18:56:08.9070171Z 
2020-03-18T18:56:08.9070388Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-18T18:56:08.9070388Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-18T18:56:08.9070597Z 
2020-03-18T18:56:08.9071273Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-18T18:56:08.9072038Z note: rustc 1.44.0-nightly (44f5de6f8 2020-03-18) running on x86_64-unknown-linux-gnu
2020-03-18T18:56:08.9072425Z 
2020-03-18T18:56:08.9072425Z 
2020-03-18T18:56:08.9078819Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-18T18:56:08.9079384Z 
2020-03-18T18:56:08.9080053Z ------------------------------------------
2020-03-18T18:56:08.9080223Z 
2020-03-18T18:56:08.9080315Z 
---
2020-03-18T18:56:08.9082605Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-18T18:56:08.9083027Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-18T18:56:08.9083251Z 
2020-03-18T18:56:08.9083353Z 
2020-03-18T18:56:08.9087724Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-18T18:56:08.9090693Z 
2020-03-18T18:56:08.9090789Z 
2020-03-18T18:56:08.9091350Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-18T18:56:08.9091709Z Build completed unsuccessfully in 1:10:01
2020-03-18T18:56:08.9091709Z Build completed unsuccessfully in 1:10:01
2020-03-18T18:56:08.9136544Z == clock drift check ==
2020-03-18T18:56:08.9155473Z   local time: Wed Mar 18 18:56:08 UTC 2020
2020-03-18T18:56:09.2139784Z   network time: Wed, 18 Mar 2020 18:56:09 GMT
2020-03-18T18:56:09.2142468Z == end clock drift check ==
2020-03-18T18:56:09.6868535Z 
2020-03-18T18:56:09.6953530Z ##[error]Bash exited with code '1'.
2020-03-18T18:56:09.6971617Z ##[section]Finishing: Run build
2020-03-18T18:56:09.7028831Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70107/merge to s
2020-03-18T18:56:09.7034115Z Task         : Get sources
2020-03-18T18:56:09.7034470Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-18T18:56:09.7034812Z Version      : 1.0.0
2020-03-18T18:56:09.7035036Z Author       : Microsoft
2020-03-18T18:56:09.7035036Z Author       : Microsoft
2020-03-18T18:56:09.7035402Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-18T18:56:09.7035837Z ==============================================================================
2020-03-18T18:56:10.0625852Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-18T18:56:10.0671963Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70107/merge to s
2020-03-18T18:56:10.0772750Z Cleaning up task key
2020-03-18T18:56:10.0774160Z Start cleaning up orphan processes.
2020-03-18T18:56:10.1121783Z Terminate orphan process: pid (3577) (python)
2020-03-18T18:56:10.1177270Z ##[section]Finishing: Finalize Job
