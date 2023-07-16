plain
2019-09-18T20:52:27.0768002Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-18T20:52:27.0956456Z ##[command]git config gc.auto 0
2019-09-18T20:52:27.1039403Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-18T20:52:27.1105134Z ##[command]git config --get-all http.proxy
2019-09-18T20:52:27.1237148Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64151/merge:refs/remotes/pull/64151/merge
---
2019-09-18T21:56:12.7609939Z .................................................................................................... 1500/9024
2019-09-18T21:56:18.9713816Z .......F............................................................................................ 1600/9024
2019-09-18T21:56:32.0081454Z .................................................................i...............i.................. 1700/9024
2019-09-18T21:56:39.3602089Z .................................................................................................... 1800/9024
2019-09-18T21:56:54.7754343Z ........................................................iiiii....................................... 1900/9024
2019-09-18T21:57:06.6011127Z .................................................................................................... 2100/9024
2019-09-18T21:57:09.2735368Z .................................................................................................... 2200/9024
2019-09-18T21:57:12.7298747Z .................................................................................................... 2300/9024
2019-09-18T21:57:21.2629319Z .................................................................................................... 2400/9024
---
2019-09-18T22:00:23.7420622Z ............................................i...............i....................................... 4700/9024
2019-09-18T22:00:34.8893745Z .................................................................................................... 4800/9024
2019-09-18T22:00:42.1383929Z .................................................................................................... 4900/9024
2019-09-18T22:00:52.0952053Z .................................................................................................... 5000/9024
2019-09-18T22:01:00.0901411Z ............................ii.ii................................................................... 5100/9024
2019-09-18T22:01:10.4745489Z .................................................................................................... 5300/9024
2019-09-18T22:01:21.0561903Z ............................................................................................i....... 5400/9024
2019-09-18T22:01:29.6897237Z .................................................................................................... 5500/9024
2019-09-18T22:01:34.7543470Z .................................................................................................... 5600/9024
2019-09-18T22:01:34.7543470Z .................................................................................................... 5600/9024
2019-09-18T22:01:45.5913886Z .......................................................................................ii...i..ii... 5700/9024
2019-09-18T22:02:11.6880583Z .................................................................................................... 5900/9024
2019-09-18T22:02:22.1161831Z .................................................................................................... 6000/9024
2019-09-18T22:02:22.1161831Z .................................................................................................... 6000/9024
2019-09-18T22:02:31.4313705Z .........................................................................................i..ii...... 6100/9024
2019-09-18T22:03:02.2491150Z .................................................................................................... 6300/9024
2019-09-18T22:03:06.5688627Z ................................................i................................................... 6400/9024
2019-09-18T22:03:08.9317451Z .................................................................................................... 6500/9024
2019-09-18T22:03:11.5254771Z ....................i............................................................................... 6600/9024
---
2019-09-18T22:07:24.3591493Z 
2019-09-18T22:07:24.3592519Z ---- [ui] ui/consts/const-size_of-cycle.rs stdout ----
2019-09-18T22:07:24.3592619Z diff of stderr:
2019-09-18T22:07:24.3592654Z 
2019-09-18T22:07:24.3592699Z 15 LL |     intrinsics::size_of::<T>()
2019-09-18T22:07:24.3592767Z 16    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-18T22:07:24.3593096Z 17 note: ...which requires const-evaluating + checking `std::intrinsics::size_of`...
2019-09-18T22:07:24.3593350Z +   --> $SRC_DIR/libcore/intrinsics.rs:LL:COL
2019-09-18T22:07:24.3593400Z +    |
2019-09-18T22:07:24.3593649Z + LL |     pub fn size_of<T>() -> usize;
2019-09-18T22:07:24.3593699Z +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-18T22:07:24.3593748Z 18    = note: ...which requires computing layout of `Foo`...
2019-09-18T22:07:24.3593828Z 19    = note: ...which requires normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All, def_id: None }, value: [u8; _] }`...
2019-09-18T22:07:24.3594178Z 20    = note: ...which again requires const-evaluating + checking `Foo::bytes::{{constant}}#0`, completing the cycle
2019-09-18T22:07:24.3594248Z 
2019-09-18T22:07:24.3594295Z The actual stderr differed from the expected stderr.
2019-09-18T22:07:24.3594655Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/const-size_of-cycle.stderr
2019-09-18T22:07:24.3594655Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/const-size_of-cycle.stderr
2019-09-18T22:07:24.3594925Z To update references, rerun the tests and pass the `--bless` flag
2019-09-18T22:07:24.3595194Z To only update this specific test, also pass `--test-args consts/const-size_of-cycle.rs`
2019-09-18T22:07:24.3595456Z error: 1 errors occurred comparing output.
2019-09-18T22:07:24.3595491Z status: exit code: 1
2019-09-18T22:07:24.3595491Z status: exit code: 1
2019-09-18T22:07:24.3596238Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-size_of-cycle.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/auxiliary" "-A" "unused"
2019-09-18T22:07:24.3596507Z ------------------------------------------
2019-09-18T22:07:24.3596534Z 
2019-09-18T22:07:24.3596696Z ------------------------------------------
2019-09-18T22:07:24.3596730Z stderr:
2019-09-18T22:07:24.3596730Z stderr:
2019-09-18T22:07:24.3596906Z ------------------------------------------
2019-09-18T22:07:24.3597106Z error[E0391]: cycle detected when const-evaluating + checking `Foo::bytes::{{constant}}#0`
2019-09-18T22:07:24.3597352Z    |
2019-09-18T22:07:24.3597352Z    |
2019-09-18T22:07:24.3597387Z LL |     bytes: [u8; std::mem::size_of::<Foo>()]
2019-09-18T22:07:24.3597471Z    |
2019-09-18T22:07:24.3597471Z    |
2019-09-18T22:07:24.3597668Z note: ...which requires const-evaluating + checking `Foo::bytes::{{constant}}#0`...
2019-09-18T22:07:24.3598268Z    |
2019-09-18T22:07:24.3598268Z    |
2019-09-18T22:07:24.3598320Z LL |     bytes: [u8; std::mem::size_of::<Foo>()]
2019-09-18T22:07:24.3598356Z    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-18T22:07:24.3598620Z note: ...which requires const-evaluating `Foo::bytes::{{constant}}#0`...
2019-09-18T22:07:24.3598836Z    |
2019-09-18T22:07:24.3598836Z    |
2019-09-18T22:07:24.3598870Z LL |     intrinsics::size_of::<T>()
2019-09-18T22:07:24.3598922Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-18T22:07:24.3599426Z note: ...which requires const-evaluating + checking `std::intrinsics::size_of`...
2019-09-18T22:07:24.3599872Z    |
2019-09-18T22:07:24.3599872Z    |
2019-09-18T22:07:24.3600223Z LL |     pub fn size_of<T>() -> usize;
2019-09-18T22:07:24.3600264Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-18T22:07:24.3600322Z    = note: ...which requires computing layout of `Foo`...
2019-09-18T22:07:24.3600381Z    = note: ...which requires normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All, def_id: None }, value: [u8; _] }`...
2019-09-18T22:07:24.3600643Z    = note: ...which again requires const-evaluating + checking `Foo::bytes::{{constant}}#0`, completing the cycle
2019-09-18T22:07:24.3600711Z note: cycle used when processing `Foo`
2019-09-18T22:07:24.3600957Z    |
2019-09-18T22:07:24.3601008Z LL | struct Foo {
2019-09-18T22:07:24.3601052Z    | ^^^^^^^^^^
2019-09-18T22:07:24.3601230Z 
---
2019-09-18T22:07:24.3635967Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-18T22:07:24.3636050Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-18T22:07:24.3648340Z 
2019-09-18T22:07:24.3648778Z 
2019-09-18T22:07:24.3656023Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-18T22:07:24.3679275Z 
2019-09-18T22:07:24.3679376Z 
2019-09-18T22:07:24.3740815Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-18T22:07:24.3740903Z Build completed unsuccessfully in 1:07:35
2019-09-18T22:07:24.3740903Z Build completed unsuccessfully in 1:07:35
2019-09-18T22:07:24.3740969Z == clock drift check ==
2019-09-18T22:07:24.3741005Z   local time: Wed Sep 18 22:07:24 UTC 2019
2019-09-18T22:07:24.4454219Z   network time: Wed, 18 Sep 2019 22:07:24 GMT
2019-09-18T22:07:24.4455177Z == end clock drift check ==
2019-09-18T22:07:25.2819406Z ##[error]Bash exited with code '1'.
2019-09-18T22:07:25.2858281Z ##[section]Starting: Checkout
2019-09-18T22:07:25.2860512Z ==============================================================================
2019-09-18T22:07:25.2860557Z Task         : Get sources
2019-09-18T22:07:25.2860610Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
