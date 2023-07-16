plain
2020-01-17T03:26:15.5832460Z ========================== Starting Command Output ===========================
2020-01-17T03:26:15.5835667Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/0d949022-d876-437f-9a87-a6825d510412.sh
2020-01-17T03:26:15.5835916Z 
2020-01-17T03:26:15.5838910Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-17T03:26:15.5845303Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68269/merge to s
2020-01-17T03:26:15.5846903Z Task         : Get sources
2020-01-17T03:26:15.5846932Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-17T03:26:15.5846960Z Version      : 1.0.0
2020-01-17T03:26:15.5847033Z Author       : Microsoft
---
2020-01-17T03:26:16.4724489Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-17T03:26:16.4826792Z ##[command]git config gc.auto 0
2020-01-17T03:26:16.4903563Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-17T03:26:16.4961560Z ##[command]git config --get-all http.proxy
2020-01-17T03:26:16.5112278Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68269/merge:refs/remotes/pull/68269/merge
---
2020-01-17T04:22:47.7744372Z .................................................................................................... 1700/9525
2020-01-17T04:22:55.9986809Z .................................................................................................... 1800/9525
2020-01-17T04:23:06.0072421Z ............i....................................................................................... 1900/9525
2020-01-17T04:23:13.3067126Z .................................................................................................... 2000/9525
2020-01-17T04:23:29.7017365Z ..iiiii............................................................................................. 2100/9525
2020-01-17T04:23:38.6997550Z .................................................................................................... 2300/9525
2020-01-17T04:23:41.2465649Z .................................................................................................... 2400/9525
2020-01-17T04:23:47.0747414Z .................................................................................................... 2500/9525
2020-01-17T04:24:08.1681565Z .................................................................................................... 2600/9525
2020-01-17T04:24:08.1681565Z .................................................................................................... 2600/9525
2020-01-17T04:24:10.8743637Z .................................................................................................... 2700/9525
2020-01-17T04:24:21.9494919Z ..........................................................................i......................... 2800/9525
2020-01-17T04:24:28.6903770Z .................................................................................................... 2900/9525
2020-01-17T04:24:37.1063960Z .................................................................................................... 3000/9525
2020-01-17T04:24:42.3855531Z ...........i........................................................................................ 3100/9525
2020-01-17T04:24:51.7395356Z .................................................................................................... 3200/9525
2020-01-17T04:24:56.3590085Z ................................................................................................ii.. 3300/9525
2020-01-17T04:25:13.1848016Z .................................................................................................... 3500/9525
2020-01-17T04:25:21.4104981Z .......................................................................................i............ 3600/9525
2020-01-17T04:25:28.2595320Z ..................................i................................................................. 3700/9525
2020-01-17T04:25:34.2072629Z .................................................................................................... 3800/9525
---
2020-01-17T04:26:54.2218013Z .............................................i...............i...................................... 4900/9525
2020-01-17T04:27:03.4704291Z .................................................................................................... 5000/9525
2020-01-17T04:27:10.5504129Z ........................................................................................i........... 5100/9525
2020-01-17T04:27:15.9840673Z .................................................................................................... 5200/9525
2020-01-17T04:27:27.0328042Z ............................................................ii.ii...........i....................... 5300/9525
2020-01-17T04:27:31.5774406Z .................................................................................................i.. 5400/9525
2020-01-17T04:27:47.1442592Z .................................................................................................... 5600/9525
2020-01-17T04:27:53.6043075Z .............................................i...................................................... 5700/9525
2020-01-17T04:28:00.9603606Z .................................................................................................... 5800/9525
2020-01-17T04:28:11.9324654Z .................................................................................................... 5900/9525
2020-01-17T04:28:11.9324654Z .................................................................................................... 5900/9525
2020-01-17T04:28:21.5919389Z ....................................ii...i..ii...........i.......................................... 6000/9525
2020-01-17T04:28:41.8014640Z .................................................................................................... 6200/9525
2020-01-17T04:28:50.2828192Z .................................................................................................... 6300/9525
2020-01-17T04:28:50.2828192Z .................................................................................................... 6300/9525
2020-01-17T04:28:58.7727005Z ................................................................i..ii............................... 6400/9525
2020-01-17T04:29:28.5827858Z .................................................................................................... 6600/9525
2020-01-17T04:29:30.9257768Z ........................................i........................................................... 6700/9525
2020-01-17T04:29:33.2303091Z .................................................................................................... 6800/9525
2020-01-17T04:29:35.8093970Z ........................................i........................................................... 6900/9525
---
2020-01-17T04:31:17.0921589Z .................................................................................................... 7500/9525
2020-01-17T04:31:21.7776084Z .................................................................................................... 7600/9525
2020-01-17T04:31:28.0776470Z .................................................................................................... 7700/9525
2020-01-17T04:31:35.1070558Z .................................................................................................... 7800/9525
2020-01-17T04:31:45.8921440Z .........................................................................................iiii....... 7900/9525
2020-01-17T04:32:03.5269977Z .......................i......i..................................................................... 8100/9525
2020-01-17T04:32:08.8380939Z .................................................................................................... 8200/9525
2020-01-17T04:32:22.1374883Z .................................................................................................... 8300/9525
2020-01-17T04:32:33.8241955Z .................................................................................................... 8400/9525
---
2020-01-17T04:34:39.0445240Z 22 LL |         match client.status() {
2020-01-17T04:34:39.0445296Z 
2020-01-17T04:34:39.0445328Z 
2020-01-17T04:34:39.0445379Z The actual stderr differed from the expected stderr.
2020-01-17T04:34:39.0446027Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-64130-4-async-move/issue-64130-4-async-move.stderr
2020-01-17T04:34:39.0446339Z To update references, rerun the tests and pass the `--bless` flag
2020-01-17T04:34:39.0446659Z To only update this specific test, also pass `--test-args async-await/issue-64130-4-async-move.rs`
2020-01-17T04:34:39.0446901Z error: 1 errors occurred comparing output.
2020-01-17T04:34:39.0446965Z status: exit code: 1
2020-01-17T04:34:39.0446965Z status: exit code: 1
2020-01-17T04:34:39.0447971Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-64130-4-async-move.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-64130-4-async-move" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-64130-4-async-move/auxiliary" "-A" "unused"
2020-01-17T04:34:39.0448361Z ------------------------------------------
2020-01-17T04:34:39.0448420Z 
2020-01-17T04:34:39.0448669Z ------------------------------------------
2020-01-17T04:34:39.0448730Z stderr:
---
2020-01-17T04:34:39.0450861Z    = help: the trait `std::marker::Sync` is not implemented for `(dyn std::any::Any + std::marker::Send + 'static)`
2020-01-17T04:34:39.0450958Z note: future is not `Send` as this value is used across an await
2020-01-17T04:34:39.0451287Z   --> /checkout/src/test/ui/async-await/issue-64130-4-async-move.rs:21:26
2020-01-17T04:34:39.0451343Z    |
2020-01-17T04:34:39.0451392Z LL |         match client.status() {
2020-01-17T04:34:39.0451668Z    |               ------ has type `&Client`
2020-01-17T04:34:39.0451775Z LL |                 let _x = get().await;
2020-01-17T04:34:39.0451834Z    |                          ^^^^^^^^^^^ await occurs here, with `client` maybe used later
2020-01-17T04:34:39.0451907Z ...
2020-01-17T04:34:39.0451955Z LL |     }
---
2020-01-17T04:34:39.0483784Z - help: consider moving this into a `let` binding to create a shorter lived borrow
2020-01-17T04:34:39.0483910Z + help: consider moving this this into a `let` binding to create a shorter lived borrow
2020-01-17T04:34:39.0484182Z 20   --> $DIR/issue-65436-raw-ptr-not-send.rs:14:13
2020-01-17T04:34:39.0484237Z 21    |
2020-01-17T04:34:39.0484301Z 22 LL |         bar(Foo(std::ptr::null())).await;
2020-01-17T04:34:39.0484473Z 
2020-01-17T04:34:39.0484536Z The actual stderr differed from the expected stderr.
2020-01-17T04:34:39.0484980Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-65436-raw-ptr-not-send/issue-65436-raw-ptr-not-send.stderr
2020-01-17T04:34:39.0484980Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-65436-raw-ptr-not-send/issue-65436-raw-ptr-not-send.stderr
2020-01-17T04:34:39.0485263Z To update references, rerun the tests and pass the `--bless` flag
2020-01-17T04:34:39.0485611Z To only update this specific test, also pass `--test-args async-await/issues/issue-65436-raw-ptr-not-send.rs`
2020-01-17T04:34:39.0485717Z error: 1 errors occurred comparing output.
2020-01-17T04:34:39.0485768Z status: exit code: 1
2020-01-17T04:34:39.0485768Z status: exit code: 1
2020-01-17T04:34:39.0486793Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-65436-raw-ptr-not-send.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-65436-raw-ptr-not-send" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-65436-raw-ptr-not-send/auxiliary" "-A" "unused"
2020-01-17T04:34:39.0487162Z ------------------------------------------
2020-01-17T04:34:39.0487202Z 
2020-01-17T04:34:39.0487445Z ------------------------------------------
2020-01-17T04:34:39.0487522Z stderr:
2020-01-17T04:34:39.0487522Z stderr:
2020-01-17T04:34:39.0487763Z ------------------------------------------
2020-01-17T04:34:39.0487819Z error: future cannot be sent between threads safely
2020-01-17T04:34:39.0488117Z   --> /checkout/src/test/ui/async-await/issues/issue-65436-raw-ptr-not-send.rs:12:5
2020-01-17T04:34:39.0488177Z    |
2020-01-17T04:34:39.0488230Z LL | fn assert_send<T: Send>(_: T) {}
2020-01-17T04:34:39.0488572Z ...
2020-01-17T04:34:39.0488622Z LL |     assert_send(async {
2020-01-17T04:34:39.0488685Z    |     ^^^^^^^^^^^ future returned by `main` is not `Send`
2020-01-17T04:34:39.0488736Z    |
2020-01-17T04:34:39.0488736Z    |
2020-01-17T04:34:39.0488796Z    = help: within `impl std::future::Future`, the trait `std::marker::Send` is not implemented for `*const u8`
2020-01-17T04:34:39.0488871Z note: future is not `Send` as this value is used across an await
2020-01-17T04:34:39.0489172Z   --> /checkout/src/test/ui/async-await/issues/issue-65436-raw-ptr-not-send.rs:14:9
2020-01-17T04:34:39.0489236Z    |
2020-01-17T04:34:39.0489297Z LL |         bar(Foo(std::ptr::null())).await;
2020-01-17T04:34:39.0489590Z    |         ^^^^^^^^----------------^^^^^^^^- `std::ptr::null()` is later dropped here
2020-01-17T04:34:39.0489705Z    |         |       has type `*const u8`
2020-01-17T04:34:39.0489772Z    |         await occurs here, with `std::ptr::null()` maybe used later
2020-01-17T04:34:39.0489834Z help: consider moving this this into a `let` binding to create a shorter lived borrow
2020-01-17T04:34:39.0490873Z   --> /checkout/src/test/ui/async-await/issues/issue-65436-raw-ptr-not-send.rs:14:13
2020-01-17T04:34:39.0490873Z   --> /checkout/src/test/ui/async-await/issues/issue-65436-raw-ptr-not-send.rs:14:13
2020-01-17T04:34:39.0490944Z    |
2020-01-17T04:34:39.0490994Z LL |         bar(Foo(std::ptr::null())).await;
2020-01-17T04:34:39.0491099Z 
2020-01-17T04:34:39.0491149Z error: aborting due to previous error
2020-01-17T04:34:39.0491583Z 
2020-01-17T04:34:39.0491613Z 
---
2020-01-17T04:34:39.0531015Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:387:22
2020-01-17T04:34:39.0531205Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-01-17T04:34:39.0531423Z 
2020-01-17T04:34:39.0531592Z 
2020-01-17T04:34:39.0533667Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-17T04:34:39.0534293Z 
2020-01-17T04:34:39.0534442Z 
2020-01-17T04:34:39.0534608Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-17T04:34:39.0534767Z Build completed unsuccessfully in 1:02:49
2020-01-17T04:34:39.0534767Z Build completed unsuccessfully in 1:02:49
2020-01-17T04:34:39.0589144Z == clock drift check ==
2020-01-17T04:34:39.0607816Z   local time: Fri Jan 17 04:34:39 UTC 2020
2020-01-17T04:34:39.2227949Z   network time: Fri, 17 Jan 2020 04:34:39 GMT
2020-01-17T04:34:39.2231178Z == end clock drift check ==
2020-01-17T04:34:39.6382071Z 
2020-01-17T04:34:39.6516086Z ##[error]Bash exited with code '1'.
2020-01-17T04:34:39.6529470Z ##[section]Finishing: Run build
2020-01-17T04:34:39.6551542Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68269/merge to s
2020-01-17T04:34:39.6553625Z Task         : Get sources
2020-01-17T04:34:39.6553673Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-17T04:34:39.6553718Z Version      : 1.0.0
2020-01-17T04:34:39.6553760Z Author       : Microsoft
2020-01-17T04:34:39.6553760Z Author       : Microsoft
2020-01-17T04:34:39.6553827Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-17T04:34:39.6553878Z ==============================================================================
2020-01-17T04:34:40.0873457Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-17T04:34:40.0915971Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68269/merge to s
2020-01-17T04:34:40.1047686Z Cleaning up task key
2020-01-17T04:34:40.1048523Z Start cleaning up orphan processes.
2020-01-17T04:34:40.1172496Z Terminate orphan process: pid (3530) (python)
2020-01-17T04:34:40.1392267Z ##[section]Finishing: Finalize Job
