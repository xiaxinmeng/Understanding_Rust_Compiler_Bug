plain
2019-12-03T18:36:36.3266324Z 42 
2019-12-03T18:36:36.3266586Z 
2019-12-03T18:36:36.3266719Z 
2019-12-03T18:36:36.3266904Z The actual stderr differed from the expected stderr.
2019-12-03T18:36:36.3267489Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closure-expected-type/expect-fn-supply-fn.nll/expect-fn-supply-fn.nll.stderr
2019-12-03T18:36:36.3268058Z To update references, rerun the tests and pass the `--bless` flag
2019-12-03T18:36:36.3268612Z To only update this specific test, also pass `--test-args closure-expected-type/expect-fn-supply-fn.rs`
2019-12-03T18:36:36.3269172Z error: 1 errors occurred comparing output.
2019-12-03T18:36:36.3269342Z status: exit code: 1
2019-12-03T18:36:36.3269342Z status: exit code: 1
2019-12-03T18:36:36.3270849Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closure-expected-type/expect-fn-supply-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closure-expected-type/expect-fn-supply-fn.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closure-expected-type/expect-fn-supply-fn.nll/auxiliary" "-A" "unused"
2019-12-03T18:36:36.3271788Z ------------------------------------------
2019-12-03T18:36:36.3271968Z 
2019-12-03T18:36:36.3272365Z ------------------------------------------
2019-12-03T18:36:36.3272573Z stderr:
2019-12-03T18:36:36.3272573Z stderr:
2019-12-03T18:36:36.3272960Z ------------------------------------------
2019-12-03T18:36:36.3273187Z error[E0631]: type mismatch in closure arguments
2019-12-03T18:36:36.3273630Z   --> /checkout/src/test/ui/closure-expected-type/expect-fn-supply-fn.rs:30:5
2019-12-03T18:36:36.3274059Z    |
2019-12-03T18:36:36.3274228Z LL | fn with_closure_expecting_fn_with_free_region<F>(_: F)
2019-12-03T18:36:36.3274757Z    |    ------------------------------------------
2019-12-03T18:36:36.3275188Z LL |     where F: for<'a> FnOnce(fn(&'a u32), &i32)
2019-12-03T18:36:36.3275730Z    |                      ------------------------- required by this bound in `with_closure_expecting_fn_with_free_region`
2019-12-03T18:36:36.3276667Z ...
2019-12-03T18:36:36.3276771Z LL |     with_closure_expecting_fn_with_free_region(|x: fn(&u32), y| {});
2019-12-03T18:36:36.3277213Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ---------------- found signature of `fn(for<'r> fn(&'r u32), _) -> _`
2019-12-03T18:36:36.3277329Z    |     |
2019-12-03T18:36:36.3277586Z    |     expected signature of `fn(fn(&'a u32), &i32) -> _`
2019-12-03T18:36:36.3277743Z error[E0631]: type mismatch in closure arguments
2019-12-03T18:36:36.3278053Z   --> /checkout/src/test/ui/closure-expected-type/expect-fn-supply-fn.rs:37:5
2019-12-03T18:36:36.3278315Z    |
2019-12-03T18:36:36.3278315Z    |
2019-12-03T18:36:36.3278399Z LL | fn with_closure_expecting_fn_with_bound_region<F>(_: F)
2019-12-03T18:36:36.3278663Z    |    -------------------------------------------
2019-12-03T18:36:36.3278738Z LL |     where F: FnOnce(fn(&u32), &i32)
2019-12-03T18:36:36.3279062Z    |              ---------------------- required by this bound in `with_closure_expecting_fn_with_bound_region`
2019-12-03T18:36:36.3279153Z ...
2019-12-03T18:36:36.3279429Z LL |     with_closure_expecting_fn_with_bound_region(|x: fn(&'x u32), y| {});
2019-12-03T18:36:36.3279752Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ------------------- found signature of `fn(fn(&'x u32), _) -> _`
2019-12-03T18:36:36.3279864Z    |     |
2019-12-03T18:36:36.3282153Z    |     expected signature of `fn(for<'r> fn(&'r u32), &i32) -> _`
2019-12-03T18:36:36.3282464Z error[E0631]: type mismatch in closure arguments
2019-12-03T18:36:36.3282919Z   --> /checkout/src/test/ui/closure-expected-type/expect-fn-supply-fn.rs:46:5
2019-12-03T18:36:36.3283022Z    |
2019-12-03T18:36:36.3283022Z    |
2019-12-03T18:36:36.3283096Z LL | fn with_closure_expecting_fn_with_bound_region<F>(_: F)
2019-12-03T18:36:36.3283380Z    |    -------------------------------------------
2019-12-03T18:36:36.3283462Z LL |     where F: FnOnce(fn(&u32), &i32)
2019-12-03T18:36:36.3283809Z    |              ---------------------- required by this bound in `with_closure_expecting_fn_with_bound_region`
2019-12-03T18:36:36.3283907Z ...
2019-12-03T18:36:36.3284355Z LL |     with_closure_expecting_fn_with_bound_region(|x: Foo<'_>, y| {
2019-12-03T18:36:36.3284928Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ --------------- found signature of `for<'r> fn(fn(&'r u32), _) -> _`
2019-12-03T18:36:36.3285023Z    |     |
2019-12-03T18:36:36.3285307Z    |     expected signature of `fn(for<'r> fn(&'r u32), &i32) -> _`
2019-12-03T18:36:36.3285451Z error: aborting due to 3 previous errors
2019-12-03T18:36:36.3285685Z 
2019-12-03T18:36:36.3285946Z For more information about this error, try `rustc --explain E0631`.
2019-12-03T18:36:36.3286018Z 
---
2019-12-03T18:36:36.3309895Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-03T18:36:36.3310413Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-03T18:36:36.3325840Z 
2019-12-03T18:36:36.3325940Z 
2019-12-03T18:36:36.3328500Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.41.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
2019-12-03T18:36:36.3329202Z 
2019-12-03T18:36:36.3329240Z 
2019-12-03T18:36:36.3335776Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-03T18:36:36.3335908Z Build completed unsuccessfully in 1:52:48
2019-12-03T18:36:36.3335908Z Build completed unsuccessfully in 1:52:48
2019-12-03T18:36:36.3414223Z == clock drift check ==
2019-12-03T18:36:36.3432020Z   local time: Tue Dec  3 18:36:36 UTC 2019
2019-12-03T18:36:36.6217693Z   network time: Tue, 03 Dec 2019 18:36:36 GMT
2019-12-03T18:36:36.6220214Z == end clock drift check ==
2019-12-03T18:36:38.4803471Z 
2019-12-03T18:36:38.4881142Z ##[error]Bash exited with code '1'.
2019-12-03T18:36:38.4922623Z ##[section]Starting: Checkout
2019-12-03T18:36:38.4924979Z ==============================================================================
2019-12-03T18:36:38.4925136Z Task         : Get sources
2019-12-03T18:36:38.4925233Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
