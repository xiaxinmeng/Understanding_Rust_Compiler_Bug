plain
2019-09-30T21:25:21.7938736Z 25 
2019-09-30T21:25:21.7938935Z 
2019-09-30T21:25:21.7939093Z 
2019-09-30T21:25:21.7939270Z The actual stderr differed from the expected stderr.
2019-09-30T21:25:21.7939908Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/in-band-lifetimes/mismatched_trait_impl.nll/mismatched_trait_impl.nll.stderr
2019-09-30T21:25:21.7940554Z To update references, rerun the tests and pass the `--bless` flag
2019-09-30T21:25:21.7941154Z To only update this specific test, also pass `--test-args in-band-lifetimes/mismatched_trait_impl.rs`
2019-09-30T21:25:21.7941565Z error: 1 errors occurred comparing output.
2019-09-30T21:25:21.7941927Z status: exit code: 1
2019-09-30T21:25:21.7941927Z status: exit code: 1
2019-09-30T21:25:21.7943206Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/in-band-lifetimes/mismatched_trait_impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/in-band-lifetimes/mismatched_trait_impl.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/in-band-lifetimes/mismatched_trait_impl.nll/auxiliary" "-A" "unused"
2019-09-30T21:25:21.7943889Z ------------------------------------------
2019-09-30T21:25:21.7944053Z 
2019-09-30T21:25:21.7944373Z ------------------------------------------
2019-09-30T21:25:21.7944559Z stderr:
2019-09-30T21:25:21.7944559Z stderr:
2019-09-30T21:25:21.7944884Z ------------------------------------------
2019-09-30T21:25:21.7945526Z error[E0495]: cannot infer an appropriate lifetime for lifetime parameter 'a in generic type due to conflicting requirements
2019-09-30T21:25:21.7946543Z    |
2019-09-30T21:25:21.7946543Z    |
2019-09-30T21:25:21.7947124Z LL |     fn foo(&self, x: &u32, y: &'a u32) -> &'a u32 { //~ ERROR cannot infer
2019-09-30T21:25:21.7947501Z    |
2019-09-30T21:25:21.7947501Z    |
2019-09-30T21:25:21.7947700Z note: first, the lifetime cannot outlive the anonymous lifetime #2 defined on the method body at 9:5...
2019-09-30T21:25:21.7949175Z    |
2019-09-30T21:25:21.7949175Z    |
2019-09-30T21:25:21.7950097Z LL | /     fn foo(&self, x: &u32, y: &'a u32) -> &'a u32 { //~ ERROR cannot infer
2019-09-30T21:25:21.7950911Z LL | |         x //~ ERROR lifetime mismatch
2019-09-30T21:25:21.7951077Z    | |_____^
2019-09-30T21:25:21.7951077Z    | |_____^
2019-09-30T21:25:21.7951649Z note: ...but the lifetime must also be valid for the lifetime 'a as defined on the method body at 9:32...
2019-09-30T21:25:21.7951992Z    |
2019-09-30T21:25:21.7951992Z    |
2019-09-30T21:25:21.7952240Z LL |     fn foo(&self, x: &u32, y: &'a u32) -> &'a u32 { //~ ERROR cannot infer
2019-09-30T21:25:21.7952393Z    = note: ...so that the method type is compatible with trait:
2019-09-30T21:25:21.7952393Z    = note: ...so that the method type is compatible with trait:
2019-09-30T21:25:21.7952636Z            expected fn(&i32, &'a u32, &u32) -> &'a u32
2019-09-30T21:25:21.7953078Z               found fn(&i32, &u32, &u32) -> &u32
2019-09-30T21:25:21.7953189Z error: aborting due to previous error
2019-09-30T21:25:21.7953305Z 
2019-09-30T21:25:21.7953569Z For more information about this error, try `rustc --explain E0495`.
2019-09-30T21:25:21.7953614Z 
---
2019-09-30T21:25:21.7974217Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-09-30T21:25:21.7974325Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-30T21:25:21.7993656Z 
2019-09-30T21:25:21.7993734Z 
2019-09-30T21:25:21.7995645Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.40.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
2019-09-30T21:25:21.7996605Z 
2019-09-30T21:25:21.7996639Z 
2019-09-30T21:25:22.4341761Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-30T21:25:22.4342609Z Build completed unsuccessfully in 2:00:43
2019-09-30T21:25:22.4342609Z Build completed unsuccessfully in 2:00:43
2019-09-30T21:25:22.4342933Z == clock drift check ==
2019-09-30T21:25:22.4343005Z   local time: Mon Sep 30 21:25:21 UTC 2019
2019-09-30T21:25:22.4343384Z   network time: Mon, 30 Sep 2019 21:25:21 GMT
2019-09-30T21:25:22.4343663Z == end clock drift check ==
2019-09-30T21:25:22.7931050Z ##[error]Bash exited with code '1'.
2019-09-30T21:25:22.7983075Z ##[section]Starting: Upload CPU usage statistics
2019-09-30T21:25:22.7991993Z ==============================================================================
2019-09-30T21:25:22.7992113Z Task         : Bash
2019-09-30T21:25:22.7992191Z Description  : Run a Bash script on macOS, Linux, or Windows
