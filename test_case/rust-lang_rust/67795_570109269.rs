plain
2020-01-02T01:59:42.0631927Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-02T01:59:42.0845512Z ##[command]git config gc.auto 0
2020-01-02T01:59:42.0904100Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-02T01:59:42.0961654Z ##[command]git config --get-all http.proxy
2020-01-02T01:59:42.1118549Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67795/merge:refs/remotes/pull/67795/merge
---
2020-01-02T03:01:33.0907056Z .................................................................................................... 1500/9468
2020-01-02T03:01:38.9258258Z .................................................................................................... 1600/9468
2020-01-02T03:01:44.0078936Z .................................................................................................... 1700/9468
2020-01-02T03:01:53.3979291Z .................................................................................................... 1800/9468
2020-01-02T03:02:01.5778217Z .i.................................................................................................. 1900/9468
2020-01-02T03:02:08.2827269Z .......................................................................................iiiii........ 2000/9468
2020-01-02T03:02:30.2380006Z .................................................................................................... 2200/9468
2020-01-02T03:02:32.6297092Z .................................................................................................... 2300/9468
2020-01-02T03:02:35.1313273Z .................................................................................................... 2400/9468
2020-01-02T03:02:41.4866882Z .................................................................................................... 2500/9468
---
2020-01-02T03:05:41.1955241Z ..................i...............i................................................................. 4900/9468
2020-01-02T03:05:50.9250414Z .................................................................................................... 5000/9468
2020-01-02T03:05:56.5022807Z ...............................................................i.................................... 5100/9468
2020-01-02T03:06:04.7100095Z .................................................................................................... 5200/9468
2020-01-02T03:06:12.1047677Z ..............................ii.ii...........i..................................................... 5300/9468
2020-01-02T03:06:21.4614903Z .................................................................................................... 5500/9468
2020-01-02T03:06:31.7083055Z .................................................................................................... 5600/9468
2020-01-02T03:06:38.6485851Z .............i...................................................................................... 5700/9468
2020-01-02T03:06:44.7909093Z .................................................................................................... 5800/9468
2020-01-02T03:06:44.7909093Z .................................................................................................... 5800/9468
2020-01-02T03:06:55.5877759Z .................................................................................................... 5900/9468
2020-01-02T03:07:07.3682908Z .ii...i..ii...........i............................................................................. 6000/9468
2020-01-02T03:07:25.2393534Z .................................................................................................... 6200/9468
2020-01-02T03:07:30.4876708Z .................................................................................................... 6300/9468
2020-01-02T03:07:30.4876708Z .................................................................................................... 6300/9468
2020-01-02T03:07:43.8141685Z ............................i..ii................................................................... 6400/9468
2020-01-02T03:08:03.9998691Z .................................................................................................... 6600/9468
2020-01-02T03:08:06.8759067Z ...i................................................................................................ 6700/9468
2020-01-02T03:08:08.5069931Z .................................................................................................... 6800/9468
2020-01-02T03:08:11.1136334Z ...i................................................................................................ 6900/9468
---
2020-01-02T03:09:50.2620231Z .................................................................................................... 7500/9468
2020-01-02T03:09:55.1872762Z .................................................................................................... 7600/9468
2020-01-02T03:10:00.7332311Z .................................................................................................... 7700/9468
2020-01-02T03:10:10.9116001Z .................................................................................................... 7800/9468
2020-01-02T03:10:18.4973745Z .....................................iiii........................................................... 7900/9468
2020-01-02T03:10:33.2954499Z .................................................................................................... 8100/9468
2020-01-02T03:10:41.9447549Z .................................................................................................... 8200/9468
2020-01-02T03:10:56.1415487Z .................................................................................................... 8300/9468
2020-01-02T03:11:04.0019015Z .................................................................................................... 8400/9468
---
2020-01-02T03:12:59.3218507Z 64 
2020-01-02T03:12:59.3219954Z - error: future cannot be sent between threads safely
2020-01-02T03:12:59.3220583Z -   --> $DIR/async-fn-nonsend.rs:54:5
2020-01-02T03:12:59.3221035Z -    |
2020-01-02T03:12:59.3221496Z - LL | fn assert_send(_: impl Send) {}
2020-01-02T03:12:59.3222472Z - ...
2020-01-02T03:12:59.3222472Z - ...
2020-01-02T03:12:59.3222927Z - LL |     assert_send(non_sync_with_method_call());
2020-01-02T03:12:59.3223426Z -    |     ^^^^^^^^^^^ future returned by `non_sync_with_method_call` is not `Send`
2020-01-02T03:12:59.3223828Z -    |
2020-01-02T03:12:59.3224432Z -    = help: within `std::fmt::ArgumentV1<'_>`, the trait `std::marker::Sync` is not implemented for `*mut (dyn std::ops::Fn() + 'static)`
2020-01-02T03:12:59.3224912Z - note: future is not `Send` as this value is used across an await
2020-01-02T03:12:59.3225784Z -    |
2020-01-02T03:12:59.3226219Z - LL |     let f: &mut std::fmt::Formatter = panic!();
2020-01-02T03:12:59.3227211Z -    |         - has type `&mut std::fmt::Formatter<'_>`
2020-01-02T03:12:59.3227211Z -    |         - has type `&mut std::fmt::Formatter<'_>`
2020-01-02T03:12:59.3227716Z - LL |     if non_sync().fmt(f).unwrap() == () {
2020-01-02T03:12:59.3228143Z - LL |         fut().await;
2020-01-02T03:12:59.3229033Z -    |         ^^^^^^^^^^^ await occurs here, with `f` maybe used later
2020-01-02T03:12:59.3229459Z - LL |     }
2020-01-02T03:12:59.3229884Z - LL | }
2020-01-02T03:12:59.3230344Z -    | - `f` is later dropped here
2020-01-02T03:12:59.3231178Z - error: aborting due to 4 previous errors
2020-01-02T03:12:59.3231385Z + error: aborting due to 3 previous errors
2020-01-02T03:12:59.3231561Z 88 
2020-01-02T03:12:59.3231708Z 89 
2020-01-02T03:12:59.3231708Z 89 
2020-01-02T03:12:59.3231840Z 
2020-01-02T03:12:59.3231971Z 
2020-01-02T03:12:59.3232732Z The actual stderr differed from the expected stderr.
2020-01-02T03:12:59.3233389Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-nonsend/async-fn-nonsend.stderr
2020-01-02T03:12:59.3234210Z To update references, rerun the tests and pass the `--bless` flag
2020-01-02T03:12:59.3234767Z To only update this specific test, also pass `--test-args async-await/async-fn-nonsend.rs`
2020-01-02T03:12:59.3235295Z error: 1 errors occurred comparing output.
2020-01-02T03:12:59.3235535Z status: exit code: 1
2020-01-02T03:12:59.3235535Z status: exit code: 1
2020-01-02T03:12:59.3236797Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-fn-nonsend.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-nonsend" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--crate-type" "lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-nonsend/auxiliary" "-A" "unused"
2020-01-02T03:12:59.3237555Z ------------------------------------------
2020-01-02T03:12:59.3237765Z 
2020-01-02T03:12:59.3238206Z ------------------------------------------
2020-01-02T03:12:59.3238813Z stderr:
2020-01-02T03:12:59.3238813Z stderr:
2020-01-02T03:12:59.3239303Z ------------------------------------------
2020-01-02T03:12:59.3239517Z error: future cannot be sent between threads safely
2020-01-02T03:12:59.3239980Z   --> /checkout/src/test/ui/async-await/async-fn-nonsend.rs:50:5
2020-01-02T03:12:59.3240188Z    |
2020-01-02T03:12:59.3240373Z LL | fn assert_send(_: impl Send) {}
2020-01-02T03:12:59.3241014Z ...
2020-01-02T03:12:59.3241014Z ...
2020-01-02T03:12:59.3241198Z LL |     assert_send(local_dropped_before_await());
2020-01-02T03:12:59.3241359Z    |     ^^^^^^^^^^^ future returned by `local_dropped_before_await` is not `Send`
2020-01-02T03:12:59.3241714Z    = help: within `impl std::future::Future`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<()>`
2020-01-02T03:12:59.3241923Z note: future is not `Send` as this value is used across an await
2020-01-02T03:12:59.3242382Z   --> /checkout/src/test/ui/async-await/async-fn-nonsend.rs:25:5
2020-01-02T03:12:59.3242588Z    |
2020-01-02T03:12:59.3242588Z    |
2020-01-02T03:12:59.3242742Z LL |     let x = non_send();
2020-01-02T03:12:59.3243173Z    |         - has type `impl std::fmt::Debug`
2020-01-02T03:12:59.3243372Z LL |     drop(x);
2020-01-02T03:12:59.3243549Z LL |     fut().await;
2020-01-02T03:12:59.3243864Z LL | }
2020-01-02T03:12:59.3244246Z    | - `x` is later dropped here
2020-01-02T03:12:59.3244450Z 
2020-01-02T03:12:59.3244609Z error: future cannot be sent between threads safely
2020-01-02T03:12:59.3244609Z error: future cannot be sent between threads safely
2020-01-02T03:12:59.3245076Z   --> /checkout/src/test/ui/async-await/async-fn-nonsend.rs:52:5
2020-01-02T03:12:59.3245276Z    |
2020-01-02T03:12:59.3245427Z LL | fn assert_send(_: impl Send) {}
2020-01-02T03:12:59.3246075Z ...
2020-01-02T03:12:59.3246075Z ...
2020-01-02T03:12:59.3246259Z LL |     assert_send(non_send_temporary_in_match());
2020-01-02T03:12:59.3246425Z    |     ^^^^^^^^^^^ future returned by `non_send_temporary_in_match` is not `Send`
2020-01-02T03:12:59.3246748Z    = help: within `impl std::future::Future`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<()>`
2020-01-02T03:12:59.3246938Z note: future is not `Send` as this value is used across an await
2020-01-02T03:12:59.3247378Z   --> /checkout/src/test/ui/async-await/async-fn-nonsend.rs:34:20
2020-01-02T03:12:59.3247573Z    |
2020-01-02T03:12:59.3247573Z    |
2020-01-02T03:12:59.3247749Z LL |     match Some(non_send()) {
2020-01-02T03:12:59.3248546Z    |                ---------- has type `impl std::fmt::Debug`
2020-01-02T03:12:59.3248975Z LL |         Some(_) => fut().await,
2020-01-02T03:12:59.3249443Z ...
2020-01-02T03:12:59.3249592Z LL | }
2020-01-02T03:12:59.3250178Z    | - `non_send()` is later dropped here
2020-01-02T03:12:59.3250397Z 
2020-01-02T03:12:59.3250397Z 
2020-01-02T03:12:59.3250585Z error: future cannot be sent between threads safely
2020-01-02T03:12:59.3251040Z   --> /checkout/src/test/ui/async-await/async-fn-nonsend.rs:54:5
2020-01-02T03:12:59.3251245Z    |
2020-01-02T03:12:59.3251398Z LL | fn assert_send(_: impl Send) {}
2020-01-02T03:12:59.3252064Z ...
2020-01-02T03:12:59.3252242Z LL |     assert_send(non_sync_with_method_call());
2020-01-02T03:12:59.3252242Z LL |     assert_send(non_sync_with_method_call());
2020-01-02T03:12:59.3252401Z    |     ^^^^^^^^^^^ future returned by `non_sync_with_method_call` is not `Send`
2020-01-02T03:12:59.3252752Z    = help: the trait `std::marker::Send` is not implemented for `dyn std::fmt::Write`
2020-01-02T03:12:59.3252914Z note: future is not `Send` as this value is used across an await
2020-01-02T03:12:59.3253378Z   --> /checkout/src/test/ui/async-await/async-fn-nonsend.rs:43:9
2020-01-02T03:12:59.3253576Z    |
2020-01-02T03:12:59.3253576Z    |
2020-01-02T03:12:59.3253740Z LL |     let f: &mut std::fmt::Formatter = panic!();
2020-01-02T03:12:59.3254159Z    |         - has type `&mut std::fmt::Formatter<'_>`
2020-01-02T03:12:59.3254385Z LL |     if non_sync().fmt(f).unwrap() == () {
2020-01-02T03:12:59.3254560Z LL |         fut().await;
2020-01-02T03:12:59.3254722Z    |         ^^^^^^^^^^^ await occurs here, with `f` maybe used later
2020-01-02T03:12:59.3255018Z LL | }
2020-01-02T03:12:59.3255427Z    | - `f` is later dropped here
2020-01-02T03:12:59.3255603Z 
2020-01-02T03:12:59.3255758Z error: aborting due to 3 previous errors
---
2020-01-02T03:12:59.3281357Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2020-01-02T03:12:59.3281778Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-02T03:12:59.3315690Z 
2020-01-02T03:12:59.3315779Z 
2020-01-02T03:12:59.3320175Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-02T03:12:59.3320699Z 
2020-01-02T03:12:59.3320751Z 
2020-01-02T03:12:59.3320838Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-02T03:12:59.3320981Z Build completed unsuccessfully in 1:05:50
2020-01-02T03:12:59.3320981Z Build completed unsuccessfully in 1:05:50
2020-01-02T03:12:59.3373318Z == clock drift check ==
2020-01-02T03:12:59.3399245Z   local time: Thu Jan  2 03:12:59 UTC 2020
2020-01-02T03:12:59.6318262Z   network time: Thu, 02 Jan 2020 03:12:59 GMT
2020-01-02T03:12:59.6321347Z == end clock drift check ==
2020-01-02T03:13:00.5954281Z 
2020-01-02T03:13:00.6065918Z ##[error]Bash exited with code '1'.
2020-01-02T03:13:00.6104985Z ##[section]Starting: Checkout
2020-01-02T03:13:00.6107668Z ==============================================================================
2020-01-02T03:13:00.6107753Z Task         : Get sources
2020-01-02T03:13:00.6107825Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
