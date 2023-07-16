plain
2020-03-17T04:16:01.5126600Z ========================== Starting Command Output ===========================
2020-03-17T04:16:01.5131766Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/eac5a23f-b6ba-4975-83e8-d8ab116fe975.sh
2020-03-17T04:16:01.5132265Z 
2020-03-17T04:16:01.5136935Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-17T04:16:01.5157359Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70043/merge to s
2020-03-17T04:16:01.5164523Z Task         : Get sources
2020-03-17T04:16:01.5164870Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-17T04:16:01.5165185Z Version      : 1.0.0
2020-03-17T04:16:01.5165398Z Author       : Microsoft
---
2020-03-17T04:16:02.6764826Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-17T04:16:03.6627193Z ##[command]git config gc.auto 0
2020-03-17T04:16:03.6633346Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-17T04:16:03.6639492Z ##[command]git config --get-all http.proxy
2020-03-17T04:16:03.6652088Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70043/merge:refs/remotes/pull/70043/merge
---
2020-03-17T05:19:34.3377570Z .................................................................................................... 1700/9789
2020-03-17T05:19:38.7729104Z .................................................................................................... 1800/9789
2020-03-17T05:19:50.5523555Z ......................................................................i............................. 1900/9789
2020-03-17T05:19:56.9859777Z .................................................................................................... 2000/9789
2020-03-17T05:20:12.0428334Z ............................................................iiiii................................... 2100/9789
2020-03-17T05:20:22.7863774Z .................................................................................................... 2300/9789
2020-03-17T05:20:24.9755579Z .................................................................................................... 2400/9789
2020-03-17T05:20:27.9956190Z .................................................................................................... 2500/9789
2020-03-17T05:20:48.6536853Z .................................................................................................... 2600/9789
---
2020-03-17T05:23:32.4069618Z ................................i...............i................................................... 5000/9789
2020-03-17T05:23:41.6602964Z .................................................................................................... 5100/9789
2020-03-17T05:23:48.2833260Z ...........................................................................i........................ 5200/9789
2020-03-17T05:23:53.8766598Z .................................................................................................... 5300/9789
2020-03-17T05:24:04.9199126Z ........................................................ii.ii........i...i.......................... 5400/9789
2020-03-17T05:24:12.9213967Z .................................................................................................... 5600/9789
2020-03-17T05:24:21.9424974Z .................................................................................................... 5700/9789
2020-03-17T05:24:28.1086779Z ................................................i................................................... 5800/9789
2020-03-17T05:24:34.6339950Z .................................................................................................... 5900/9789
2020-03-17T05:24:34.6339950Z .................................................................................................... 5900/9789
2020-03-17T05:24:44.9281716Z .................................................................................................... 6000/9789
2020-03-17T05:24:50.9364644Z ..........................................ii...i..ii...........i.................................... 6100/9789
2020-03-17T05:25:11.3295789Z .................................................................................................... 6300/9789
2020-03-17T05:25:18.2018775Z .................................................................................................... 6400/9789
2020-03-17T05:25:18.2018775Z .................................................................................................... 6400/9789
2020-03-17T05:25:27.3686462Z ........................................................................i..ii....................... 6500/9789
2020-03-17T05:25:51.7915520Z .................................................................................................... 6700/9789
2020-03-17T05:26:01.3436446Z ......................................................................i............................. 6800/9789
2020-03-17T05:26:03.4322036Z .................................................................................................... 6900/9789
2020-03-17T05:26:05.6794853Z .................................................................................................... 7000/9789
---
2020-03-17T05:27:49.9324244Z .................................................................................................... 7800/9789
2020-03-17T05:27:55.7874728Z .................................................................................................... 7900/9789
2020-03-17T05:28:01.6857407Z ......................................................i............................................. 8000/9789
2020-03-17T05:28:12.0017492Z .................................................................................................... 8100/9789
2020-03-17T05:28:17.4194479Z ...iiiiiiiiii.i..................................................................................... 8200/9789
2020-03-17T05:28:31.0476326Z .................................................................................................... 8400/9789
2020-03-17T05:28:39.2473523Z .................................................................................................... 8500/9789
2020-03-17T05:28:53.3695101Z .................................................................................................... 8600/9789
2020-03-17T05:28:59.9056661Z .................................................................................................... 8700/9789
---
2020-03-17T05:30:53.1996383Z 2   --> $DIR/ref-escapes-but-not-over-yield.rs:11:9
2020-03-17T05:30:53.1996766Z 3    |
2020-03-17T05:30:53.1997068Z 4 LL |     let mut a = &3;
2020-03-17T05:30:53.1997312Z 
2020-03-17T05:30:53.1997884Z -    |         ----- `a` declared here, outside of the generator body
2020-03-17T05:30:53.1998626Z +    |         ----- `a` declared here, outside of the closure body
2020-03-17T05:30:53.1999388Z 7 LL |         a = &b;
2020-03-17T05:30:53.2000663Z 8    |         ^^^^--
2020-03-17T05:30:53.2000807Z 
2020-03-17T05:30:53.2000953Z 9    |         |   |
2020-03-17T05:30:53.2000953Z 9    |         |   |
2020-03-17T05:30:53.2001429Z -    |         |   borrow is only valid in the generator body
2020-03-17T05:30:53.2001952Z -    |         reference to `b` escapes the generator body here
2020-03-17T05:30:53.2002282Z +    |         |   borrow is only valid in the closure body
2020-03-17T05:30:53.2002621Z +    |         reference to `b` escapes the closure body here
2020-03-17T05:30:53.2003046Z 13 error: aborting due to previous error
2020-03-17T05:30:53.2003256Z 14 
2020-03-17T05:30:53.2003357Z 
2020-03-17T05:30:53.2003454Z 
2020-03-17T05:30:53.2003454Z 
2020-03-17T05:30:53.2003660Z The actual stderr differed from the expected stderr.
2020-03-17T05:30:53.2004428Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/ref-escapes-but-not-over-yield/ref-escapes-but-not-over-yield.stderr
2020-03-17T05:30:53.2005116Z To update references, rerun the tests and pass the `--bless` flag
2020-03-17T05:30:53.2005766Z To only update this specific test, also pass `--test-args generator/ref-escapes-but-not-over-yield.rs`
2020-03-17T05:30:53.2006236Z error: 1 errors occurred comparing output.
2020-03-17T05:30:53.2006471Z status: exit code: 1
2020-03-17T05:30:53.2006471Z status: exit code: 1
2020-03-17T05:30:53.2008562Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/ref-escapes-but-not-over-yield.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/ref-escapes-but-not-over-yield" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/ref-escapes-but-not-over-yield/auxiliary"
2020-03-17T05:30:53.2010459Z ------------------------------------------
2020-03-17T05:30:53.2010634Z 
2020-03-17T05:30:53.2011074Z ------------------------------------------
2020-03-17T05:30:53.2011301Z stderr:
2020-03-17T05:30:53.2011301Z stderr:
2020-03-17T05:30:53.2011687Z ------------------------------------------
2020-03-17T05:30:53.2011979Z error[E0521]: borrowed data escapes outside of closure
2020-03-17T05:30:53.2012574Z   --> /checkout/src/test/ui/generator/ref-escapes-but-not-over-yield.rs:11:9
2020-03-17T05:30:53.2012852Z    |
2020-03-17T05:30:53.2013025Z LL |     let mut a = &3;
2020-03-17T05:30:53.2013502Z    |         ----- `a` declared here, outside of the closure body
2020-03-17T05:30:53.2013891Z LL |         a = &b;
2020-03-17T05:30:53.2014234Z    |         ^^^^--
2020-03-17T05:30:53.2014417Z    |         |   |
2020-03-17T05:30:53.2014662Z    |         |   borrow is only valid in the closure body
2020-03-17T05:30:53.2014662Z    |         |   borrow is only valid in the closure body
2020-03-17T05:30:53.2014996Z    |         reference to `b` escapes the closure body here
2020-03-17T05:30:53.2015376Z error: aborting due to previous error
2020-03-17T05:30:53.2015542Z 
2020-03-17T05:30:53.2015644Z 
2020-03-17T05:30:53.2016017Z ------------------------------------------
---
2020-03-17T05:30:53.2025475Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-17T05:30:53.2025908Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-17T05:30:53.2040686Z 
2020-03-17T05:30:53.2040878Z 
2020-03-17T05:30:53.2045363Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-17T05:30:53.2048472Z 
2020-03-17T05:30:53.2048584Z 
2020-03-17T05:30:53.2055880Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-03-17T05:30:53.2056425Z Build completed unsuccessfully in 1:07:43
2020-03-17T05:30:53.2056425Z Build completed unsuccessfully in 1:07:43
2020-03-17T05:30:53.2115923Z == clock drift check ==
2020-03-17T05:30:53.2134003Z   local time: Tue Mar 17 05:30:53 UTC 2020
2020-03-17T05:30:53.7658143Z   network time: Tue, 17 Mar 2020 05:30:53 GMT
2020-03-17T05:30:53.7659056Z == end clock drift check ==
2020-03-17T05:30:54.3018997Z 
2020-03-17T05:30:54.3101052Z ##[error]Bash exited with code '1'.
2020-03-17T05:30:54.3121823Z ##[section]Finishing: Run build
2020-03-17T05:30:54.3168934Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70043/merge to s
2020-03-17T05:30:54.3173919Z Task         : Get sources
2020-03-17T05:30:54.3174260Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-17T05:30:54.3174593Z Version      : 1.0.0
2020-03-17T05:30:54.3174815Z Author       : Microsoft
2020-03-17T05:30:54.3174815Z Author       : Microsoft
2020-03-17T05:30:54.3175165Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-17T05:30:54.3175591Z ==============================================================================
2020-03-17T05:30:54.6850900Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-17T05:30:54.6899233Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70043/merge to s
2020-03-17T05:30:54.7006333Z Cleaning up task key
2020-03-17T05:30:54.7007717Z Start cleaning up orphan processes.
2020-03-17T05:30:54.7235424Z Terminate orphan process: pid (3805) (python)
2020-03-17T05:30:54.7414189Z ##[section]Finishing: Finalize Job
