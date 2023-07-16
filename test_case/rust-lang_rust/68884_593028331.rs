plain
2020-03-01T01:35:34.4472141Z 
2020-03-01T01:35:34.4472568Z 34 LL | | }
2020-03-01T01:35:34.4473158Z 35    | |_^
2020-03-01T01:35:34.4473392Z 36    |
2020-03-01T01:35:34.4473962Z -    = note: hidden type `(&u8, &u8)` captures lifetime '_#8r
2020-03-01T01:35:34.4474793Z +    = note: hidden type `(&u8, &u8)` captures lifetime '_#4r
2020-03-01T01:35:34.4475531Z 39 error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2020-03-01T01:35:34.4476144Z 40   --> $DIR/ret-impl-trait-no-fg.rs:9:1
2020-03-01T01:35:34.4476607Z 
2020-03-01T01:35:34.4476825Z 48 LL | | }
2020-03-01T01:35:34.4476825Z 48 LL | | }
2020-03-01T01:35:34.4477068Z 49    | |_^
2020-03-01T01:35:34.4477286Z 50    |
2020-03-01T01:35:34.4477793Z -    = note: hidden type `(&u8, &u8)` captures lifetime '_#9r
2020-03-01T01:35:34.4478412Z +    = note: hidden type `(&u8, &u8)` captures lifetime '_#5r
2020-03-01T01:35:34.4479217Z 53 error: aborting due to 5 previous errors
2020-03-01T01:35:34.4479726Z 54 
2020-03-01T01:35:34.4479926Z 
2020-03-01T01:35:34.4480103Z 
2020-03-01T01:35:34.4480103Z 
2020-03-01T01:35:34.4480405Z The actual stderr differed from the expected stderr.
2020-03-01T01:35:34.4481358Z Actual stderr saved to /checkout/obj/build/i686-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/ret-impl-trait-no-fg/ret-impl-trait-no-fg.stderr
2020-03-01T01:35:34.4482231Z To update references, rerun the tests and pass the `--bless` flag
2020-03-01T01:35:34.4483325Z To only update this specific test, also pass `--test-args async-await/multiple-lifetimes/ret-impl-trait-no-fg.rs`
2020-03-01T01:35:34.4485186Z error: 1 errors occurred comparing output.
2020-03-01T01:35:34.4485384Z status: exit code: 1
2020-03-01T01:35:34.4485384Z status: exit code: 1
2020-03-01T01:35:34.4487214Z command: "/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/multiple-lifetimes/ret-impl-trait-no-fg.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/i686-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/ret-impl-trait-no-fg" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "--edition=2018" "-L" "/checkout/obj/build/i686-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/ret-impl-trait-no-fg/auxiliary"
2020-03-01T01:35:34.4488837Z ------------------------------------------
2020-03-01T01:35:34.4489160Z 
2020-03-01T01:35:34.4489900Z ------------------------------------------
2020-03-01T01:35:34.4490095Z stderr:
2020-03-01T01:35:34.4490095Z stderr:
2020-03-01T01:35:34.4490439Z ------------------------------------------
2020-03-01T01:35:34.4490697Z error: ambiguous lifetime bound in `impl Trait`
2020-03-01T01:35:34.4491228Z   --> /checkout/src/test/ui/async-await/multiple-lifetimes/ret-impl-trait-no-fg.rs:9:64
2020-03-01T01:35:34.4491512Z    |
2020-03-01T01:35:34.4491998Z LL | async fn async_ret_impl_trait<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a, 'b> {
2020-03-01T01:35:34.4492848Z    |                                                                ^^^^^^^^^^^^^^^^^^ neither `'a` nor `'b` outlives the other
2020-03-01T01:35:34.4493501Z    |
2020-03-01T01:35:34.4493746Z    = help: add #![feature(member_constraints)] to the crate attributes to enable
2020-03-01T01:35:34.4494132Z error: ambiguous lifetime bound in `impl Trait`
2020-03-01T01:35:34.4497451Z   --> /checkout/src/test/ui/async-await/multiple-lifetimes/ret-impl-trait-no-fg.rs:9:64
2020-03-01T01:35:34.4498168Z    |
2020-03-01T01:35:34.4498168Z    |
2020-03-01T01:35:34.4499247Z LL | async fn async_ret_impl_trait<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a, 'b> {
2020-03-01T01:35:34.4500353Z    |                                                                ^^^^^^^^^^^^^^^^^^ neither `'a` nor `'b` outlives the other
2020-03-01T01:35:34.4500684Z    |
2020-03-01T01:35:34.4500953Z    = help: add #![feature(member_constraints)] to the crate attributes to enable
2020-03-01T01:35:34.4501385Z error: ambiguous lifetime bound in `impl Trait`
2020-03-01T01:35:34.4501909Z   --> /checkout/src/test/ui/async-await/multiple-lifetimes/ret-impl-trait-no-fg.rs:9:64
2020-03-01T01:35:34.4502200Z    |
2020-03-01T01:35:34.4502200Z    |
2020-03-01T01:35:34.4502682Z LL | async fn async_ret_impl_trait<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a, 'b> {
2020-03-01T01:35:34.4503643Z    |                                                                ^^^^^^^^^^^^^^^^^^ the elided lifetimes here do not outlive one another
2020-03-01T01:35:34.4503954Z    |
2020-03-01T01:35:34.4504175Z    = help: add #![feature(member_constraints)] to the crate attributes to enable
2020-03-01T01:35:34.4504625Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2020-03-01T01:35:34.4505181Z   --> /checkout/src/test/ui/async-await/multiple-lifetimes/ret-impl-trait-no-fg.rs:9:1
2020-03-01T01:35:34.4505420Z    |
2020-03-01T01:35:34.4505420Z    |
2020-03-01T01:35:34.4505851Z LL | / async fn async_ret_impl_trait<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a, 'b> {
2020-03-01T01:35:34.4506183Z LL | |     //~^ ERROR ambiguous lifetime bound
2020-03-01T01:35:34.4506412Z LL | |     //~| ERROR ambiguous lifetime bound
2020-03-01T01:35:34.4506763Z LL | |     //~| ERROR ambiguous lifetime bound
2020-03-01T01:35:34.4507103Z LL | |     (a, b)
2020-03-01T01:35:34.4507242Z LL | | }
2020-03-01T01:35:34.4507378Z    | |_^
2020-03-01T01:35:34.4507494Z    |
2020-03-01T01:35:34.4507494Z    |
2020-03-01T01:35:34.4507885Z    = note: hidden type `(&u8, &u8)` captures lifetime '_#4r
2020-03-01T01:35:34.4511186Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2020-03-01T01:35:34.4512078Z   --> /checkout/src/test/ui/async-await/multiple-lifetimes/ret-impl-trait-no-fg.rs:9:1
2020-03-01T01:35:34.4512506Z    |
2020-03-01T01:35:34.4512506Z    |
2020-03-01T01:35:34.4513344Z LL | / async fn async_ret_impl_trait<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a, 'b> {
2020-03-01T01:35:34.4513856Z LL | |     //~^ ERROR ambiguous lifetime bound
2020-03-01T01:35:34.4514102Z LL | |     //~| ERROR ambiguous lifetime bound
2020-03-01T01:35:34.4514331Z LL | |     //~| ERROR ambiguous lifetime bound
2020-03-01T01:35:34.4514656Z LL | |     (a, b)
2020-03-01T01:35:34.4514814Z LL | | }
2020-03-01T01:35:34.4514935Z    | |_^
2020-03-01T01:35:34.4515067Z    |
2020-03-01T01:35:34.4515067Z    |
2020-03-01T01:35:34.4515418Z    = note: hidden type `(&u8, &u8)` captures lifetime '_#5r
2020-03-01T01:35:34.4515766Z error: aborting due to 5 previous errors
2020-03-01T01:35:34.4515923Z 
2020-03-01T01:35:34.4516264Z For more information about this error, try `rustc --explain E0700`.
2020-03-01T01:35:34.4535709Z 
---
2020-03-01T01:35:34.4538513Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-01T01:35:34.4538878Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-01T01:35:34.4539275Z 
2020-03-01T01:35:34.4539363Z 
2020-03-01T01:35:34.4543334Z command did not execute successfully: "/checkout/obj/build/i686-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/i686-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i686-unknown-linux-gnu" "--mode" "ui" "--target" "i686-unknown-linux-gnu" "--host" "i686-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/i686-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.1-rust-1.43.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-01T01:35:34.4545513Z 
2020-03-01T01:35:34.4545594Z 
2020-03-01T01:35:34.4546389Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/bootstrap --exclude src/test/rustdoc-js --exclude src/tools/error_index_generator --exclude src/tools/linkchecker
2020-03-01T01:35:34.4546874Z Build completed unsuccessfully in 1:37:22
2020-03-01T01:35:34.4546874Z Build completed unsuccessfully in 1:37:22
2020-03-01T01:35:34.4569664Z == clock drift check ==
2020-03-01T01:35:34.4583552Z   local time: Sun Mar  1 01:35:34 UTC 2020
2020-03-01T01:35:34.7367730Z   network time: Sun, 01 Mar 2020 01:35:34 GMT
2020-03-01T01:35:35.3767780Z 
2020-03-01T01:35:35.3767780Z 
2020-03-01T01:35:35.3835661Z ##[error]Bash exited with code '1'.
2020-03-01T01:35:35.3904746Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-03-01T01:35:35.3909761Z ==============================================================================
2020-03-01T01:35:35.3910137Z Task         : Get sources
2020-03-01T01:35:35.3911051Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
