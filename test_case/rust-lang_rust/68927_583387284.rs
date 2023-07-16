plain
2020-02-07T12:24:53.0591259Z ========================== Starting Command Output ===========================
2020-02-07T12:24:53.0607549Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/a244c40b-f66f-439f-a5eb-6db9f5d9efc1.sh
2020-02-07T12:24:53.0754651Z 
2020-02-07T12:24:53.0853162Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-07T12:24:53.0857919Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68927/merge to s
2020-02-07T12:24:53.0859377Z Task         : Get sources
2020-02-07T12:24:53.0859405Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-07T12:24:53.0859429Z Version      : 1.0.0
2020-02-07T12:24:53.0859452Z Author       : Microsoft
---
2020-02-07T12:24:53.8072477Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-07T12:24:53.8290918Z ##[command]git config gc.auto 0
2020-02-07T12:24:53.8293815Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-07T12:24:53.8295938Z ##[command]git config --get-all http.proxy
2020-02-07T12:24:53.8364124Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68927/merge:refs/remotes/pull/68927/merge
---
2020-02-07T13:14:35.5837425Z .................................................................................................... 1700/9607
2020-02-07T13:14:39.7303325Z .................................................................................................... 1800/9607
2020-02-07T13:14:49.5830287Z ..............................i..................................................................... 1900/9607
2020-02-07T13:14:56.0520922Z .................................................................................................... 2000/9607
2020-02-07T13:15:07.7571099Z ....................iiiii........................................................................... 2100/9607
2020-02-07T13:15:15.8063816Z .................................................................................................... 2300/9607
2020-02-07T13:15:17.7465845Z .................................................................................................... 2400/9607
2020-02-07T13:15:21.6035231Z .................................................................................................... 2500/9607
2020-02-07T13:15:39.4238092Z .................................................................................................... 2600/9607
---
2020-02-07T13:17:55.3795490Z ........................................................................i...............i........... 4900/9607
2020-02-07T13:18:01.9875206Z .................................................................................................... 5000/9607
2020-02-07T13:18:08.6926830Z .................................................................................................... 5100/9607
2020-02-07T13:18:12.5389607Z ...............i.................................................................................... 5200/9607
2020-02-07T13:18:21.5320312Z .........................................................................................ii.ii...... 5300/9607
2020-02-07T13:18:25.1306153Z ..i...i............................................................................................. 5400/9607
2020-02-07T13:18:33.7772670Z .................................................................................................... 5600/9607
2020-02-07T13:18:42.2679693Z .............................................................................i...................... 5700/9607
2020-02-07T13:18:48.4575031Z .................................................................................................... 5800/9607
2020-02-07T13:18:53.8228616Z .................................................................................................... 5900/9607
2020-02-07T13:18:53.8228616Z .................................................................................................... 5900/9607
2020-02-07T13:19:02.1087743Z ....................................................................ii...i..ii...........i.......... 6000/9607
2020-02-07T13:19:19.8353684Z .................................................................................................... 6200/9607
2020-02-07T13:19:26.0167214Z .................................................................................................... 6300/9607
2020-02-07T13:19:32.5778779Z ................................................................................................i..i 6400/9607
2020-02-07T13:19:49.5152342Z i................................................................................................... 6500/9607
---
2020-02-07T13:21:33.2082911Z .................................................................................................... 7600/9607
2020-02-07T13:21:37.4198003Z .................................................................................................... 7700/9607
2020-02-07T13:21:41.8588915Z .................................................................................................... 7800/9607
2020-02-07T13:21:49.7766586Z .................................................................................................... 7900/9607
2020-02-07T13:21:57.4003297Z ......................................................iiiiiii.i..................................... 8000/9607
2020-02-07T13:22:09.9489730Z .i.................................................................................................. 8200/9607
2020-02-07T13:22:14.4925276Z .................................................................................................... 8300/9607
2020-02-07T13:22:27.8834634Z .................................................................................................... 8400/9607
2020-02-07T13:22:34.8833139Z .................................................................................................... 8500/9607
---
2020-02-07T13:24:13.6177728Z 9    |
2020-02-07T13:24:13.6177782Z 10    = note: the return type of a function must have a statically known size
2020-02-07T13:24:13.6178067Z -    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2020-02-07T13:24:13.6178114Z 12 
2020-02-07T13:24:13.6178167Z 13 error[E0698]: type inside `async fn` body must be known in this context
2020-02-07T13:24:13.6178398Z 
2020-02-07T13:24:13.6178417Z 
2020-02-07T13:24:13.6178467Z The actual stderr differed from the expected stderr.
2020-02-07T13:24:13.6178751Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-error-span/async-error-span.stderr
2020-02-07T13:24:13.6178751Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-error-span/async-error-span.stderr
2020-02-07T13:24:13.6178976Z To update references, rerun the tests and pass the `--bless` flag
2020-02-07T13:24:13.6179242Z To only update this specific test, also pass `--test-args async-await/async-error-span.rs`
2020-02-07T13:24:13.6179301Z error: 1 errors occurred comparing output.
2020-02-07T13:24:13.6179350Z status: exit code: 1
2020-02-07T13:24:13.6179350Z status: exit code: 1
2020-02-07T13:24:13.6180001Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-error-span.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-error-span" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-error-span/auxiliary" "-A" "unused"
2020-02-07T13:24:13.6180285Z ------------------------------------------
2020-02-07T13:24:13.6180310Z 
2020-02-07T13:24:13.6180574Z ------------------------------------------
2020-02-07T13:24:13.6180610Z stderr:
2020-02-07T13:24:13.6180610Z stderr:
2020-02-07T13:24:13.6180799Z ------------------------------------------
2020-02-07T13:24:13.6180839Z error[E0277]: the trait bound `(): std::future::Future` is not satisfied
2020-02-07T13:24:13.6181079Z   --> /checkout/src/test/ui/async-await/async-error-span.rs:7:20
2020-02-07T13:24:13.6181118Z    |
2020-02-07T13:24:13.6181307Z LL | fn get_future() -> impl Future<Output = ()> {
2020-02-07T13:24:13.6181368Z    |                    ^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::future::Future` is not implemented for `()`
2020-02-07T13:24:13.6181411Z LL | //~^ ERROR the trait bound `(): std::future::Future` is not satisfied
2020-02-07T13:24:13.6181671Z    |     -------- this returned value is of type `!`
2020-02-07T13:24:13.6181705Z    |
2020-02-07T13:24:13.6181740Z    = note: the return type of a function must have a statically known size
2020-02-07T13:24:13.6181785Z 
2020-02-07T13:24:13.6181785Z 
2020-02-07T13:24:13.6181820Z error[E0698]: type inside `async fn` body must be known in this context
2020-02-07T13:24:13.6182081Z    |
2020-02-07T13:24:13.6182081Z    |
2020-02-07T13:24:13.6182116Z LL |     let a; //~ ERROR type inside `async fn` body must be known in this context
2020-02-07T13:24:13.6182200Z    |
2020-02-07T13:24:13.6182200Z    |
2020-02-07T13:24:13.6182234Z note: the type is part of the `async fn` body because of this `await`
2020-02-07T13:24:13.6182476Z    |
2020-02-07T13:24:13.6182476Z    |
2020-02-07T13:24:13.6182522Z LL |     get_future().await;
2020-02-07T13:24:13.6182676Z 
2020-02-07T13:24:13.6182724Z error: aborting due to 2 previous errors
2020-02-07T13:24:13.6182745Z 
2020-02-07T13:24:13.6182778Z Some errors have detailed explanations: E0277, E0698.
---
2020-02-07T13:24:13.6193343Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-07T13:24:13.6193419Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-07T13:24:13.6200948Z 
2020-02-07T13:24:13.6201265Z 
2020-02-07T13:24:13.6202661Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-07T13:24:13.6203061Z 
2020-02-07T13:24:13.6203179Z 
2020-02-07T13:24:13.6250320Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-07T13:24:13.6251911Z Build completed unsuccessfully in 0:53:41
2020-02-07T13:24:13.6251911Z Build completed unsuccessfully in 0:53:41
2020-02-07T13:24:13.6255681Z == clock drift check ==
2020-02-07T13:24:13.6277360Z   local time: Fri Feb  7 13:24:13 UTC 2020
2020-02-07T13:24:13.9242253Z   network time: Fri, 07 Feb 2020 13:24:13 GMT
2020-02-07T13:24:13.9242320Z == end clock drift check ==
2020-02-07T13:24:14.3507728Z 
2020-02-07T13:24:14.3558367Z ##[error]Bash exited with code '1'.
2020-02-07T13:24:14.3567718Z ##[section]Finishing: Run build
2020-02-07T13:24:14.3589914Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68927/merge to s
2020-02-07T13:24:14.3591534Z Task         : Get sources
2020-02-07T13:24:14.3591572Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-07T13:24:14.3591624Z Version      : 1.0.0
2020-02-07T13:24:14.3591654Z Author       : Microsoft
2020-02-07T13:24:14.3591654Z Author       : Microsoft
2020-02-07T13:24:14.3591689Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-07T13:24:14.3591741Z ==============================================================================
2020-02-07T13:24:14.7220047Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-07T13:24:14.7256367Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68927/merge to s
2020-02-07T13:24:14.7345817Z Cleaning up task key
2020-02-07T13:24:14.7346462Z Start cleaning up orphan processes.
2020-02-07T13:24:14.7622287Z Terminate orphan process: pid (5561) (python)
2020-02-07T13:24:14.7644815Z ##[section]Finishing: Finalize Job
