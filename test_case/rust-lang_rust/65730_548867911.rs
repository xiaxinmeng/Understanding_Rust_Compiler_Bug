plain
2019-11-01T16:58:05.5135176Z 
2019-11-01T16:58:05.5135721Z ---- [ui (nll)] ui/async-await/issues/issue-62097.rs stdout ----
2019-11-01T16:58:05.5135824Z diff of stderr:
2019-11-01T16:58:05.5135867Z 
2019-11-01T16:58:05.5136124Z - error: cannot infer an appropriate lifetime
2019-11-01T16:58:05.5136548Z -   --> $DIR/issue-62097.rs:12:31
2019-11-01T16:58:05.5136775Z + error[E0373]: closure may outlive the current function, but it borrows `self`, which is owned by the current function
2019-11-01T16:58:05.5137093Z +   --> $DIR/issue-62097.rs:13:13
2019-11-01T16:58:05.5137189Z 3    |
2019-11-01T16:58:05.5137422Z - LL |     pub async fn run_dummy_fn(&self) {
2019-11-01T16:58:05.5137666Z -    |                               ^^^^^ ...but this borrow...
2019-11-01T16:58:05.5137892Z 6 LL |         foo(|| self.bar()).await;
2019-11-01T16:58:05.5138208Z -    |         --- this return type evaluates to the `'static` lifetime...
2019-11-01T16:58:05.5138583Z +    |             ^^ ---- `self` is borrowed here
2019-11-01T16:58:05.5138878Z +    |             may outlive borrowed value `self`
2019-11-01T16:58:05.5138954Z 8    |
2019-11-01T16:58:05.5138954Z 8    |
2019-11-01T16:58:05.5139335Z - note: ...can't outlive the lifetime `'_` as defined on the method body at 12:31
2019-11-01T16:58:05.5139883Z -   --> $DIR/issue-62097.rs:12:31
2019-11-01T16:58:05.5140286Z + note: function requires argument type to outlive `'static`
2019-11-01T16:58:05.5140651Z +   --> $DIR/issue-62097.rs:13:9
2019-11-01T16:58:05.5140848Z 11    |
2019-11-01T16:58:05.5140953Z + LL |         foo(|| self.bar()).await;
2019-11-01T16:58:05.5141015Z +    |         ^^^^^^^^^^^^^^^^^^
2019-11-01T16:58:05.5141217Z + help: to force the closure to take ownership of `self` (and any other referenced variables), use the `move` keyword
2019-11-01T16:58:05.5141323Z +    |
2019-11-01T16:58:05.5141413Z + LL |         foo(move || self.bar()).await;
2019-11-01T16:58:05.5141567Z + 
2019-11-01T16:58:05.5141741Z + error[E0521]: borrowed data escapes outside of function
2019-11-01T16:58:05.5142017Z +   --> $DIR/issue-62097.rs:13:9
2019-11-01T16:58:05.5142097Z +    |
2019-11-01T16:58:05.5142097Z +    |
2019-11-01T16:58:05.5142288Z 12 LL |     pub async fn run_dummy_fn(&self) {
2019-11-01T16:58:05.5142560Z -    |                               ^
2019-11-01T16:58:05.5143000Z +    |                               ----- `self` is a reference that is only valid in the function body
2019-11-01T16:58:05.5143537Z + LL |         foo(|| self.bar()).await;
2019-11-01T16:58:05.5143933Z +    |         ^^^^^^^^^^^^^^^^^^ `self` escapes the function body here
2019-11-01T16:58:05.5144332Z - error: aborting due to previous error
2019-11-01T16:58:05.5144725Z + error: aborting due to 2 previous errors
2019-11-01T16:58:05.5144827Z 16 
2019-11-01T16:58:05.5145129Z + For more information about this error, try `rustc --explain E0373`.
2019-11-01T16:58:05.5145129Z + For more information about this error, try `rustc --explain E0373`.
2019-11-01T16:58:05.5145341Z 17 
2019-11-01T16:58:05.5145442Z 
2019-11-01T16:58:05.5145494Z 
2019-11-01T16:58:05.5145592Z The actual stderr differed from the expected stderr.
2019-11-01T16:58:05.5145996Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62097.nll/issue-62097.nll.stderr
2019-11-01T16:58:05.5146425Z To update references, rerun the tests and pass the `--bless` flag
2019-11-01T16:58:05.5147070Z To only update this specific test, also pass `--test-args async-await/issues/issue-62097.rs`
2019-11-01T16:58:05.5147564Z error: 1 errors occurred comparing output.
2019-11-01T16:58:05.5148012Z status: exit code: 1
2019-11-01T16:58:05.5148012Z status: exit code: 1
2019-11-01T16:58:05.5149128Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-62097.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62097.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62097.nll/auxiliary" "-A" "unused"
2019-11-01T16:58:05.5149900Z ------------------------------------------
2019-11-01T16:58:05.5150068Z 
2019-11-01T16:58:05.5150555Z ------------------------------------------
2019-11-01T16:58:05.5150761Z stderr:
2019-11-01T16:58:05.5150761Z stderr:
2019-11-01T16:58:05.5151203Z ------------------------------------------
2019-11-01T16:58:05.5151761Z error[E0373]: closure may outlive the current function, but it borrows `self`, which is owned by the current function
2019-11-01T16:58:05.5152106Z   --> /checkout/src/test/ui/async-await/issues/issue-62097.rs:13:13
2019-11-01T16:58:05.5152298Z    |
2019-11-01T16:58:05.5152389Z LL |         foo(|| self.bar()).await;
2019-11-01T16:58:05.5152645Z    |             ^^ ---- `self` is borrowed here
2019-11-01T16:58:05.5152941Z    |             may outlive borrowed value `self`
2019-11-01T16:58:05.5153000Z    |
2019-11-01T16:58:05.5153364Z note: function requires argument type to outlive `'static`
2019-11-01T16:58:05.5153753Z   --> /checkout/src/test/ui/async-await/issues/issue-62097.rs:13:9
2019-11-01T16:58:05.5153753Z   --> /checkout/src/test/ui/async-await/issues/issue-62097.rs:13:9
2019-11-01T16:58:05.5153951Z    |
2019-11-01T16:58:05.5154040Z LL |         foo(|| self.bar()).await;
2019-11-01T16:58:05.5154119Z    |         ^^^^^^^^^^^^^^^^^^
2019-11-01T16:58:05.5154252Z help: to force the closure to take ownership of `self` (and any other referenced variables), use the `move` keyword
2019-11-01T16:58:05.5154338Z    |
2019-11-01T16:58:05.5154406Z LL |         foo(move || self.bar()).await;
2019-11-01T16:58:05.5154524Z 
2019-11-01T16:58:05.5154579Z error[E0521]: borrowed data escapes outside of function
2019-11-01T16:58:05.5154999Z   --> /checkout/src/test/ui/async-await/issues/issue-62097.rs:13:9
2019-11-01T16:58:05.5155086Z    |
2019-11-01T16:58:05.5155086Z    |
2019-11-01T16:58:05.5155143Z LL |     pub async fn run_dummy_fn(&self) { //~ ERROR cannot infer
2019-11-01T16:58:05.5155581Z    |                               ----- `self` is a reference that is only valid in the function body
2019-11-01T16:58:05.5155778Z LL |         foo(|| self.bar()).await;
2019-11-01T16:58:05.5155886Z    |         ^^^^^^^^^^^^^^^^^^ `self` escapes the function body here
2019-11-01T16:58:05.5156028Z error: aborting due to 2 previous errors
2019-11-01T16:58:05.5156066Z 
2019-11-01T16:58:05.5156762Z For more information about this error, try `rustc --explain E0373`.
2019-11-01T16:58:05.5156927Z 
---
2019-11-01T16:58:05.5185258Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-01T16:58:05.5185350Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-01T16:58:05.5198939Z 
2019-11-01T16:58:05.5199010Z 
2019-11-01T16:58:05.5208964Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.40.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
2019-11-01T16:58:05.5209608Z 
2019-11-01T16:58:05.5209642Z 
2019-11-01T16:58:05.5213292Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-01T16:58:05.5213827Z Build completed unsuccessfully in 1:41:31
2019-11-01T16:58:05.5213827Z Build completed unsuccessfully in 1:41:31
2019-11-01T16:58:05.5264050Z == clock drift check ==
2019-11-01T16:58:05.5282100Z   local time: Fri Nov  1 16:58:05 UTC 2019
2019-11-01T16:58:05.8050040Z   network time: Fri, 01 Nov 2019 16:58:05 GMT
2019-11-01T16:58:05.8050248Z == end clock drift check ==
2019-11-01T16:58:06.8586814Z 
2019-11-01T16:58:06.8693187Z ##[error]Bash exited with code '1'.
2019-11-01T16:58:06.8730912Z ##[section]Starting: Checkout
2019-11-01T16:58:06.8733720Z ==============================================================================
2019-11-01T16:58:06.8733838Z Task         : Get sources
2019-11-01T16:58:06.8734152Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
