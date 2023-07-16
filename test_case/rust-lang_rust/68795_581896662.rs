plain
2020-02-04T11:54:56.4756473Z ========================== Starting Command Output ===========================
2020-02-04T11:54:56.4790444Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/f01af0ac-6f67-4205-8e8c-d33827c7c275.sh
2020-02-04T11:54:56.4790617Z 
2020-02-04T11:54:56.4793361Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-04T11:54:56.4797582Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68795/merge to s
2020-02-04T11:54:56.4798795Z Task         : Get sources
2020-02-04T11:54:56.4798820Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-04T11:54:56.4798856Z Version      : 1.0.0
2020-02-04T11:54:56.4798880Z Author       : Microsoft
---
2020-02-04T11:54:57.2398444Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-04T11:54:57.2466871Z ##[command]git config gc.auto 0
2020-02-04T11:54:57.2535458Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-04T11:54:57.2598217Z ##[command]git config --get-all http.proxy
2020-02-04T11:54:57.2710386Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68795/merge:refs/remotes/pull/68795/merge
---
2020-02-04T12:42:50.0791127Z .................................................................................................... 1700/9582
2020-02-04T12:42:54.7990839Z .................................................................................................... 1800/9582
2020-02-04T12:43:06.4166856Z ............................i....................................................................... 1900/9582
2020-02-04T12:43:13.1322855Z .................................................................................................... 2000/9582
2020-02-04T12:43:26.3756862Z ..................iiiii............................................................................. 2100/9582
2020-02-04T12:43:35.5338595Z .................................................................................................... 2300/9582
2020-02-04T12:43:38.2052883Z .................................................................................................... 2400/9582
2020-02-04T12:43:43.0018367Z .................................................................................................... 2500/9582
2020-02-04T12:44:03.2740802Z .................................................................................................... 2600/9582
---
2020-02-04T12:46:45.0837139Z .............................................................i...............i...................... 4900/9582
2020-02-04T12:46:52.8192294Z .................................................................................................... 5000/9582
2020-02-04T12:47:01.4117509Z .................................................................................................... 5100/9582
2020-02-04T12:47:06.7030193Z ....i............................................................................................... 5200/9582
2020-02-04T12:47:18.3511428Z ..............................................................................ii.ii........i...i.... 5300/9582
2020-02-04T12:47:26.9775273Z ................i................................................................................... 5500/9582
2020-02-04T12:47:36.1862644Z .................................................................................................... 5600/9582
2020-02-04T12:47:42.9666454Z .................................................................i.................................. 5700/9582
2020-02-04T12:47:50.2238174Z .................................................................................................... 5800/9582
2020-02-04T12:47:50.2238174Z .................................................................................................... 5800/9582
2020-02-04T12:47:57.5450896Z .................................................................................................... 5900/9582
2020-02-04T12:48:06.6307450Z ........................................................ii...i..ii...........i...................... 6000/9582
2020-02-04T12:48:28.2393342Z .................................................................................................... 6200/9582
2020-02-04T12:48:35.9439603Z .................................................................................................... 6300/9582
2020-02-04T12:48:35.9439603Z .................................................................................................... 6300/9582
2020-02-04T12:48:41.6315379Z ....................................................................................i..ii........... 6400/9582
2020-02-04T12:49:03.7717664Z .................................................................................................... 6600/9582
2020-02-04T12:49:13.0838260Z .......................................................................i............................ 6700/9582
2020-02-04T12:49:15.2094878Z .................................................................................................... 6800/9582
2020-02-04T12:49:17.3309892Z .........................................................................i.......................... 6900/9582
---
2020-02-04T12:50:56.9608039Z .................................................................................................... 7600/9582
2020-02-04T12:51:01.9209968Z .................................................................................................... 7700/9582
2020-02-04T12:51:09.0512415Z .................................................................................................... 7800/9582
2020-02-04T12:51:17.8377765Z .................................................................................................... 7900/9582
2020-02-04T12:51:25.3756890Z ..................................iiiiiii.i......................................................... 8000/9582
2020-02-04T12:51:39.8459363Z .................................................................................................... 8200/9582
2020-02-04T12:51:48.4000315Z .................................................................................................... 8300/9582
2020-02-04T12:52:01.5221996Z .................................................................................................... 8400/9582
2020-02-04T12:52:09.1728860Z .................................................................................................... 8500/9582
---
2020-02-04T12:53:59.2848922Z ---- [ui] ui/async-await/async-closure.rs stdout ----
2020-02-04T12:53:59.2849006Z 
2020-02-04T12:53:59.2849274Z error: test compilation failed although it shouldn't!
2020-02-04T12:53:59.2849329Z status: exit code: 101
2020-02-04T12:53:59.2850382Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-closure/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-closure/auxiliary"
2020-02-04T12:53:59.2850876Z ------------------------------------------
2020-02-04T12:53:59.2850912Z 
2020-02-04T12:53:59.2851132Z ------------------------------------------
2020-02-04T12:53:59.2851340Z stderr:
2020-02-04T12:53:59.2851340Z stderr:
2020-02-04T12:53:59.2851752Z ------------------------------------------
2020-02-04T12:53:59.2852774Z error: internal compiler error: src/librustc/traits/codegen/mod.rs:57: Encountered error `OutputTypeParameterMismatch(Binder(<[closure@/checkout/src/test/ui/async-await/async-closure.rs:50:5: 53:7] as std::ops::Fn<(u8,)>>), Binder(<[closure@/checkout/src/test/ui/async-await/async-closure.rs:50:5: 53:7] as std::ops::Fn<(u8,)>>), Sorts(ExpectedFound { expected: impl std::future::Future, found: std::future::GenFuture<[static generator@/checkout/src/test/ui/async-await/async-closure.rs:50:31: 53:6 x:u8 {WakeOnceThenComplete, ()}]> }))` selecting `Binder(<[closure@/checkout/src/test/ui/async-await/async-closure.rs:50:5: 53:7] as std::ops::Fn<(u8,)>>)` during codegen
2020-02-04T12:53:59.2853319Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:883:9
2020-02-04T12:53:59.2853399Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-04T12:53:59.2853437Z 
2020-02-04T12:53:59.2853649Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-04T12:53:59.2853649Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-04T12:53:59.2853679Z 
2020-02-04T12:53:59.2854122Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-04T12:53:59.2854446Z note: rustc 1.42.0-nightly (2f5665e83 2020-02-04) running on x86_64-unknown-linux-gnu
2020-02-04T12:53:59.2854501Z 
2020-02-04T12:53:59.2854501Z 
2020-02-04T12:53:59.2854992Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-04T12:53:59.2855083Z error: aborting due to previous error
2020-02-04T12:53:59.2855133Z 
2020-02-04T12:53:59.2855159Z 
2020-02-04T12:53:59.2855380Z ------------------------------------------
---
2020-02-04T12:53:59.2883885Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-04T12:53:59.2884245Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-04T12:53:59.2901558Z 
2020-02-04T12:53:59.2902201Z 
2020-02-04T12:53:59.2905966Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-04T12:53:59.2906657Z 
2020-02-04T12:53:59.2906796Z 
2020-02-04T12:53:59.2914407Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-04T12:53:59.2914886Z Build completed unsuccessfully in 0:54:13
2020-02-04T12:53:59.2914886Z Build completed unsuccessfully in 0:54:13
2020-02-04T12:53:59.2970635Z == clock drift check ==
2020-02-04T12:53:59.2988996Z   local time: Tue Feb  4 12:53:59 UTC 2020
2020-02-04T12:53:59.4611947Z   network time: Tue, 04 Feb 2020 12:53:59 GMT
2020-02-04T12:53:59.4614637Z == end clock drift check ==
2020-02-04T12:53:59.8199898Z 
2020-02-04T12:53:59.8335232Z ##[error]Bash exited with code '1'.
2020-02-04T12:53:59.8348820Z ##[section]Finishing: Run build
2020-02-04T12:53:59.8372205Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68795/merge to s
2020-02-04T12:53:59.8374095Z Task         : Get sources
2020-02-04T12:53:59.8374159Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-04T12:53:59.8374205Z Version      : 1.0.0
2020-02-04T12:53:59.8374263Z Author       : Microsoft
2020-02-04T12:53:59.8374263Z Author       : Microsoft
2020-02-04T12:53:59.8374309Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-04T12:53:59.8374357Z ==============================================================================
2020-02-04T12:54:00.2722404Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-04T12:54:00.2762855Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68795/merge to s
2020-02-04T12:54:00.2926029Z Cleaning up task key
2020-02-04T12:54:00.2926756Z Start cleaning up orphan processes.
2020-02-04T12:54:00.3032879Z Terminate orphan process: pid (4564) (python)
2020-02-04T12:54:00.3315841Z ##[section]Finishing: Finalize Job
