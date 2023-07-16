plain
2020-02-19T05:04:24.5104204Z ========================== Starting Command Output ===========================
2020-02-19T05:04:24.5106760Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ff622616-d5c4-4e72-ba7a-ca54359cbd47.sh
2020-02-19T05:04:24.5106795Z 
2020-02-19T05:04:24.5109436Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-19T05:04:24.5114526Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68847/merge to s
2020-02-19T05:04:24.5116166Z Task         : Get sources
2020-02-19T05:04:24.5116195Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-19T05:04:24.5116239Z Version      : 1.0.0
2020-02-19T05:04:24.5116267Z Author       : Microsoft
---
2020-02-19T05:04:25.5540912Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-19T05:04:25.5637773Z ##[command]git config gc.auto 0
2020-02-19T05:04:25.5718557Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-19T05:04:25.5774648Z ##[command]git config --get-all http.proxy
2020-02-19T05:04:25.5930823Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68847/merge:refs/remotes/pull/68847/merge
---
2020-02-19T05:55:47.9771038Z .................................................................................................... 1700/9682
2020-02-19T05:55:51.9412071Z .................................................................................................... 1800/9682
2020-02-19T05:56:01.6632080Z ...................................i................................................................ 1900/9682
2020-02-19T05:56:07.9710408Z .................................................................................................... 2000/9682
2020-02-19T05:56:20.2059150Z .........................iiiii...................................................................... 2100/9682
2020-02-19T05:56:28.2312242Z .................................................................................................... 2300/9682
2020-02-19T05:56:30.1761019Z .................................................................................................... 2400/9682
2020-02-19T05:56:33.8329018Z .................................................................................................... 2500/9682
2020-02-19T05:56:50.9151424Z .................................................................................................... 2600/9682
---
2020-02-19T05:59:06.3588911Z i................................................................................................... 5000/9682
2020-02-19T05:59:13.9320900Z .................................................................................................... 5100/9682
2020-02-19T05:59:18.0567493Z ...........................i........................................................................ 5200/9682
2020-02-19T05:59:27.1425914Z .................................................................................................... 5300/9682
2020-02-19T05:59:31.8599767Z ..ii.ii........i...i................................................................................ 5400/9682
2020-02-19T05:59:40.6506275Z .................................................................................................... 5600/9682
2020-02-19T05:59:49.3488807Z ............................................................................................i....... 5700/9682
2020-02-19T05:59:55.9943993Z .................................................................................................... 5800/9682
2020-02-19T06:00:00.1982038Z ..........................................................................................i......... 5900/9682
2020-02-19T06:00:00.1982038Z ..........................................................................................i......... 5900/9682
2020-02-19T06:00:08.8344233Z ...................................................................................ii...i..ii....... 6000/9682
2020-02-19T06:00:28.4158523Z .................................................................................................... 6200/9682
2020-02-19T06:00:34.6539817Z .................................................................................................... 6300/9682
2020-02-19T06:00:40.9875123Z .................................................................................................... 6400/9682
2020-02-19T06:00:40.9875123Z .................................................................................................... 6400/9682
2020-02-19T06:00:54.3179584Z ...........i..ii.................................................................................... 6500/9682
2020-02-19T06:01:10.5670791Z .................................................................................................... 6700/9682
2020-02-19T06:01:12.4287633Z ...i................................................................................................ 6800/9682
2020-02-19T06:01:14.3352193Z .................................................................................................... 6900/9682
2020-02-19T06:01:16.3085913Z .........................i.......................................................................... 7000/9682
---
2020-02-19T06:02:44.3719424Z .................................................................................................... 7700/9682
2020-02-19T06:02:49.1536277Z .................................................................................................... 7800/9682
2020-02-19T06:02:54.2924616Z .....................................................................i...F.......................... 7900/9682
2020-02-19T06:03:02.8919610Z .................................................................................................... 8000/9682
2020-02-19T06:03:07.9561742Z ..................iiiiiii.i......................................................................... 8100/9682
2020-02-19T06:03:20.0950465Z .................................................................................................... 8300/9682
2020-02-19T06:03:28.0997186Z .................................................................................................... 8400/9682
2020-02-19T06:03:40.3943821Z .................................................................................................... 8500/9682
2020-02-19T06:03:46.5597018Z .................................................................................................... 8600/9682
---
2020-02-19T06:05:28.2027549Z 4 LL |     fn foo() { non_const() }
2020-02-19T06:05:28.2027903Z -    |     ^^^^^^^^^^^^^^^^^^^^^^^^
2020-02-19T06:05:28.2028063Z +    |                ^^^^^^^^^^^
2020-02-19T06:05:28.2028190Z 6    |
2020-02-19T06:05:28.2028708Z 7    = note: see issue #57563 <***/issues/57563> for more information
2020-02-19T06:05:28.2028897Z 8    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-19T06:05:28.2029144Z 
2020-02-19T06:05:28.2029271Z The actual stderr differed from the expected stderr.
2020-02-19T06:05:28.2029744Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/const-check-fns-in-const-impl/const-check-fns-in-const-impl.stderr
2020-02-19T06:05:28.2029744Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/const-check-fns-in-const-impl/const-check-fns-in-const-impl.stderr
2020-02-19T06:05:28.2030118Z To update references, rerun the tests and pass the `--bless` flag
2020-02-19T06:05:28.2030548Z To only update this specific test, also pass `--test-args rfc-2632-const-trait-impl/const-check-fns-in-const-impl.rs`
2020-02-19T06:05:28.2030831Z error: 1 errors occurred comparing output.
2020-02-19T06:05:28.2031183Z status: exit code: 1
2020-02-19T06:05:28.2031183Z status: exit code: 1
2020-02-19T06:05:28.2032291Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/const-check-fns-in-const-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/const-check-fns-in-const-impl" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/const-check-fns-in-const-impl/auxiliary"
2020-02-19T06:05:28.2033223Z ------------------------------------------
2020-02-19T06:05:28.2033392Z 
2020-02-19T06:05:28.2033712Z ------------------------------------------
2020-02-19T06:05:28.2033884Z stderr:
2020-02-19T06:05:28.2033884Z stderr:
2020-02-19T06:05:28.2034215Z ------------------------------------------
2020-02-19T06:05:28.2034395Z error[E0723]: can only call other `const fn` within a `const fn`, but `const non_const` is not stable as `const fn`
2020-02-19T06:05:28.2034772Z   --> /checkout/src/test/ui/rfc-2632-const-trait-impl/const-check-fns-in-const-impl.rs:12:16
2020-02-19T06:05:28.2034963Z    |
2020-02-19T06:05:28.2035092Z LL |     fn foo() { non_const() }
2020-02-19T06:05:28.2035233Z    |                ^^^^^^^^^^^
2020-02-19T06:05:28.2035360Z    |
2020-02-19T06:05:28.2035941Z    = note: see issue #57563 <***/issues/57563> for more information
2020-02-19T06:05:28.2036139Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2020-02-19T06:05:28.2036390Z error: aborting due to previous error
2020-02-19T06:05:28.2036677Z 
2020-02-19T06:05:28.2037024Z For more information about this error, try `rustc --explain E0723`.
2020-02-19T06:05:28.2037170Z 
---
2020-02-19T06:05:28.2042168Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-19T06:05:28.2042418Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-19T06:05:28.2052519Z 
2020-02-19T06:05:28.2052742Z 
2020-02-19T06:05:28.2057951Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-19T06:05:28.2059280Z 
2020-02-19T06:05:28.2059309Z 
2020-02-19T06:05:28.2090894Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-19T06:05:28.2091004Z Build completed unsuccessfully in 0:55:29
2020-02-19T06:05:28.2091004Z Build completed unsuccessfully in 0:55:29
2020-02-19T06:05:28.2124900Z == clock drift check ==
2020-02-19T06:05:28.2152430Z   local time: Wed Feb 19 06:05:28 UTC 2020
2020-02-19T06:05:28.5111936Z   network time: Wed, 19 Feb 2020 06:05:28 GMT
2020-02-19T06:05:28.5112035Z == end clock drift check ==
2020-02-19T06:05:28.9678273Z 
2020-02-19T06:05:28.9768522Z ##[error]Bash exited with code '1'.
2020-02-19T06:05:28.9780643Z ##[section]Finishing: Run build
2020-02-19T06:05:28.9800359Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68847/merge to s
2020-02-19T06:05:28.9802098Z Task         : Get sources
2020-02-19T06:05:28.9802135Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-19T06:05:28.9802204Z Version      : 1.0.0
2020-02-19T06:05:28.9802237Z Author       : Microsoft
2020-02-19T06:05:28.9802237Z Author       : Microsoft
2020-02-19T06:05:28.9802273Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-19T06:05:28.9802330Z ==============================================================================
2020-02-19T06:05:29.3585942Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-19T06:05:29.3622562Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68847/merge to s
2020-02-19T06:05:29.3716802Z Cleaning up task key
2020-02-19T06:05:29.3717427Z Start cleaning up orphan processes.
2020-02-19T06:05:29.3806818Z Terminate orphan process: pid (3582) (python)
2020-02-19T06:05:29.3993593Z ##[section]Finishing: Finalize Job
