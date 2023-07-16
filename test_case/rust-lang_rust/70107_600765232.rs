plain
2020-03-18T16:27:56.0661268Z ========================== Starting Command Output ===========================
2020-03-18T16:27:56.0663735Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/f12306ff-8661-454c-b26a-3eff9e36ea37.sh
2020-03-18T16:27:56.0664033Z 
2020-03-18T16:27:56.0669579Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-18T16:27:56.0695618Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70107/merge to s
2020-03-18T16:27:56.0705993Z Task         : Get sources
2020-03-18T16:27:56.0706348Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-18T16:27:56.0706689Z Version      : 1.0.0
2020-03-18T16:27:56.0706935Z Author       : Microsoft
---
2020-03-18T16:27:57.0757658Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-18T16:27:57.0777161Z ##[command]git config gc.auto 0
2020-03-18T16:27:57.0782213Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-18T16:27:57.0786989Z ##[command]git config --get-all http.proxy
2020-03-18T16:27:57.0795564Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70107/merge:refs/remotes/pull/70107/merge
---
2020-03-18T17:20:46.3982885Z .................................................................................................... 1700/9796
2020-03-18T17:20:50.3164056Z .................................................................................................... 1800/9796
2020-03-18T17:21:00.4782764Z ..........................................................................i......................... 1900/9796
2020-03-18T17:21:06.1971250Z .................................................................................................... 2000/9796
2020-03-18T17:21:13.3545301Z ................................................................iiiii............................... 2100/9796
2020-03-18T17:21:29.6626176Z .................................................................................................... 2300/9796
2020-03-18T17:21:31.7964322Z .................................................................................................... 2400/9796
2020-03-18T17:21:34.6095833Z .................................................................................................... 2500/9796
2020-03-18T17:21:53.8345600Z .................................................................................................... 2600/9796
---
2020-03-18T17:24:27.9588710Z ....................................i...............i............................................... 5000/9796
2020-03-18T17:24:36.3969812Z .................................................................................................... 5100/9796
2020-03-18T17:24:42.4235688Z ...............................................................................i.................... 5200/9796
2020-03-18T17:24:47.5822600Z .................................................................................................... 5300/9796
2020-03-18T17:24:56.9935490Z ............................................................ii.ii........i...i...................... 5400/9796
2020-03-18T17:25:00.9674488Z ...................................................................................................i 5500/9796
2020-03-18T17:25:13.5326477Z .................................................................................................... 5700/9796
2020-03-18T17:25:19.3310627Z .....................................................i.............................................. 5800/9796
2020-03-18T17:25:25.4961658Z .................................................................................................... 5900/9796
2020-03-18T17:25:34.4424275Z .................................................................................................... 6000/9796
2020-03-18T17:25:34.4424275Z .................................................................................................... 6000/9796
2020-03-18T17:25:40.5604580Z ...............................................ii...i..i.i..........i............................... 6100/9796
2020-03-18T17:25:59.6279437Z .................................................................................................... 6300/9796
2020-03-18T17:26:05.6689144Z .................................................................................................... 6400/9796
2020-03-18T17:26:09.8178788Z ..............................................................................i.ii.................. 6500/9796
2020-03-18T17:26:21.9469211Z .................................................................................................... 6600/9796
---
2020-03-18T17:28:20.3119944Z .................................................................................................... 7800/9796
2020-03-18T17:28:25.3967069Z .................................................................................................... 7900/9796
2020-03-18T17:28:31.0851899Z .............................................................i...................................... 8000/9796
2020-03-18T17:28:40.4269007Z .................................................................................................... 8100/9796
2020-03-18T17:28:45.7577777Z ..........iiiiiiiiii.i.............................................................................. 8200/9796
2020-03-18T17:28:58.5970319Z .................................................................................................... 8400/9796
2020-03-18T17:29:05.9851036Z .................................................................................................... 8500/9796
2020-03-18T17:29:18.5824016Z .................................................................................................... 8600/9796
2020-03-18T17:29:24.6186159Z .................................................................................................... 8700/9796
---
2020-03-18T17:31:08.3173922Z ---- [ui] ui/const-generics/issues/issue-61747.rs stdout ----
2020-03-18T17:31:08.3177566Z 
2020-03-18T17:31:08.3178440Z error: test compilation failed although it shouldn't!
2020-03-18T17:31:08.3178742Z status: exit code: 1
2020-03-18T17:31:08.3180856Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-61747.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-61747" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-61747/auxiliary"
2020-03-18T17:31:08.3182576Z ------------------------------------------
2020-03-18T17:31:08.3182757Z 
2020-03-18T17:31:08.3183141Z ------------------------------------------
2020-03-18T17:31:08.3183352Z stderr:
---
2020-03-18T17:31:08.3186283Z 
2020-03-18T17:31:08.3186507Z error: constant expression depends on a generic parameter
2020-03-18T17:31:08.3187081Z   --> /checkout/src/test/ui/const-generics/issues/issue-61747.rs:9:23
2020-03-18T17:31:08.3187357Z    |
2020-03-18T17:31:08.3187734Z LL |     fn successor() -> Const<{C + 1}> {
2020-03-18T17:31:08.3188227Z    |
2020-03-18T17:31:08.3188482Z    = note: this may fail depending on what value the parameter takes
2020-03-18T17:31:08.3188700Z 
2020-03-18T17:31:08.3188896Z error: aborting due to previous error
---
2020-03-18T17:31:08.3190240Z ---- [ui] ui/const-generics/issues/issue-66205.rs stdout ----
2020-03-18T17:31:08.3190461Z 
2020-03-18T17:31:08.3190857Z error: test compilation failed although it shouldn't!
2020-03-18T17:31:08.3191143Z status: exit code: 101
2020-03-18T17:31:08.3193218Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-66205.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-66205" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-66205/auxiliary"
2020-03-18T17:31:08.3195805Z ------------------------------------------
2020-03-18T17:31:08.3195994Z 
2020-03-18T17:31:08.3197984Z ------------------------------------------
2020-03-18T17:31:08.3198615Z stderr:
2020-03-18T17:31:08.3198615Z stderr:
2020-03-18T17:31:08.3199047Z ------------------------------------------
2020-03-18T17:31:08.3203927Z error: internal compiler error: broken MIR in DefId(0:3 ~ issue_66205[317d]::fact[0]) (CanonicalUserTypeAnnotation { user_ty: Canonical { max_universe: U0, variables: [], value: TypeOf(DefId(0:3 ~ issue_66205[317d]::fact[0]), UserSubsts { substs: [Const { ty: usize, val: Unevaluated(DefId(0:5 ~ issue_66205[317d]::fact[0]::{{constant}}[0]), [Const { ty: usize, val: Param(N/#0) }], None) }], user_self_ty: None }) }, span: /checkout/src/test/ui/const-generics/issues/issue-66205.rs:7:5: 7:22, inferred_ty: fn() {fact::<{ N - 1 }>} }): bad user type AscribeUserType(fn() {fact::<{ N - 1 }>}, DefId(0:3 ~ issue_66205[317d]::fact[0]) UserSubsts { substs: [Const { ty: usize, val: Unevaluated(DefId(0:5 ~ issue_66205[317d]::fact[0]::{{constant}}[0]), [Const { ty: usize, val: Param(N/#0) }], None) }], user_self_ty: None }): NoSolution
2020-03-18T17:31:08.3206294Z 
2020-03-18T17:31:08.3206978Z thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:360:17
2020-03-18T17:31:08.3207736Z 
2020-03-18T17:31:08.3207959Z error: internal compiler error: unexpected panic
2020-03-18T17:31:08.3208161Z 
2020-03-18T17:31:08.3208401Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-18T17:31:08.3208401Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-18T17:31:08.3208607Z 
2020-03-18T17:31:08.3209317Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-18T17:31:08.3209590Z 
2020-03-18T17:31:08.3210107Z note: rustc 1.44.0-nightly (4c2e80e8c 2020-03-18) running on x86_64-unknown-linux-gnu
2020-03-18T17:31:08.3210369Z 
2020-03-18T17:31:08.3211015Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-18T17:31:08.3211630Z 
2020-03-18T17:31:08.3212074Z ------------------------------------------
2020-03-18T17:31:08.3212712Z 
2020-03-18T17:31:08.3212812Z 
---
2020-03-18T17:31:08.3221272Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-18T17:31:08.3225502Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-18T17:31:08.3230144Z 
2020-03-18T17:31:08.3230495Z 
2020-03-18T17:31:08.3235551Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-18T17:31:08.3238654Z 
2020-03-18T17:31:08.3238879Z 
2020-03-18T17:31:08.3244466Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-18T17:31:08.3244898Z Build completed unsuccessfully in 0:59:11
2020-03-18T17:31:08.3244898Z Build completed unsuccessfully in 0:59:11
2020-03-18T17:31:08.3288708Z == clock drift check ==
2020-03-18T17:31:08.3314242Z   local time: Wed Mar 18 17:31:08 UTC 2020
2020-03-18T17:31:08.7591155Z   network time: Wed, 18 Mar 2020 17:31:08 GMT
2020-03-18T17:31:08.7594414Z == end clock drift check ==
2020-03-18T17:31:09.2436726Z 
2020-03-18T17:31:09.2507713Z ##[error]Bash exited with code '1'.
2020-03-18T17:31:09.2525120Z ##[section]Finishing: Run build
2020-03-18T17:31:09.2575839Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70107/merge to s
2020-03-18T17:31:09.2581272Z Task         : Get sources
2020-03-18T17:31:09.2581630Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-18T17:31:09.2581956Z Version      : 1.0.0
2020-03-18T17:31:09.2582207Z Author       : Microsoft
2020-03-18T17:31:09.2582207Z Author       : Microsoft
2020-03-18T17:31:09.2582575Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-18T17:31:09.2582993Z ==============================================================================
2020-03-18T17:31:09.5788930Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-18T17:31:09.5836724Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70107/merge to s
2020-03-18T17:31:09.5943967Z Cleaning up task key
2020-03-18T17:31:09.5945263Z Start cleaning up orphan processes.
2020-03-18T17:31:09.6233980Z Terminate orphan process: pid (3873) (python)
2020-03-18T17:31:09.6276958Z ##[section]Finishing: Finalize Job
