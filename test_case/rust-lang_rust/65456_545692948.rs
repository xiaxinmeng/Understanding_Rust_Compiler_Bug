plain
2019-10-23T23:23:54.6965443Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-23T23:23:55.5626414Z ##[command]git config gc.auto 0
2019-10-23T23:23:55.5630391Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-23T23:23:55.5632295Z ##[command]git config --get-all http.proxy
2019-10-23T23:23:55.5635854Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65456/merge:refs/remotes/pull/65456/merge
---
2019-10-24T00:29:59.9525265Z .................................................................................................... 1600/9239
2019-10-24T00:30:05.7803304Z .................................................................................................... 1700/9239
2019-10-24T00:30:19.2702093Z ...................................................i...............i................................ 1800/9239
2019-10-24T00:30:28.2359919Z .................................................................................................... 1900/9239
2019-10-24T00:30:43.5786692Z .........................................iiiii...................................................... 2000/9239
2019-10-24T00:30:55.0269701Z .................................................................................................... 2200/9239
2019-10-24T00:30:57.7561625Z .................................................................................................... 2300/9239
2019-10-24T00:31:02.4096819Z .................................................................................................... 2400/9239
2019-10-24T00:31:27.3380617Z .................................................................................................... 2500/9239
---
2019-10-24T00:34:33.1626082Z .............................................i...............i...................................... 4800/9239
2019-10-24T00:34:43.8222190Z .................................................................................................... 4900/9239
2019-10-24T00:34:52.0477161Z .................................................................................................... 5000/9239
2019-10-24T00:34:59.4597890Z .................................................................................................... 5100/9239
2019-10-24T00:35:09.9685442Z .............................................ii.ii................................F................. 5200/9239
2019-10-24T00:35:20.5180224Z .................................................................................................... 5400/9239
2019-10-24T00:35:30.8478584Z .................................................................................................... 5500/9239
2019-10-24T00:35:38.6799594Z ............i....................................................................................... 5600/9239
2019-10-24T00:35:44.5606595Z .................................................................................................... 5700/9239
2019-10-24T00:35:44.5606595Z .................................................................................................... 5700/9239
2019-10-24T00:35:57.4439424Z .................................................................................................... 5800/9239
2019-10-24T00:36:10.0856162Z .........ii...i..ii...........i..................................................................... 5900/9239
2019-10-24T00:36:33.7104230Z .................................................................................................... 6100/9239
2019-10-24T00:36:40.7518462Z .................................................................................................... 6200/9239
2019-10-24T00:36:40.7518462Z .................................................................................................... 6200/9239
2019-10-24T00:36:55.6931837Z ...............................i..ii................................................................ 6300/9239
2019-10-24T00:37:17.7376672Z .................................................................................................i.. 6500/9239
2019-10-24T00:37:20.1500559Z .................................................................................................... 6600/9239
2019-10-24T00:37:22.6209917Z ........................................................................i........................... 6700/9239
2019-10-24T00:37:25.7245575Z .................................................................................................... 6800/9239
---
2019-10-24T00:41:48.8349237Z 
2019-10-24T00:41:48.8350170Z ---- [ui] ui/kindck/kindck-inherited-copy-bound.rs#object_safe_for_dispatch stdout ----
2019-10-24T00:41:48.8350260Z diff of stderr:
2019-10-24T00:41:48.8350365Z 
2019-10-24T00:41:48.8351510Z - error[E0277]: the trait bound `std::boxed::Box<{integer}>: std::marker::Copy` is not satisfied
2019-10-24T00:41:48.8351601Z + error[E0277]: the trait bound `std::boxed::Box<{integer}>: Foo` is not satisfied
2019-10-24T00:41:48.8351920Z 2   --> $DIR/kindck-inherited-copy-bound.rs:21:16
2019-10-24T00:41:48.8351970Z 3    |
2019-10-24T00:41:48.8352015Z 4 LL | fn take_param<T:Foo>(foo: &T) { }
2019-10-24T00:41:48.8352091Z 
2019-10-24T00:41:48.8352138Z The actual stderr differed from the expected stderr.
2019-10-24T00:41:48.8352138Z The actual stderr differed from the expected stderr.
2019-10-24T00:41:48.8352525Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kindck/kindck-inherited-copy-bound.object_safe_for_dispatch/kindck-inherited-copy-bound.object_safe_for_dispatch.stderr
2019-10-24T00:41:48.8352801Z To update references, rerun the tests and pass the `--bless` flag
2019-10-24T00:41:48.8353090Z To only update this specific test, also pass `--test-args kindck/kindck-inherited-copy-bound.rs`
2019-10-24T00:41:48.8353133Z 
2019-10-24T00:41:48.8353200Z error in revision `object_safe_for_dispatch`: 1 errors occurred comparing output.
2019-10-24T00:41:48.8353248Z status: exit code: 1
2019-10-24T00:41:48.8354091Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/kindck/kindck-inherited-copy-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "object_safe_for_dispatch" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kindck/kindck-inherited-copy-bound.object_safe_for_dispatch" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kindck/kindck-inherited-copy-bound.object_safe_for_dispatch/auxiliary" "-A" "unused"
2019-10-24T00:41:48.8354448Z ------------------------------------------
2019-10-24T00:41:48.8354506Z 
2019-10-24T00:41:48.8354726Z ------------------------------------------
2019-10-24T00:41:48.8354770Z stderr:
2019-10-24T00:41:48.8354770Z stderr:
2019-10-24T00:41:48.8354985Z ------------------------------------------
2019-10-24T00:41:48.8355053Z error[E0277]: the trait bound `std::boxed::Box<{integer}>: Foo` is not satisfied
2019-10-24T00:41:48.8355312Z   --> /checkout/src/test/ui/kindck/kindck-inherited-copy-bound.rs:21:16
2019-10-24T00:41:48.8355378Z    |
2019-10-24T00:41:48.8355424Z LL | fn take_param<T:Foo>(foo: &T) { }
2019-10-24T00:41:48.8355972Z    |    ----------   --- required by this bound in `take_param`
2019-10-24T00:41:48.8356015Z ...
2019-10-24T00:41:48.8356075Z LL |     take_param(&x); //[curr]~ ERROR E0277
2019-10-24T00:41:48.8356127Z    |                ^^ the trait `std::marker::Copy` is not implemented for `std::boxed::Box<{integer}>`
2019-10-24T00:41:48.8356315Z    |
2019-10-24T00:41:48.8356384Z    = note: required because of the requirements on the impl of `Foo` for `std::boxed::Box<{integer}>`
2019-10-24T00:41:48.8356468Z error[E0038]: the trait `Foo` cannot be made into an object
2019-10-24T00:41:48.8356786Z   --> /checkout/src/test/ui/kindck/kindck-inherited-copy-bound.rs:28:13
2019-10-24T00:41:48.8357023Z    |
2019-10-24T00:41:48.8357023Z    |
2019-10-24T00:41:48.8357082Z LL |     let z = &x as &dyn Foo;
2019-10-24T00:41:48.8357151Z    |             ^^ the trait `Foo` cannot be made into an object
2019-10-24T00:41:48.8357196Z    |
2019-10-24T00:41:48.8357240Z    = note: the trait cannot require that `Self : Sized`
2019-10-24T00:41:48.8357297Z    = note: required because of the requirements on the impl of `std::ops::CoerceUnsized<&dyn Foo>` for `&std::boxed::Box<i32>`
2019-10-24T00:41:48.8357369Z    = note: required by cast to type `&dyn Foo`
2019-10-24T00:41:48.8357442Z error: aborting due to 2 previous errors
2019-10-24T00:41:48.8357481Z 
2019-10-24T00:41:48.8357542Z Some errors have detailed explanations: E0038, E0277.
2019-10-24T00:41:48.8357941Z For more information about an error, try `rustc --explain E0038`.
---
2019-10-24T00:41:48.8399233Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-24T00:41:48.8399595Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-24T00:41:48.8417153Z 
2019-10-24T00:41:48.8417344Z 
2019-10-24T00:41:48.8423565Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-24T00:41:48.8424898Z 
2019-10-24T00:41:48.8424976Z 
2019-10-24T00:41:48.8435431Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-24T00:41:48.8435537Z Build completed unsuccessfully in 1:11:11
2019-10-24T00:41:48.8435537Z Build completed unsuccessfully in 1:11:11
2019-10-24T00:41:48.8492690Z == clock drift check ==
2019-10-24T00:41:48.8509491Z   local time: Thu Oct 24 00:41:48 UTC 2019
2019-10-24T00:41:49.0160389Z   network time: Thu, 24 Oct 2019 00:41:49 GMT
2019-10-24T00:41:49.0164606Z == end clock drift check ==
2019-10-24T00:41:50.0323372Z 
2019-10-24T00:41:50.0478977Z ##[error]Bash exited with code '1'.
2019-10-24T00:41:50.0591617Z ##[section]Starting: Checkout
2019-10-24T00:41:50.0593471Z ==============================================================================
2019-10-24T00:41:50.0593535Z Task         : Get sources
2019-10-24T00:41:50.0593583Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
