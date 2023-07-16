plain
2019-12-30T18:06:10.3713495Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-30T18:06:10.3940466Z ##[command]git config gc.auto 0
2019-12-30T18:06:10.4004563Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-30T18:06:10.4063107Z ##[command]git config --get-all http.proxy
2019-12-30T18:06:10.4187037Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67665/merge:refs/remotes/pull/67665/merge
---
2019-12-30T19:01:51.2758733Z .................................................................................................... 1500/9465
2019-12-30T19:01:56.8470831Z .................................................................................................... 1600/9465
2019-12-30T19:02:01.5114910Z .................................................................................................... 1700/9465
2019-12-30T19:02:10.5186871Z .................................................................................................... 1800/9465
2019-12-30T19:02:18.2304001Z .i.................................................................................................. 1900/9465
2019-12-30T19:02:24.5221284Z .......................................................................................iiiii........ 2000/9465
2019-12-30T19:02:44.7679367Z .................................................................................................... 2200/9465
2019-12-30T19:02:46.8882285Z .................................................................................................... 2300/9465
2019-12-30T19:02:49.2728747Z .................................................................................................... 2400/9465
2019-12-30T19:02:55.0031592Z .................................................................................................... 2500/9465
---
2019-12-30T19:05:42.2072244Z ..................i...............i................................................................. 4900/9465
2019-12-30T19:05:51.3466795Z .................................................................................................... 5000/9465
2019-12-30T19:05:56.4360891Z ...............................................................i.................................... 5100/9465
2019-12-30T19:06:04.1515929Z .................................................................................................... 5200/9465
2019-12-30T19:06:11.0730256Z ..............................ii.ii...........i..................................................... 5300/9465
2019-12-30T19:06:19.8790521Z .................................................................................................... 5500/9465
2019-12-30T19:06:29.3015989Z .................................................................................................... 5600/9465
2019-12-30T19:06:35.9693184Z .............i...................................................................................... 5700/9465
2019-12-30T19:06:41.5785591Z .................................................................................................... 5800/9465
2019-12-30T19:06:41.5785591Z .................................................................................................... 5800/9465
2019-12-30T19:06:51.6447990Z .................................................................................................... 5900/9465
2019-12-30T19:07:02.7144615Z .ii...i..ii...........i............................................................................. 6000/9465
2019-12-30T19:07:19.1600623Z .................................................................................................... 6200/9465
2019-12-30T19:07:26.3281164Z .................................................................................................... 6300/9465
2019-12-30T19:07:26.3281164Z .................................................................................................... 6300/9465
2019-12-30T19:07:44.4906692Z ............................i..ii................................................................... 6400/9465
2019-12-30T19:08:03.5962641Z .................................................................................................... 6600/9465
2019-12-30T19:08:05.7654115Z ...i................................................................................................ 6700/9465
2019-12-30T19:08:08.0094047Z .................................................................................................... 6800/9465
2019-12-30T19:08:10.4364691Z ...i................................................................................................ 6900/9465
---
2019-12-30T19:09:43.7406473Z .................................................................................................... 7500/9465
2019-12-30T19:09:48.4927041Z .................................................................................................... 7600/9465
2019-12-30T19:09:53.8154390Z .................................................................................................... 7700/9465
2019-12-30T19:10:03.2935181Z .................................................................................................... 7800/9465
2019-12-30T19:10:10.4069749Z ..................................iiii.............................................................. 7900/9465
2019-12-30T19:10:24.5521203Z .................................................................................................... 8100/9465
2019-12-30T19:10:32.8365039Z .................................................................................................... 8200/9465
2019-12-30T19:10:46.7424839Z .................................................................................................... 8300/9465
2019-12-30T19:10:54.1577953Z .................................................................................................... 8400/9465
---
2019-12-30T19:12:43.4647723Z 14 help: consider further restricting this bound with `+ std::marker::Sized`
2019-12-30T19:12:43.4647915Z -   --> $DIR/trait-suggest-where-clause.rs:6:26
2019-12-30T19:12:43.4648114Z +   --> $DIR/trait-suggest-where-clause.rs:9:26
2019-12-30T19:12:43.4648166Z 16    |
2019-12-30T19:12:43.4648210Z 17 LL | fn check<T: Iterator, U: ?Sized>() {
2019-12-30T19:12:43.4648289Z 
2019-12-30T19:12:43.4648289Z 
2019-12-30T19:12:43.4648329Z 31    = help: within `Misc<U>`, the trait `std::marker::Sized` is not implemented for `U`
2019-12-30T19:12:43.4648667Z 33 help: consider further restricting this bound with `+ std::marker::Sized`
2019-12-30T19:12:43.4648853Z -   --> $DIR/trait-suggest-where-clause.rs:6:26
2019-12-30T19:12:43.4649031Z +   --> $DIR/trait-suggest-where-clause.rs:9:26
2019-12-30T19:12:43.4649086Z 35    |
2019-12-30T19:12:43.4649086Z 35    |
2019-12-30T19:12:43.4649121Z 36 LL | fn check<T: Iterator, U: ?Sized>() {
2019-12-30T19:12:43.4649199Z 
2019-12-30T19:12:43.4649220Z 
2019-12-30T19:12:43.4649435Z The actual stderr differed from the expected stderr.
2019-12-30T19:12:43.4649820Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-suggest-where-clause/trait-suggest-where-clause.stderr
2019-12-30T19:12:43.4649820Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-suggest-where-clause/trait-suggest-where-clause.stderr
2019-12-30T19:12:43.4650048Z To update references, rerun the tests and pass the `--bless` flag
2019-12-30T19:12:43.4650267Z To only update this specific test, also pass `--test-args traits/trait-suggest-where-clause.rs`
2019-12-30T19:12:43.4650348Z error: 1 errors occurred comparing output.
2019-12-30T19:12:43.4650384Z status: exit code: 1
2019-12-30T19:12:43.4650384Z status: exit code: 1
2019-12-30T19:12:43.4651098Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-suggest-where-clause.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-suggest-where-clause" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-suggest-where-clause/auxiliary" "-A" "unused"
2019-12-30T19:12:43.4651391Z ------------------------------------------
2019-12-30T19:12:43.4651434Z 
2019-12-30T19:12:43.4651609Z ------------------------------------------
2019-12-30T19:12:43.4651645Z stderr:
---
2019-12-30T19:12:43.4652372Z    |                    ^ doesn't have a size known at compile-time
2019-12-30T19:12:43.4652410Z    | 
2019-12-30T19:12:43.4652472Z   ::: /checkout/src/libcore/mem/mod.rs:275:22
2019-12-30T19:12:43.4652513Z    |
2019-12-30T19:12:43.4652689Z LL | pub const fn size_of<T>() -> usize {
2019-12-30T19:12:43.4652943Z    |
2019-12-30T19:12:43.4652981Z    = help: the trait `std::marker::Sized` is not implemented for `U`
2019-12-30T19:12:43.4653259Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-12-30T19:12:43.4653311Z help: consider further restricting this bound with `+ std::marker::Sized`
2019-12-30T19:12:43.4653311Z help: consider further restricting this bound with `+ std::marker::Sized`
2019-12-30T19:12:43.4653514Z   --> /checkout/src/test/ui/traits/trait-suggest-where-clause.rs:9:26
2019-12-30T19:12:43.4653569Z    |
2019-12-30T19:12:43.4653603Z LL | fn check<T: Iterator, U: ?Sized>() {
2019-12-30T19:12:43.4653663Z 
2019-12-30T19:12:43.4653726Z error[E0277]: the size for values of type `U` cannot be known at compilation time
2019-12-30T19:12:43.4653936Z   --> /checkout/src/test/ui/traits/trait-suggest-where-clause.rs:14:5
2019-12-30T19:12:43.4653973Z    |
2019-12-30T19:12:43.4653973Z    |
2019-12-30T19:12:43.4654026Z LL |     mem::size_of::<Misc<U>>();
2019-12-30T19:12:43.4654264Z    | 
2019-12-30T19:12:43.4654319Z   ::: /checkout/src/libcore/mem/mod.rs:275:22
2019-12-30T19:12:43.4654353Z    |
2019-12-30T19:12:43.4654353Z    |
2019-12-30T19:12:43.4654527Z LL | pub const fn size_of<T>() -> usize {
2019-12-30T19:12:43.4654778Z    |
2019-12-30T19:12:43.4654778Z    |
2019-12-30T19:12:43.4654818Z    = help: within `Misc<U>`, the trait `std::marker::Sized` is not implemented for `U`
2019-12-30T19:12:43.4655215Z help: consider further restricting this bound with `+ std::marker::Sized`
2019-12-30T19:12:43.4655494Z   --> /checkout/src/test/ui/traits/trait-suggest-where-clause.rs:9:26
2019-12-30T19:12:43.4655550Z    |
2019-12-30T19:12:43.4655550Z    |
2019-12-30T19:12:43.4655585Z LL | fn check<T: Iterator, U: ?Sized>() {
2019-12-30T19:12:43.4655660Z    = note: required because it appears within the type `Misc<U>`
2019-12-30T19:12:43.4655706Z 
2019-12-30T19:12:43.4655706Z 
2019-12-30T19:12:43.4655746Z error[E0277]: the trait bound `u64: std::convert::From<T>` is not satisfied
2019-12-30T19:12:43.4656007Z    |
2019-12-30T19:12:43.4656007Z    |
2019-12-30T19:12:43.4656041Z LL |     <u64 as From<T>>::from;
2019-12-30T19:12:43.4656084Z    |     ^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<T>` is not implemented for `u64`
2019-12-30T19:12:43.4656174Z    = note: required by `std::convert::From::from`
2019-12-30T19:12:43.4656206Z 
2019-12-30T19:12:43.4656206Z 
2019-12-30T19:12:43.4656252Z error[E0277]: the trait bound `u64: std::convert::From<<T as std::iter::Iterator>::Item>` is not satisfied
2019-12-30T19:12:43.4656514Z    |
2019-12-30T19:12:43.4656514Z    |
2019-12-30T19:12:43.4656550Z LL |     <u64 as From<<T as Iterator>::Item>>::from;
2019-12-30T19:12:43.4656615Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<<T as std::iter::Iterator>::Item>` is not implemented for `u64`
2019-12-30T19:12:43.4656690Z    = note: required by `std::convert::From::from`
2019-12-30T19:12:43.4656714Z 
2019-12-30T19:12:43.4656714Z 
2019-12-30T19:12:43.4656770Z error[E0277]: the trait bound `Misc<_>: std::convert::From<T>` is not satisfied
2019-12-30T19:12:43.4657010Z    |
2019-12-30T19:12:43.4657010Z    |
2019-12-30T19:12:43.4657062Z LL |     <Misc<_> as From<T>>::from;
2019-12-30T19:12:43.4657111Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<T>` is not implemented for `Misc<_>`
2019-12-30T19:12:43.4657206Z    = note: required by `std::convert::From::from`
2019-12-30T19:12:43.4657230Z 
2019-12-30T19:12:43.4657268Z error[E0277]: the size for values of type `[T]` cannot be known at compilation time
2019-12-30T19:12:43.4657493Z   --> /checkout/src/test/ui/traits/trait-suggest-where-clause.rs:32:20
2019-12-30T19:12:43.4657493Z   --> /checkout/src/test/ui/traits/trait-suggest-where-clause.rs:32:20
2019-12-30T19:12:43.4657531Z    |
2019-12-30T19:12:43.4657565Z LL |     mem::size_of::<[T]>();
2019-12-30T19:12:43.4657816Z    | 
2019-12-30T19:12:43.4657851Z   ::: /checkout/src/libcore/mem/mod.rs:275:22
2019-12-30T19:12:43.4657884Z    |
2019-12-30T19:12:43.4657884Z    |
2019-12-30T19:12:43.4658076Z LL | pub const fn size_of<T>() -> usize {
2019-12-30T19:12:43.4658312Z    |
2019-12-30T19:12:43.4658374Z    = help: the trait `std::marker::Sized` is not implemented for `[T]`
2019-12-30T19:12:43.4658638Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-12-30T19:12:43.4658669Z 
2019-12-30T19:12:43.4658669Z 
2019-12-30T19:12:43.4658708Z error[E0277]: the size for values of type `[&U]` cannot be known at compilation time
2019-12-30T19:12:43.4658963Z    |
2019-12-30T19:12:43.4658963Z    |
2019-12-30T19:12:43.4658996Z LL |     mem::size_of::<[&U]>();
2019-12-30T19:12:43.4659246Z    | 
2019-12-30T19:12:43.4659281Z   ::: /checkout/src/libcore/mem/mod.rs:275:22
2019-12-30T19:12:43.4659332Z    |
2019-12-30T19:12:43.4659332Z    |
2019-12-30T19:12:43.4659507Z LL | pub const fn size_of<T>() -> usize {
2019-12-30T19:12:43.4659823Z    |
2019-12-30T19:12:43.4659906Z    = help: the trait `std::marker::Sized` is not implemented for `[&U]`
2019-12-30T19:12:43.4660191Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-12-30T19:12:43.4660222Z 
---
2019-12-30T19:12:43.4664680Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2019-12-30T19:12:43.4664733Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-30T19:12:43.4676507Z 
2019-12-30T19:12:43.4676599Z 
2019-12-30T19:12:43.4678030Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-30T19:12:43.4678464Z 
2019-12-30T19:12:43.4678488Z 
2019-12-30T19:12:43.4683159Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-30T19:12:43.4683490Z Build completed unsuccessfully in 1:00:03
2019-12-30T19:12:43.4683490Z Build completed unsuccessfully in 1:00:03
2019-12-30T19:12:43.4733404Z == clock drift check ==
2019-12-30T19:12:43.4747891Z   local time: Mon Dec 30 19:12:43 UTC 2019
2019-12-30T19:12:44.0011793Z   network time: Mon, 30 Dec 2019 19:12:43 GMT
2019-12-30T19:12:44.0011861Z == end clock drift check ==
2019-12-30T19:12:45.0857007Z 
2019-12-30T19:12:45.0996710Z ##[error]Bash exited with code '1'.
2019-12-30T19:12:45.1032724Z ##[section]Starting: Checkout
2019-12-30T19:12:45.1034262Z ==============================================================================
2019-12-30T19:12:45.1034331Z Task         : Get sources
2019-12-30T19:12:45.1034371Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
