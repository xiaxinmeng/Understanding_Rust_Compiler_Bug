plain
2019-11-09T04:40:56.2961713Z 
2019-11-09T04:40:56.2962665Z ---- [ui (nll)] ui/async-await/issues/issue-62097.rs stdout ----
2019-11-09T04:40:56.2963067Z diff of stderr:
2019-11-09T04:40:56.2963296Z 
2019-11-09T04:40:56.2963804Z - error: cannot infer an appropriate lifetime
2019-11-09T04:40:56.2964386Z -   --> $DIR/issue-62097.rs:12:31
2019-11-09T04:40:56.2964950Z + error[E0373]: closure may outlive the current function, but it borrows `self`, which is owned by the current function
2019-11-09T04:40:56.2965453Z +   --> $DIR/issue-62097.rs:13:13
2019-11-09T04:40:56.2965816Z 3    |
2019-11-09T04:40:56.2966302Z - LL |     pub async fn run_dummy_fn(&self) {
2019-11-09T04:40:56.2966848Z -    |                               ^^^^^ ...but this borrow...
2019-11-09T04:40:56.2967187Z 6 LL |         foo(|| self.bar()).await;
2019-11-09T04:40:56.2967702Z -    |         --- this return type evaluates to the `'static` lifetime...
2019-11-09T04:40:56.2968265Z +    |             ^^ ---- `self` is borrowed here
2019-11-09T04:40:56.2969283Z +    |             may outlive borrowed value `self`
2019-11-09T04:40:56.2969550Z 8    |
2019-11-09T04:40:56.2969550Z 8    |
2019-11-09T04:40:56.2970099Z - note: ...can't outlive the lifetime `'_` as defined on the method body at 12:31
2019-11-09T04:40:56.2970639Z -   --> $DIR/issue-62097.rs:12:31
2019-11-09T04:40:56.2971514Z + note: function requires argument type to outlive `'static`
2019-11-09T04:40:56.2972575Z +   --> $DIR/issue-62097.rs:13:9
2019-11-09T04:40:56.2972910Z 11    |
2019-11-09T04:40:56.2973162Z + LL |         foo(|| self.bar()).await;
2019-11-09T04:40:56.2973426Z +    |         ^^^^^^^^^^^^^^^^^^
2019-11-09T04:40:56.2973879Z + help: to force the closure to take ownership of `self` (and any other referenced variables), use the `move` keyword
2019-11-09T04:40:56.2974208Z +    |
2019-11-09T04:40:56.2974465Z + LL |         foo(move || self.bar()).await;
2019-11-09T04:40:56.2974936Z + 
2019-11-09T04:40:56.2975186Z + error[E0521]: borrowed data escapes outside of function
2019-11-09T04:40:56.2975675Z +   --> $DIR/issue-62097.rs:13:9
2019-11-09T04:40:56.2975991Z +    |
2019-11-09T04:40:56.2975991Z +    |
2019-11-09T04:40:56.2976248Z 12 LL |     pub async fn run_dummy_fn(&self) {
2019-11-09T04:40:56.2976855Z -    |                               ^
2019-11-09T04:40:56.2977448Z +    |                               ----- `self` is a reference that is only valid in the function body
2019-11-09T04:40:56.2978262Z + LL |         foo(|| self.bar()).await;
2019-11-09T04:40:56.2978546Z +    |         ^^^^^^^^^^^^^^^^^^ `self` escapes the function body here
2019-11-09T04:40:56.2979440Z - error: aborting due to previous error
2019-11-09T04:40:56.2979671Z + error: aborting due to 2 previous errors
2019-11-09T04:40:56.2979848Z 16 
2019-11-09T04:40:56.2980285Z + For more information about this error, try `rustc --explain E0373`.
2019-11-09T04:40:56.2980285Z + For more information about this error, try `rustc --explain E0373`.
2019-11-09T04:40:56.2980491Z 17 
2019-11-09T04:40:56.2980635Z 
2019-11-09T04:40:56.2980760Z 
2019-11-09T04:40:56.2980948Z The actual stderr differed from the expected stderr.
2019-11-09T04:40:56.2981623Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62097.nll/issue-62097.nll.stderr
2019-11-09T04:40:56.2982123Z To update references, rerun the tests and pass the `--bless` flag
2019-11-09T04:40:56.2982823Z To only update this specific test, also pass `--test-args async-await/issues/issue-62097.rs`
2019-11-09T04:40:56.2983189Z error: 1 errors occurred comparing output.
2019-11-09T04:40:56.2983367Z status: exit code: 1
2019-11-09T04:40:56.2983367Z status: exit code: 1
2019-11-09T04:40:56.2984554Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-62097.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62097.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62097.nll/auxiliary" "-A" "unused"
2019-11-09T04:40:56.2985380Z ------------------------------------------
2019-11-09T04:40:56.2985554Z 
2019-11-09T04:40:56.2985938Z ------------------------------------------
2019-11-09T04:40:56.2986133Z stderr:
2019-11-09T04:40:56.2986133Z stderr:
2019-11-09T04:40:56.2986499Z ------------------------------------------
2019-11-09T04:40:56.2986761Z error[E0373]: closure may outlive the current function, but it borrows `self`, which is owned by the current function
2019-11-09T04:40:56.2987203Z   --> /checkout/src/test/ui/async-await/issues/issue-62097.rs:13:13
2019-11-09T04:40:56.2987611Z    |
2019-11-09T04:40:56.2987945Z LL |         foo(|| self.bar()).await;
2019-11-09T04:40:56.2988341Z    |             ^^ ---- `self` is borrowed here
2019-11-09T04:40:56.2988730Z    |             may outlive borrowed value `self`
2019-11-09T04:40:56.2988908Z    |
2019-11-09T04:40:56.2989277Z note: function requires argument type to outlive `'static`
2019-11-09T04:40:56.2989726Z   --> /checkout/src/test/ui/async-await/issues/issue-62097.rs:13:9
2019-11-09T04:40:56.2989726Z   --> /checkout/src/test/ui/async-await/issues/issue-62097.rs:13:9
2019-11-09T04:40:56.2989946Z    |
2019-11-09T04:40:56.2990102Z LL |         foo(|| self.bar()).await;
2019-11-09T04:40:56.2990285Z    |         ^^^^^^^^^^^^^^^^^^
2019-11-09T04:40:56.2993102Z help: to force the closure to take ownership of `self` (and any other referenced variables), use the `move` keyword
2019-11-09T04:40:56.2993339Z    |
2019-11-09T04:40:56.2993521Z LL |         foo(move || self.bar()).await;
2019-11-09T04:40:56.2993970Z 
2019-11-09T04:40:56.2994209Z error[E0521]: borrowed data escapes outside of function
2019-11-09T04:40:56.2994750Z   --> /checkout/src/test/ui/async-await/issues/issue-62097.rs:13:9
2019-11-09T04:40:56.2994987Z    |
2019-11-09T04:40:56.2994987Z    |
2019-11-09T04:40:56.2995163Z LL |     pub async fn run_dummy_fn(&self) { //~ ERROR cannot infer
2019-11-09T04:40:56.2995635Z    |                               ----- `self` is a reference that is only valid in the function body
2019-11-09T04:40:56.2995857Z LL |         foo(|| self.bar()).await;
2019-11-09T04:40:56.2996056Z    |         ^^^^^^^^^^^^^^^^^^ `self` escapes the function body here
2019-11-09T04:40:56.2996450Z error: aborting due to 2 previous errors
2019-11-09T04:40:56.2996595Z 
2019-11-09T04:40:56.2997020Z For more information about this error, try `rustc --explain E0373`.
2019-11-09T04:40:56.2997339Z 
---
2019-11-09T04:40:56.3000324Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-09T04:40:56.3000571Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-09T04:40:56.3010630Z 
2019-11-09T04:40:56.3010961Z 
2019-11-09T04:40:56.3013799Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.40.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
2019-11-09T04:40:56.3015083Z 
2019-11-09T04:40:56.3015242Z 
2019-11-09T04:40:56.3020297Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-09T04:40:56.3020874Z Build completed unsuccessfully in 1:41:07
2019-11-09T04:40:56.3020874Z Build completed unsuccessfully in 1:41:07
2019-11-09T04:40:56.3071498Z == clock drift check ==
2019-11-09T04:40:56.3086770Z   local time: Sat Nov  9 04:40:56 UTC 2019
2019-11-09T04:40:56.5817622Z   network time: Sat, 09 Nov 2019 04:40:56 GMT
2019-11-09T04:40:56.5821586Z == end clock drift check ==
2019-11-09T04:40:57.8004730Z 
2019-11-09T04:40:57.8103658Z ##[error]Bash exited with code '1'.
2019-11-09T04:40:57.8148314Z ##[section]Starting: Checkout
2019-11-09T04:40:57.8151098Z ==============================================================================
2019-11-09T04:40:57.8151212Z Task         : Get sources
2019-11-09T04:40:57.8151478Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
