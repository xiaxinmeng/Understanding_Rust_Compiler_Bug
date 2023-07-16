plain
2019-11-26T20:11:20.4421751Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-26T20:11:20.4625838Z ##[command]git config gc.auto 0
2019-11-26T20:11:20.4692893Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-26T20:11:21.2759980Z ##[command]git config --get-all http.proxy
2019-11-26T20:11:21.2773014Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66020/merge:refs/remotes/pull/66020/merge
---
2019-11-26T21:10:07.2822846Z .................................................................................................... 1600/9295
2019-11-26T21:10:12.1513065Z .................................................................................................... 1700/9295
2019-11-26T21:10:25.1693475Z ................................i................................................................... 1800/9295
2019-11-26T21:10:32.5180085Z .................................................................................................... 1900/9295
2019-11-26T21:10:46.3036549Z .................iiiii.............................................................................. 2000/9295
2019-11-26T21:10:56.4023748Z .................................................................................................... 2200/9295
2019-11-26T21:10:59.3094636Z .................................................................................................... 2300/9295
2019-11-26T21:11:04.1658737Z .................................................................................................... 2400/9295
2019-11-26T21:11:25.6784098Z .................................................................................................... 2500/9295
---
2019-11-26T21:14:08.4996944Z ..................i...............i................................................................. 4800/9295
2019-11-26T21:14:19.0162214Z .................................................................................................... 4900/9295
2019-11-26T21:14:24.8773347Z .................................................................................................... 5000/9295
2019-11-26T21:14:34.5404673Z .................................................................................................... 5100/9295
2019-11-26T21:14:41.0061495Z .......................ii.ii...........i............................................................ 5200/9295
2019-11-26T21:14:50.2665803Z .................................................................................................... 5400/9295
2019-11-26T21:15:01.1677057Z .................................................................................................... 5500/9295
2019-11-26T21:15:08.0803787Z .....i.............................................................................................. 5600/9295
2019-11-26T21:15:14.5565882Z .................................................................................................... 5700/9295
2019-11-26T21:15:14.5565882Z .................................................................................................... 5700/9295
2019-11-26T21:15:25.0742817Z ...........................................................................................ii...i... 5800/9295
2019-11-26T21:15:37.6419327Z ii..........i....................................................................................... 5900/9295
2019-11-26T21:15:47.4597532Z .................................................................................................... 6000/9295
2019-11-26T21:15:56.0004255Z .................................................................................................... 6100/9295
2019-11-26T21:16:02.4634668Z .................................................................................................... 6200/9295
2019-11-26T21:16:16.9079137Z ..............i..ii................................................................................. 6300/9295
2019-11-26T21:16:36.5581967Z ..................................................................................i................. 6500/9295
2019-11-26T21:16:38.8654268Z .................................................................................................... 6600/9295
2019-11-26T21:16:41.1840010Z .........................................................................i.......................... 6700/9295
2019-11-26T21:16:44.0219213Z .................................................................................................... 6800/9295
---
2019-11-26T21:21:32.7519563Z error: /checkout/src/test/ui/associated-types/associated-types-overridden-binding.rs:7: expected error not found: type annotations needed
2019-11-26T21:21:32.7519728Z 
2019-11-26T21:21:32.7519871Z error: 0 unexpected errors found, 1 expected errors not found
2019-11-26T21:21:32.7520024Z status: exit code: 1
2019-11-26T21:21:32.7520988Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-overridden-binding.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-overridden-binding" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-overridden-binding/auxiliary" "-A" "unused"
2019-11-26T21:21:32.7521680Z not found errors (from test file): [
2019-11-26T21:21:32.7521995Z         line_num: 7,
2019-11-26T21:21:32.7522132Z         kind: Some(
2019-11-26T21:21:32.7522269Z             Error,
2019-11-26T21:21:32.7522426Z         ),
---
2019-11-26T21:21:32.7523713Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-26T21:21:32.7523855Z 
2019-11-26T21:21:32.7524241Z ---- [ui] ui/issues/issue-20831-debruijn.rs stdout ----
2019-11-26T21:21:32.7524415Z 
2019-11-26T21:21:32.7525000Z error: /checkout/src/test/ui/issues/issue-20831-debruijn.rs:28: expected error not found: mismatched types
2019-11-26T21:21:32.7525152Z 
2019-11-26T21:21:32.7525564Z error: /checkout/src/test/ui/issues/issue-20831-debruijn.rs:28: expected error not found: mismatched types
2019-11-26T21:21:32.7525870Z error: 0 unexpected errors found, 2 expected errors not found
2019-11-26T21:21:32.7526019Z status: exit code: 1
2019-11-26T21:21:32.7526019Z status: exit code: 1
2019-11-26T21:21:32.7526886Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-20831-debruijn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20831-debruijn" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20831-debruijn/auxiliary" "-A" "unused"
2019-11-26T21:21:32.7527126Z not found errors (from test file): [
2019-11-26T21:21:32.7527407Z         line_num: 28,
2019-11-26T21:21:32.7527538Z         kind: Some(
2019-11-26T21:21:32.7527664Z             Error,
2019-11-26T21:21:32.7527804Z         ),
---
2019-11-26T21:21:32.7530542Z ---- [ui] ui/suggestions/missing-assoc-type-bound-restriction.rs stdout ----
2019-11-26T21:21:32.7530729Z diff of stderr:
2019-11-26T21:21:32.7530841Z 
2019-11-26T21:21:32.7530966Z 32    |
2019-11-26T21:21:32.7531598Z 33    = note: required because of the requirements on the impl of `Child<A>` for `ChildWrapper<<T as Parent>::Assoc>`
2019-11-26T21:21:32.7531788Z 34 
2019-11-26T21:21:32.7532217Z - error[E0277]: the trait bound `<T as Parent>::Assoc: Child<A>` is not satisfied
2019-11-26T21:21:32.7532998Z -    |
2019-11-26T21:21:32.7532998Z -    |
2019-11-26T21:21:32.7533356Z - LL | trait Parent {
2019-11-26T21:21:32.7533758Z -    | ------------ required by `Parent`
2019-11-26T21:21:32.7534102Z - ...
2019-11-26T21:21:32.7534512Z - LL | impl<A, T: Parent<Ty = A>> Parent for ParentWrapper<T> {
2019-11-26T21:21:32.7535199Z -    |                                                       - help: consider further restricting the associated type: `where <T as Parent>::Assoc: Child<A>`
2019-11-26T21:21:32.7535539Z - ...
2019-11-26T21:21:32.7535932Z - LL |     type Assoc = ChildWrapper<T::Assoc>;
2019-11-26T21:21:32.7536360Z -    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Child<A>` is not implemented for `<T as Parent>::Assoc`
2019-11-26T21:21:32.7537066Z - error: aborting due to 3 previous errors
2019-11-26T21:21:32.7537229Z + error: aborting due to 2 previous errors
2019-11-26T21:21:32.7537396Z 48 
2019-11-26T21:21:32.7537755Z 49 For more information about this error, try `rustc --explain E0277`.
2019-11-26T21:21:32.7537755Z 49 For more information about this error, try `rustc --explain E0277`.
2019-11-26T21:21:32.7537918Z 50 
2019-11-26T21:21:32.7538030Z 
2019-11-26T21:21:32.7538157Z 
2019-11-26T21:21:32.7538287Z The actual stderr differed from the expected stderr.
2019-11-26T21:21:32.7538756Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/missing-assoc-type-bound-restriction/missing-assoc-type-bound-restriction.stderr
2019-11-26T21:21:32.7540061Z To update references, rerun the tests and pass the `--bless` flag
2019-11-26T21:21:32.7540351Z To only update this specific test, also pass `--test-args suggestions/missing-assoc-type-bound-restriction.rs`
2019-11-26T21:21:32.7540450Z error: 1 errors occurred comparing output.
2019-11-26T21:21:32.7540493Z status: exit code: 1
2019-11-26T21:21:32.7540493Z status: exit code: 1
2019-11-26T21:21:32.7541693Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/missing-assoc-type-bound-restriction.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/missing-assoc-type-bound-restriction" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/missing-assoc-type-bound-restriction/auxiliary" "-A" "unused"
2019-11-26T21:21:32.7542079Z ------------------------------------------
2019-11-26T21:21:32.7542114Z 
2019-11-26T21:21:32.7542337Z ------------------------------------------
2019-11-26T21:21:32.7542383Z stderr:
2019-11-26T21:21:32.7542383Z stderr:
2019-11-26T21:21:32.7542616Z ------------------------------------------
2019-11-26T21:21:32.7542671Z error[E0277]: the trait bound `<T as Parent>::Assoc: Child<A>` is not satisfied
2019-11-26T21:21:32.7543016Z    |
2019-11-26T21:21:32.7543057Z LL |   trait Parent {
2019-11-26T21:21:32.7543278Z    |   ------------ required by `Parent`
2019-11-26T21:21:32.7543340Z ...
2019-11-26T21:21:32.7543340Z ...
2019-11-26T21:21:32.7543388Z LL |   impl<A, T: Parent<Ty = A>> Parent for ParentWrapper<T> {
2019-11-26T21:21:32.7544002Z    |   ^                                                     - help: consider further restricting the associated type: `where <T as Parent>::Assoc: Child<A>`
2019-11-26T21:21:32.7544090Z    |  _|
2019-11-26T21:21:32.7544133Z    | |
2019-11-26T21:21:32.7544183Z LL | |     //~^ ERROR the trait bound `<T as Parent>::Assoc: Child<A>` is not satisfied
2019-11-26T21:21:32.7544248Z LL | |     type Ty = A;
2019-11-26T21:21:32.7544294Z LL | |     type Assoc = ChildWrapper<T::Assoc>;
2019-11-26T21:21:32.7544346Z LL | |     //~^ ERROR the trait bound `<T as Parent>::Assoc: Child<A>` is not satisfied
2019-11-26T21:21:32.7544420Z LL | |     //~| ERROR the trait bound `<T as Parent>::Assoc: Child<A>` is not satisfied
2019-11-26T21:21:32.7544466Z LL | | }
2019-11-26T21:21:32.7544514Z    | |_^ the trait `Child<A>` is not implemented for `<T as Parent>::Assoc`
2019-11-26T21:21:32.7544565Z 
2019-11-26T21:21:32.7544613Z error[E0277]: the trait bound `<T as Parent>::Assoc: Child<A>` is not satisfied
2019-11-26T21:21:32.7545175Z    |
2019-11-26T21:21:32.7545175Z    |
2019-11-26T21:21:32.7545234Z LL |     type Assoc: Child<Self::Ty>;
2019-11-26T21:21:32.7545449Z    |          ----- associated type defined here
2019-11-26T21:21:32.7545493Z ...
2019-11-26T21:21:32.7545552Z LL | impl<A, T: Parent<Ty = A>> Parent for ParentWrapper<T> {
2019-11-26T21:21:32.7547094Z    | ------------------------------------------------------- help: consider further restricting the associated type: `where <T as Parent>::Assoc: Child<A>`
2019-11-26T21:21:32.7547211Z    | in this `impl` item
2019-11-26T21:21:32.7547248Z ...
2019-11-26T21:21:32.7547248Z ...
2019-11-26T21:21:32.7547288Z LL |     type Assoc = ChildWrapper<T::Assoc>;
2019-11-26T21:21:32.7547354Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Child<A>` is not implemented for `<T as Parent>::Assoc`
2019-11-26T21:21:32.7547398Z    |
2019-11-26T21:21:32.7547456Z    = note: required because of the requirements on the impl of `Child<A>` for `ChildWrapper<<T as Parent>::Assoc>`
2019-11-26T21:21:32.7547553Z error: aborting due to 2 previous errors
2019-11-26T21:21:32.7547580Z 
2019-11-26T21:21:32.7547862Z For more information about this error, try `rustc --explain E0277`.
2019-11-26T21:21:32.7547894Z 
---
2019-11-26T21:21:32.7549212Z 
2019-11-26T21:21:32.7553499Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-26T21:21:32.7565246Z 
2019-11-26T21:21:32.7565328Z 
2019-11-26T21:21:32.7567150Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-26T21:21:32.7567454Z 
2019-11-26T21:21:32.7567481Z 
2019-11-26T21:21:32.7573211Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-26T21:21:32.7573284Z Build completed unsuccessfully in 1:04:02
2019-11-26T21:21:32.7573284Z Build completed unsuccessfully in 1:04:02
2019-11-26T21:21:32.7628279Z == clock drift check ==
2019-11-26T21:21:32.7645086Z   local time: Tue Nov 26 21:21:32 UTC 2019
2019-11-26T21:21:32.9176473Z   network time: Tue, 26 Nov 2019 21:21:32 GMT
2019-11-26T21:21:32.9179247Z == end clock drift check ==
2019-11-26T21:21:33.7326173Z 
2019-11-26T21:21:33.7417445Z ##[error]Bash exited with code '1'.
2019-11-26T21:21:33.7484403Z ##[section]Starting: Checkout
2019-11-26T21:21:33.7486268Z ==============================================================================
2019-11-26T21:21:33.7486335Z Task         : Get sources
2019-11-26T21:21:33.7486377Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
