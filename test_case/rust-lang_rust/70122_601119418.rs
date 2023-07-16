plain
2020-03-19T09:53:36.0003917Z ========================== Starting Command Output ===========================
2020-03-19T09:53:36.0009234Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/e109ca65-1b9a-4e92-988e-5a32d42787c2.sh
2020-03-19T09:53:36.0009776Z 
2020-03-19T09:53:36.0015338Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-19T09:53:36.0036077Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70122/merge to s
2020-03-19T09:53:36.0039835Z Task         : Get sources
2020-03-19T09:53:36.0040183Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-19T09:53:36.0040499Z Version      : 1.0.0
2020-03-19T09:53:36.0040717Z Author       : Microsoft
---
2020-03-19T09:53:37.1359823Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-19T09:53:37.1380264Z ##[command]git config gc.auto 0
2020-03-19T09:53:37.1385490Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-19T09:53:37.1389323Z ##[command]git config --get-all http.proxy
2020-03-19T09:53:37.1398246Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70122/merge:refs/remotes/pull/70122/merge
---
2020-03-19T10:53:32.4278128Z .................................................................................................... 1700/9805
2020-03-19T10:53:36.7389482Z .................................................................................................... 1800/9805
2020-03-19T10:53:48.1232405Z ...........................................................................i........................ 1900/9805
2020-03-19T10:53:55.0402533Z .................................................................................................... 2000/9805
2020-03-19T10:54:03.4924252Z .................................................................iiiii.............................. 2100/9805
2020-03-19T10:54:22.4162866Z .................................................................................................... 2300/9805
2020-03-19T10:54:24.6698137Z .................................................................................................... 2400/9805
2020-03-19T10:54:27.6325599Z .................................................................................................... 2500/9805
2020-03-19T10:54:49.1243443Z .................................................................................................... 2600/9805
---
2020-03-19T10:57:31.5606170Z .......................................i...............i............................................ 5000/9805
2020-03-19T10:57:41.1602922Z .................................................................................................... 5100/9805
2020-03-19T10:57:48.4091260Z ..................................................................................i................. 5200/9805
2020-03-19T10:57:54.5085214Z .................................................................................................... 5300/9805
2020-03-19T10:58:05.4858120Z ...............................................................ii.ii........i...i................... 5400/9805
2020-03-19T10:58:14.0669699Z ..i................................................................................................. 5600/9805
2020-03-19T10:58:24.0941769Z .......i............................................................................................ 5700/9805
2020-03-19T10:58:30.5174974Z ..........................................................i......................................... 5800/9805
2020-03-19T10:58:37.4999822Z .................................................................................................... 5900/9805
2020-03-19T10:58:37.4999822Z .................................................................................................... 5900/9805
2020-03-19T10:58:45.4572578Z .................................................................................................... 6000/9805
2020-03-19T10:58:53.7521658Z ....................................................ii...i..ii...........i.......................... 6100/9805
2020-03-19T10:59:14.3505763Z .................................................................................................... 6300/9805
2020-03-19T10:59:21.6744961Z .................................................................................................... 6400/9805
2020-03-19T10:59:21.6744961Z .................................................................................................... 6400/9805
2020-03-19T10:59:29.3623642Z ..................................................................................i..ii............. 6500/9805
2020-03-19T10:59:52.9040887Z .................................................................................................... 6700/9805
2020-03-19T11:00:02.9054097Z .................................................................................i.................. 6800/9805
2020-03-19T11:00:05.0500112Z .................................................................................................... 6900/9805
2020-03-19T11:00:07.1771472Z .................................................................................................... 7000/9805
---
2020-03-19T11:01:54.6887121Z .................................................................................................... 7800/9805
2020-03-19T11:02:00.2135788Z .................................................................................................... 7900/9805
2020-03-19T11:02:06.3561323Z ....................................................................i............................... 8000/9805
2020-03-19T11:02:16.8354781Z .................................................................................................... 8100/9805
2020-03-19T11:02:22.6791762Z .................iiiiiiiiii.i....................................................................... 8200/9805
2020-03-19T11:02:37.6752478Z .................................................................................................... 8400/9805
2020-03-19T11:02:43.9880630Z .................................................................................................... 8500/9805
2020-03-19T11:02:59.8613415Z .................................................................................................... 8600/9805
2020-03-19T11:03:07.2054996Z .................................................................................................... 8700/9805
---
2020-03-19T11:05:10.4313531Z 1 error[E0658]: const generics are unstable
2020-03-19T11:05:10.4314348Z -   --> $DIR/feature-gate-const_generics-fn.rs:1:14
2020-03-19T11:05:10.4315225Z +   --> $DIR/feature-gate-const_generics-fn.rs:3:14
2020-03-19T11:05:10.4315724Z 3    |
2020-03-19T11:05:10.4316145Z 4 LL | fn foo<const X: ()>() {}
2020-03-19T11:05:10.4317001Z 
2020-03-19T11:05:10.4317421Z 
2020-03-19T11:05:10.4317833Z The actual stderr differed from the expected stderr.
2020-03-19T11:05:10.4317833Z The actual stderr differed from the expected stderr.
2020-03-19T11:05:10.4318898Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-const_generics-fn/feature-gate-const_generics-fn.stderr
2020-03-19T11:05:10.4322829Z To update references, rerun the tests and pass the `--bless` flag
2020-03-19T11:05:10.4323619Z To only update this specific test, also pass `--test-args feature-gates/feature-gate-const_generics-fn.rs`
2020-03-19T11:05:10.4324114Z error: 1 errors occurred comparing output.
2020-03-19T11:05:10.4324351Z status: exit code: 1
2020-03-19T11:05:10.4324351Z status: exit code: 1
2020-03-19T11:05:10.4326503Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-const_generics-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-const_generics-fn" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-const_generics-fn/auxiliary"
2020-03-19T11:05:10.4331266Z ------------------------------------------
2020-03-19T11:05:10.4331448Z 
2020-03-19T11:05:10.4331828Z ------------------------------------------
2020-03-19T11:05:10.4332066Z stderr:
2020-03-19T11:05:10.4332066Z stderr:
2020-03-19T11:05:10.4332453Z ------------------------------------------
2020-03-19T11:05:10.4332721Z error[E0658]: const generics are unstable
2020-03-19T11:05:10.4333317Z   --> /checkout/src/test/ui/feature-gates/feature-gate-const_generics-fn.rs:3:14
2020-03-19T11:05:10.4333600Z    |
2020-03-19T11:05:10.4333850Z LL | fn foo<const X: ()>() {} //~ ERROR const generics are unstable
2020-03-19T11:05:10.4334295Z    |
2020-03-19T11:05:10.4334295Z    |
2020-03-19T11:05:10.4334957Z    = note: see issue #44580 <***/issues/44580> for more information
2020-03-19T11:05:10.4336927Z    = help: add `#![feature(const_generics)]` to the crate attributes to enable
2020-03-19T11:05:10.4337357Z error: aborting due to previous error
2020-03-19T11:05:10.4337543Z 
2020-03-19T11:05:10.4338104Z For more information about this error, try `rustc --explain E0658`.
2020-03-19T11:05:10.4338323Z 
---
2020-03-19T11:05:10.4340921Z 1 error[E0658]: const generics are unstable
2020-03-19T11:05:10.4342762Z -   --> $DIR/feature-gate-const_generics-struct.rs:1:18
2020-03-19T11:05:10.4343384Z +   --> $DIR/feature-gate-const_generics-struct.rs:3:18
2020-03-19T11:05:10.4343629Z 3    |
2020-03-19T11:05:10.4344003Z 4 LL | struct Foo<const X: usize>([(); X]);
2020-03-19T11:05:10.4344420Z 
2020-03-19T11:05:10.4344516Z 
2020-03-19T11:05:10.4344719Z The actual stderr differed from the expected stderr.
2020-03-19T11:05:10.4344719Z The actual stderr differed from the expected stderr.
2020-03-19T11:05:10.4345577Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-const_generics-struct/feature-gate-const_generics-struct.stderr
2020-03-19T11:05:10.4346324Z To update references, rerun the tests and pass the `--bless` flag
2020-03-19T11:05:10.4346996Z To only update this specific test, also pass `--test-args feature-gates/feature-gate-const_generics-struct.rs`
2020-03-19T11:05:10.4347491Z error: 1 errors occurred comparing output.
2020-03-19T11:05:10.4347725Z status: exit code: 1
2020-03-19T11:05:10.4347725Z status: exit code: 1
2020-03-19T11:05:10.4349887Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-const_generics-struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-const_generics-struct" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-const_generics-struct/auxiliary"
2020-03-19T11:05:10.4351632Z ------------------------------------------
2020-03-19T11:05:10.4351807Z 
2020-03-19T11:05:10.4352299Z ------------------------------------------
2020-03-19T11:05:10.4352509Z stderr:
2020-03-19T11:05:10.4352509Z stderr:
2020-03-19T11:05:10.4352874Z ------------------------------------------
2020-03-19T11:05:10.4353132Z error[E0658]: const generics are unstable
2020-03-19T11:05:10.4353714Z   --> /checkout/src/test/ui/feature-gates/feature-gate-const_generics-struct.rs:3:18
2020-03-19T11:05:10.4353996Z    |
2020-03-19T11:05:10.4354260Z LL | struct Foo<const X: usize>([(); X]); //~ ERROR const generics are unstable
2020-03-19T11:05:10.4354719Z    |
2020-03-19T11:05:10.4354719Z    |
2020-03-19T11:05:10.4355285Z    = note: see issue #44580 <***/issues/44580> for more information
2020-03-19T11:05:10.4355674Z    = help: add `#![feature(const_generics)]` to the crate attributes to enable
2020-03-19T11:05:10.4356084Z error: aborting due to previous error
2020-03-19T11:05:10.4356241Z 
2020-03-19T11:05:10.4356698Z For more information about this error, try `rustc --explain E0658`.
2020-03-19T11:05:10.4357030Z 
---
2020-03-19T11:05:10.4360483Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-19T11:05:10.4361014Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-19T11:05:10.4361377Z 
2020-03-19T11:05:10.4361485Z 
2020-03-19T11:05:10.4365243Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-19T11:05:10.4368010Z 
2020-03-19T11:05:10.4368107Z 
2020-03-19T11:05:10.4368662Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-19T11:05:10.4369053Z Build completed unsuccessfully in 1:06:37
2020-03-19T11:05:10.4369053Z Build completed unsuccessfully in 1:06:37
2020-03-19T11:05:10.4400730Z == clock drift check ==
2020-03-19T11:05:10.4418846Z   local time: Thu Mar 19 11:05:10 UTC 2020
2020-03-19T11:05:10.6096603Z   network time: Thu, 19 Mar 2020 11:05:10 GMT
2020-03-19T11:05:10.6099330Z == end clock drift check ==
2020-03-19T11:05:11.0575871Z 
2020-03-19T11:05:11.0682299Z ##[error]Bash exited with code '1'.
2020-03-19T11:05:11.0698847Z ##[section]Finishing: Run build
2020-03-19T11:05:11.0754144Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70122/merge to s
2020-03-19T11:05:11.0759620Z Task         : Get sources
2020-03-19T11:05:11.0759996Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-19T11:05:11.0760322Z Version      : 1.0.0
2020-03-19T11:05:11.0760877Z Author       : Microsoft
2020-03-19T11:05:11.0760877Z Author       : Microsoft
2020-03-19T11:05:11.0761277Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-19T11:05:11.0761687Z ==============================================================================
2020-03-19T11:05:11.4621705Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-19T11:05:11.4676163Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70122/merge to s
2020-03-19T11:05:11.4771193Z Cleaning up task key
2020-03-19T11:05:11.4772479Z Start cleaning up orphan processes.
2020-03-19T11:05:11.5088889Z Terminate orphan process: pid (3711) (python)
2020-03-19T11:05:11.5149154Z ##[section]Finishing: Finalize Job
