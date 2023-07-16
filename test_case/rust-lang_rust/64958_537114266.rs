plain
2019-10-01T16:19:41.9786459Z 25 
2019-10-01T16:19:41.9786487Z 
2019-10-01T16:19:41.9786549Z 
2019-10-01T16:19:41.9786605Z The actual stderr differed from the expected stderr.
2019-10-01T16:19:41.9786948Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/in-band-lifetimes/mismatched_trait_impl.nll/mismatched_trait_impl.nll.stderr
2019-10-01T16:19:41.9787230Z To update references, rerun the tests and pass the `--bless` flag
2019-10-01T16:19:41.9787528Z To only update this specific test, also pass `--test-args in-band-lifetimes/mismatched_trait_impl.rs`
2019-10-01T16:19:41.9787669Z error: 1 errors occurred comparing output.
2019-10-01T16:19:41.9787730Z status: exit code: 1
2019-10-01T16:19:41.9787730Z status: exit code: 1
2019-10-01T16:19:41.9788530Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/in-band-lifetimes/mismatched_trait_impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/in-band-lifetimes/mismatched_trait_impl.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/in-band-lifetimes/mismatched_trait_impl.nll/auxiliary" "-A" "unused"
2019-10-01T16:19:41.9789000Z ------------------------------------------
2019-10-01T16:19:41.9789044Z 
2019-10-01T16:19:41.9789265Z ------------------------------------------
2019-10-01T16:19:41.9790042Z stderr:
2019-10-01T16:19:41.9790042Z stderr:
2019-10-01T16:19:41.9790402Z ------------------------------------------
2019-10-01T16:19:41.9790763Z error[E0495]: cannot infer an appropriate lifetime for lifetime parameter 'a in generic type due to conflicting requirements
2019-10-01T16:19:41.9791219Z    |
2019-10-01T16:19:41.9791219Z    |
2019-10-01T16:19:41.9791660Z LL |     fn foo(&self, x: &u32, y: &'a u32) -> &'a u32 { //~ ERROR cannot infer
2019-10-01T16:19:41.9791875Z    |
2019-10-01T16:19:41.9791875Z    |
2019-10-01T16:19:41.9791983Z note: first, the lifetime cannot outlive the anonymous lifetime #2 defined on the method body at 9:5...
2019-10-01T16:19:41.9792429Z    |
2019-10-01T16:19:41.9792429Z    |
2019-10-01T16:19:41.9792720Z LL | /     fn foo(&self, x: &u32, y: &'a u32) -> &'a u32 { //~ ERROR cannot infer
2019-10-01T16:19:41.9792837Z LL | |         x //~ ERROR lifetime mismatch
2019-10-01T16:19:41.9793155Z    | |_____^
2019-10-01T16:19:41.9793155Z    | |_____^
2019-10-01T16:19:41.9793608Z note: ...but the lifetime must also be valid for the lifetime 'a as defined on the method body at 9:32...
2019-10-01T16:19:41.9793937Z    |
2019-10-01T16:19:41.9793937Z    |
2019-10-01T16:19:41.9794164Z LL |     fn foo(&self, x: &u32, y: &'a u32) -> &'a u32 { //~ ERROR cannot infer
2019-10-01T16:19:41.9794336Z    = note: ...so that the method type is compatible with trait:
2019-10-01T16:19:41.9794336Z    = note: ...so that the method type is compatible with trait:
2019-10-01T16:19:41.9794555Z            expected fn(&i32, &'a u32, &u32) -> &'a u32
2019-10-01T16:19:41.9794792Z               found fn(&i32, &u32, &u32) -> &u32
2019-10-01T16:19:41.9795073Z error: aborting due to previous error
2019-10-01T16:19:41.9795111Z 
2019-10-01T16:19:41.9795330Z For more information about this error, try `rustc --explain E0495`.
2019-10-01T16:19:41.9795403Z 
---
2019-10-01T16:19:41.9830995Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-01T16:19:41.9831166Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-01T16:19:41.9847030Z 
2019-10-01T16:19:41.9847133Z 
2019-10-01T16:19:41.9850124Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.40.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
2019-10-01T16:19:41.9851102Z 
2019-10-01T16:19:41.9851142Z 
2019-10-01T16:19:41.9855092Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-01T16:19:41.9855207Z Build completed unsuccessfully in 1:51:47
2019-10-01T16:19:41.9855207Z Build completed unsuccessfully in 1:51:47
2019-10-01T16:19:41.9907008Z == clock drift check ==
2019-10-01T16:19:41.9926360Z   local time: Tue Oct  1 16:19:41 UTC 2019
2019-10-01T16:19:42.0637918Z   network time: Tue, 01 Oct 2019 16:19:42 GMT
2019-10-01T16:19:42.0638058Z == end clock drift check ==
2019-10-01T16:19:42.9266946Z ##[error]Bash exited with code '1'.
2019-10-01T16:19:42.9303548Z ##[section]Starting: Upload CPU usage statistics
2019-10-01T16:19:42.9309838Z ==============================================================================
2019-10-01T16:19:42.9309977Z Task         : Bash
2019-10-01T16:19:42.9310073Z Description  : Run a Bash script on macOS, Linux, or Windows
