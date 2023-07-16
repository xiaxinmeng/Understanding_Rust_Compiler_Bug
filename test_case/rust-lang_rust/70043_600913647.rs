plain
2020-03-18T22:44:03.4661325Z ========================== Starting Command Output ===========================
2020-03-18T22:44:03.4667102Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/9e7965b3-cce1-4c9a-a9e4-c26a49e68b0a.sh
2020-03-18T22:44:03.4667606Z 
2020-03-18T22:44:03.4673386Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-18T22:44:03.4692178Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70043/merge to s
2020-03-18T22:44:03.4695701Z Task         : Get sources
2020-03-18T22:44:03.4696003Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-18T22:44:03.4696268Z Version      : 1.0.0
2020-03-18T22:44:03.4696449Z Author       : Microsoft
---
2020-03-18T22:44:04.4561012Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-18T22:44:04.4574272Z ##[command]git config gc.auto 0
2020-03-18T22:44:04.4580931Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-18T22:44:04.4587612Z ##[command]git config --get-all http.proxy
2020-03-18T22:44:04.4598848Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70043/merge:refs/remotes/pull/70043/merge
---
2020-03-18T23:40:25.7895771Z .................................................................................................... 1700/9796
2020-03-18T23:40:29.6803149Z .................................................................................................... 1800/9796
2020-03-18T23:40:39.9226057Z ..........................................................................i......................... 1900/9796
2020-03-18T23:40:45.7652711Z .................................................................................................... 2000/9796
2020-03-18T23:40:53.1133517Z ................................................................iiiii............................... 2100/9796
2020-03-18T23:41:09.3693091Z .................................................................................................... 2300/9796
2020-03-18T23:41:11.3649876Z .................................................................................................... 2400/9796
2020-03-18T23:41:14.0259571Z .................................................................................................... 2500/9796
2020-03-18T23:41:32.5345988Z .................................................................................................... 2600/9796
---
2020-03-18T23:44:02.3424780Z ....................................i...............i............................................... 5000/9796
2020-03-18T23:44:10.9538126Z .................................................................................................... 5100/9796
2020-03-18T23:44:16.9718564Z ...............................................................................i.................... 5200/9796
2020-03-18T23:44:22.1886481Z .................................................................................................... 5300/9796
2020-03-18T23:44:32.1918166Z ............................................................ii.ii........i...i...................... 5400/9796
2020-03-18T23:44:36.5331004Z ...................................................................................................i 5500/9796
2020-03-18T23:44:49.5902076Z .................................................................................................... 5700/9796
2020-03-18T23:44:55.7981724Z .....................................................i.............................................. 5800/9796
2020-03-18T23:45:02.3300913Z .................................................................................................... 5900/9796
2020-03-18T23:45:11.6793893Z .................................................................................................... 6000/9796
2020-03-18T23:45:11.6793893Z .................................................................................................... 6000/9796
2020-03-18T23:45:17.5902320Z ...............................................ii...i..ii...........i............................... 6100/9796
2020-03-18T23:45:36.4918842Z .................................................................................................... 6300/9796
2020-03-18T23:45:43.2244124Z .................................................................................................... 6400/9796
2020-03-18T23:45:43.2244124Z .................................................................................................... 6400/9796
2020-03-18T23:45:51.1751232Z .............................................................................i..ii.................. 6500/9796
2020-03-18T23:46:11.9803796Z .................................................................................................... 6700/9796
2020-03-18T23:46:20.4819314Z ...........................................................................i........................ 6800/9796
2020-03-18T23:46:22.4547754Z .................................................................................................... 6900/9796
2020-03-18T23:46:24.4956571Z .................................................................................................... 7000/9796
---
2020-03-18T23:48:00.0383113Z .................................................................................................... 7800/9796
2020-03-18T23:48:04.9965095Z .................................................................................................... 7900/9796
2020-03-18T23:48:10.8085207Z .............................................................i...................................... 8000/9796
2020-03-18T23:48:19.9851296Z .................................................................................................... 8100/9796
2020-03-18T23:48:25.1584223Z ..........iiiiiiiiii.i........F..................................................................... 8200/9796
2020-03-18T23:48:37.8255126Z .................................................................................................... 8400/9796
2020-03-18T23:48:45.1077753Z .................................................................................................... 8500/9796
2020-03-18T23:48:57.5398192Z .................................................................................................... 8600/9796
2020-03-18T23:49:03.6455042Z .................................................................................................... 8700/9796
---
2020-03-18T23:50:49.9523801Z failures:
2020-03-18T23:50:49.9549243Z 
2020-03-18T23:50:49.9550084Z ---- [ui] ui/save-analysis/issue-68621.rs stdout ----
2020-03-18T23:50:49.9550506Z 
2020-03-18T23:50:49.9550942Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-18T23:50:49.9551423Z status: exit code: 101
2020-03-18T23:50:49.9555562Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/save-analysis/issue-68621.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/save-analysis/issue-68621" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-Zsave-analysis" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/save-analysis/issue-68621/auxiliary"
2020-03-18T23:50:49.9557479Z ------------------------------------------
2020-03-18T23:50:49.9557658Z 
2020-03-18T23:50:49.9558029Z ------------------------------------------
2020-03-18T23:50:49.9558252Z stderr:
---
2020-03-18T23:50:49.9562592Z error: internal compiler error: unexpected panic
2020-03-18T23:50:49.9562787Z 
2020-03-18T23:50:49.9563023Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-18T23:50:49.9563222Z 
2020-03-18T23:50:49.9563923Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-18T23:50:49.9564719Z note: rustc 1.44.0-nightly (ad181c37d 2020-03-18) running on x86_64-unknown-linux-gnu
2020-03-18T23:50:49.9565341Z 
2020-03-18T23:50:49.9565341Z 
2020-03-18T23:50:49.9565997Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z save-analysis -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-18T23:50:49.9566562Z error: aborting due to previous error
2020-03-18T23:50:49.9566833Z 
2020-03-18T23:50:49.9566923Z 
2020-03-18T23:50:49.9567280Z ------------------------------------------
2020-03-18T23:50:49.9567280Z ------------------------------------------
2020-03-18T23:50:49.9567442Z 
2020-03-18T23:50:49.9567531Z 
2020-03-18T23:50:49.9567920Z ---- [ui] ui/type-alias-impl-trait/issue-63279.rs stdout ----
2020-03-18T23:50:49.9568220Z 
2020-03-18T23:50:49.9568467Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-03-18T23:50:49.9568733Z status: exit code: 101
2020-03-18T23:50:49.9570586Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/issue-63279.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-63279" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-Zsave-analysis" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-63279/auxiliary"
2020-03-18T23:50:49.9572068Z ------------------------------------------
2020-03-18T23:50:49.9572220Z 
2020-03-18T23:50:49.9572534Z ------------------------------------------
2020-03-18T23:50:49.9572709Z stderr:
2020-03-18T23:50:49.9572709Z stderr:
2020-03-18T23:50:49.9573052Z ------------------------------------------
2020-03-18T23:50:49.9573751Z error[E0271]: type mismatch resolving `<[closure@/checkout/src/test/ui/type-alias-impl-trait/issue-63279.rs:8:5: 8:28] as std::ops::FnOnce<()>>::Output == ()`
2020-03-18T23:50:49.9574664Z    |
2020-03-18T23:50:49.9575018Z LL | type Closure = impl FnOnce(); //~ ERROR: type mismatch resolving
2020-03-18T23:50:49.9575383Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected opaque type, found `()`
2020-03-18T23:50:49.9575604Z    |
---
2020-03-18T23:50:49.9578262Z error: internal compiler error: unexpected panic
2020-03-18T23:50:49.9578475Z 
2020-03-18T23:50:49.9578684Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-18T23:50:49.9578856Z 
2020-03-18T23:50:49.9579418Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-18T23:50:49.9580106Z note: rustc 1.44.0-nightly (ad181c37d 2020-03-18) running on x86_64-unknown-linux-gnu
2020-03-18T23:50:49.9580326Z 
2020-03-18T23:50:49.9580326Z 
2020-03-18T23:50:49.9580938Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z save-analysis -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-18T23:50:49.9581432Z error: aborting due to previous error
2020-03-18T23:50:49.9581576Z 
2020-03-18T23:50:49.9581973Z For more information about this error, try `rustc --explain E0271`.
2020-03-18T23:50:49.9582162Z 
2020-03-18T23:50:49.9582162Z 
2020-03-18T23:50:49.9582484Z ------------------------------------------
2020-03-18T23:50:49.9582634Z 
2020-03-18T23:50:49.9582718Z 
2020-03-18T23:50:49.9583176Z ---- [ui] ui/type-alias-impl-trait/issue-65679-inst-opaque-ty-from-val-twice.rs stdout ----
2020-03-18T23:50:49.9583401Z 
2020-03-18T23:50:49.9583742Z error: test compilation failed although it shouldn't!
2020-03-18T23:50:49.9583987Z status: exit code: 101
2020-03-18T23:50:49.9586223Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/issue-65679-inst-opaque-ty-from-val-twice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-65679-inst-opaque-ty-from-val-twice" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-Zsave-analysis" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-65679-inst-opaque-ty-from-val-twice/auxiliary"
2020-03-18T23:50:49.9588206Z ------------------------------------------
2020-03-18T23:50:49.9588381Z 
2020-03-18T23:50:49.9588763Z ------------------------------------------
2020-03-18T23:50:49.9588966Z stderr:
---
2020-03-18T23:50:49.9591077Z error: internal compiler error: unexpected panic
2020-03-18T23:50:49.9591272Z 
2020-03-18T23:50:49.9591491Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-18T23:50:49.9591688Z 
2020-03-18T23:50:49.9598463Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-18T23:50:49.9599462Z note: rustc 1.44.0-nightly (ad181c37d 2020-03-18) running on x86_64-unknown-linux-gnu
2020-03-18T23:50:49.9599709Z 
2020-03-18T23:50:49.9599709Z 
2020-03-18T23:50:49.9600308Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -Z save-analysis -C prefer-dynamic -C rpath -C debuginfo=0
2020-03-18T23:50:49.9600752Z 
2020-03-18T23:50:49.9601076Z ------------------------------------------
2020-03-18T23:50:49.9601276Z 
2020-03-18T23:50:49.9601361Z 
---
2020-03-18T23:50:49.9604417Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-18T23:50:49.9604784Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-18T23:50:49.9661289Z 
2020-03-18T23:50:49.9661474Z 
2020-03-18T23:50:49.9667407Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-18T23:50:49.9669971Z 
2020-03-18T23:50:49.9670060Z 
2020-03-18T23:50:49.9670548Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-18T23:50:49.9670902Z Build completed unsuccessfully in 1:01:36
2020-03-18T23:50:49.9670902Z Build completed unsuccessfully in 1:01:36
2020-03-18T23:50:49.9671500Z == clock drift check ==
2020-03-18T23:50:49.9686289Z   local time: Wed Mar 18 23:50:49 UTC 2020
2020-03-18T23:50:50.1371815Z   network time: Wed, 18 Mar 2020 23:50:50 GMT
2020-03-18T23:50:50.1373846Z == end clock drift check ==
2020-03-18T23:50:50.5943944Z 
2020-03-18T23:50:50.6024702Z ##[error]Bash exited with code '1'.
2020-03-18T23:50:50.6040731Z ##[section]Finishing: Run build
2020-03-18T23:50:50.6088013Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70043/merge to s
2020-03-18T23:50:50.6092489Z Task         : Get sources
2020-03-18T23:50:50.6092806Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-18T23:50:50.6093082Z Version      : 1.0.0
2020-03-18T23:50:50.6093274Z Author       : Microsoft
2020-03-18T23:50:50.6093274Z Author       : Microsoft
2020-03-18T23:50:50.6093600Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-18T23:50:50.6093953Z ==============================================================================
2020-03-18T23:50:50.9157619Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-18T23:50:50.9197480Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70043/merge to s
2020-03-18T23:50:50.9277051Z Cleaning up task key
2020-03-18T23:50:50.9278296Z Start cleaning up orphan processes.
2020-03-18T23:50:50.9556086Z Terminate orphan process: pid (4484) (python)
2020-03-18T23:50:50.9602736Z ##[section]Finishing: Finalize Job
