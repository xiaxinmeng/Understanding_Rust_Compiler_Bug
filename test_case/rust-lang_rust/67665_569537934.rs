plain
2019-12-29T19:03:09.7597450Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-29T19:03:09.7829710Z ##[command]git config gc.auto 0
2019-12-29T19:03:09.7923056Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-29T19:03:09.7985419Z ##[command]git config --get-all http.proxy
2019-12-29T19:03:09.8136637Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67665/merge:refs/remotes/pull/67665/merge
---
2019-12-29T20:01:42.4844849Z .................................................................................................... 1600/9463
2019-12-29T20:01:47.3426854Z .................................................................................................... 1700/9463
2019-12-29T20:01:56.5284734Z ..................................................................................................i. 1800/9463
2019-12-29T20:02:04.5151412Z .................................................................................................... 1900/9463
2019-12-29T20:02:11.5669035Z ....................................................................................iiiii........... 2000/9463
2019-12-29T20:02:32.4325169Z .................................................................................................... 2200/9463
2019-12-29T20:02:34.8195867Z .................................................................................................... 2300/9463
2019-12-29T20:02:37.2873814Z .................................................................................................... 2400/9463
2019-12-29T20:02:43.3946487Z .................................................................................................... 2500/9463
---
2019-12-29T20:05:40.9694181Z ...............i...............i.................................................................... 4900/9463
2019-12-29T20:05:50.7560204Z .................................................................................................... 5000/9463
2019-12-29T20:05:56.2098125Z ............................................................i....................................... 5100/9463
2019-12-29T20:06:04.2392500Z .................................................................................................... 5200/9463
2019-12-29T20:06:11.6029450Z ...........................ii.ii...........i........................................................ 5300/9463
2019-12-29T20:06:20.6924624Z .................................................................................................... 5500/9463
2019-12-29T20:06:31.6599451Z .................................................................................................... 5600/9463
2019-12-29T20:06:37.7502386Z .........i.......................................................................................... 5700/9463
2019-12-29T20:06:43.8105551Z .................................................................................................... 5800/9463
2019-12-29T20:06:43.8105551Z .................................................................................................... 5800/9463
2019-12-29T20:06:54.4175256Z .................................................................................................ii. 5900/9463
2019-12-29T20:07:06.2057619Z ..i..ii...........i................................................................................. 6000/9463
2019-12-29T20:07:23.9827418Z .................................................................................................... 6200/9463
2019-12-29T20:07:31.3240794Z .................................................................................................... 6300/9463
2019-12-29T20:07:31.3240794Z .................................................................................................... 6300/9463
2019-12-29T20:07:46.0364106Z ........................i..ii....................................................................... 6400/9463
2019-12-29T20:08:05.7994716Z .................................................................................................... 6600/9463
2019-12-29T20:08:07.9424852Z .i.................................................................................................. 6700/9463
2019-12-29T20:08:10.2240327Z .................................................................................................... 6800/9463
2019-12-29T20:08:12.8166651Z .i.................................................................................................. 6900/9463
---
2019-12-29T20:09:48.5514476Z .................................................................................................... 7500/9463
2019-12-29T20:09:53.4569024Z .................................................................................................... 7600/9463
2019-12-29T20:09:58.9337895Z .................................................................................................... 7700/9463
2019-12-29T20:10:08.7901350Z .................................................................................................... 7800/9463
2019-12-29T20:10:16.2126912Z ................................iiii................................................................ 7900/9463
2019-12-29T20:10:30.7064101Z .................................................................................................... 8100/9463
2019-12-29T20:10:39.4317928Z .................................................................................................... 8200/9463
2019-12-29T20:10:53.4322317Z .................................................................................................... 8300/9463
2019-12-29T20:11:00.8767318Z .................................................................................................... 8400/9463
---
2019-12-29T20:12:53.3659009Z 14 help: consider further restricting this bound with `+ std::marker::Sized`
2019-12-29T20:12:53.3659238Z -   --> $DIR/trait-suggest-where-clause.rs:6:26
2019-12-29T20:12:53.3659455Z +   --> $DIR/trait-suggest-where-clause.rs:9:26
2019-12-29T20:12:53.3659495Z 16    |
2019-12-29T20:12:53.3659534Z 17 LL | fn check<T: Iterator, U: ?Sized>() {
2019-12-29T20:12:53.3659618Z 
2019-12-29T20:12:53.3659618Z 
2019-12-29T20:12:53.3660057Z 31    = help: within `Misc<U>`, the trait `std::marker::Sized` is not implemented for `U`
2019-12-29T20:12:53.3660497Z 33 help: consider further restricting this bound with `+ std::marker::Sized`
2019-12-29T20:12:53.3660703Z -   --> $DIR/trait-suggest-where-clause.rs:6:26
2019-12-29T20:12:53.3661110Z +   --> $DIR/trait-suggest-where-clause.rs:9:26
2019-12-29T20:12:53.3661154Z 35    |
2019-12-29T20:12:53.3661154Z 35    |
2019-12-29T20:12:53.3661193Z 36 LL | fn check<T: Iterator, U: ?Sized>() {
2019-12-29T20:12:53.3661400Z 
2019-12-29T20:12:53.3661424Z 
2019-12-29T20:12:53.3661464Z The actual stderr differed from the expected stderr.
2019-12-29T20:12:53.3661995Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-suggest-where-clause/trait-suggest-where-clause.stderr
2019-12-29T20:12:53.3661995Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-suggest-where-clause/trait-suggest-where-clause.stderr
2019-12-29T20:12:53.3662262Z To update references, rerun the tests and pass the `--bless` flag
2019-12-29T20:12:53.3662673Z To only update this specific test, also pass `--test-args traits/trait-suggest-where-clause.rs`
2019-12-29T20:12:53.3662763Z error: 1 errors occurred comparing output.
2019-12-29T20:12:53.3662803Z status: exit code: 1
2019-12-29T20:12:53.3662803Z status: exit code: 1
2019-12-29T20:12:53.3663715Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-suggest-where-clause.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-suggest-where-clause" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-suggest-where-clause/auxiliary" "-A" "unused"
2019-12-29T20:12:53.3664018Z ------------------------------------------
2019-12-29T20:12:53.3664072Z 
2019-12-29T20:12:53.3664262Z ------------------------------------------
2019-12-29T20:12:53.3664301Z stderr:
---
2019-12-29T20:12:53.3665083Z    |                    ^ doesn't have a size known at compile-time
2019-12-29T20:12:53.3665141Z    | 
2019-12-29T20:12:53.3665181Z   ::: /checkout/src/libcore/mem/mod.rs:275:22
2019-12-29T20:12:53.3665217Z    |
2019-12-29T20:12:53.3665401Z LL | pub const fn size_of<T>() -> usize {
2019-12-29T20:12:53.3665684Z    |
2019-12-29T20:12:53.3665728Z    = help: the trait `std::marker::Sized` is not implemented for `U`
2019-12-29T20:12:53.3666023Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-12-29T20:12:53.3666077Z help: consider further restricting this bound with `+ std::marker::Sized`
2019-12-29T20:12:53.3666077Z help: consider further restricting this bound with `+ std::marker::Sized`
2019-12-29T20:12:53.3666465Z   --> /checkout/src/test/ui/traits/trait-suggest-where-clause.rs:9:26
2019-12-29T20:12:53.3666527Z    |
2019-12-29T20:12:53.3666564Z LL | fn check<T: Iterator, U: ?Sized>() {
2019-12-29T20:12:53.3666655Z 
2019-12-29T20:12:53.3666698Z error[E0277]: the size for values of type `U` cannot be known at compilation time
2019-12-29T20:12:53.3666925Z   --> /checkout/src/test/ui/traits/trait-suggest-where-clause.rs:14:5
2019-12-29T20:12:53.3667070Z    |
2019-12-29T20:12:53.3667070Z    |
2019-12-29T20:12:53.3667116Z LL |     mem::size_of::<Misc<U>>();
2019-12-29T20:12:53.3667415Z    | 
2019-12-29T20:12:53.3667474Z   ::: /checkout/src/libcore/mem/mod.rs:275:22
2019-12-29T20:12:53.3667511Z    |
2019-12-29T20:12:53.3667511Z    |
2019-12-29T20:12:53.3667703Z LL | pub const fn size_of<T>() -> usize {
2019-12-29T20:12:53.3667986Z    |
2019-12-29T20:12:53.3667986Z    |
2019-12-29T20:12:53.3668029Z    = help: within `Misc<U>`, the trait `std::marker::Sized` is not implemented for `U`
2019-12-29T20:12:53.3668496Z help: consider further restricting this bound with `+ std::marker::Sized`
2019-12-29T20:12:53.3668718Z   --> /checkout/src/test/ui/traits/trait-suggest-where-clause.rs:9:26
2019-12-29T20:12:53.3668787Z    |
2019-12-29T20:12:53.3668787Z    |
2019-12-29T20:12:53.3668825Z LL | fn check<T: Iterator, U: ?Sized>() {
2019-12-29T20:12:53.3668927Z    = note: required because it appears within the type `Misc<U>`
2019-12-29T20:12:53.3668955Z 
2019-12-29T20:12:53.3668955Z 
2019-12-29T20:12:53.3668997Z error[E0277]: the trait bound `u64: std::convert::From<T>` is not satisfied
2019-12-29T20:12:53.3669282Z    |
2019-12-29T20:12:53.3669282Z    |
2019-12-29T20:12:53.3669319Z LL |     <u64 as From<T>>::from;
2019-12-29T20:12:53.3669366Z    |     ^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<T>` is not implemented for `u64`
2019-12-29T20:12:53.3669472Z    = note: required by `std::convert::From::from`
2019-12-29T20:12:53.3669500Z 
2019-12-29T20:12:53.3669500Z 
2019-12-29T20:12:53.3669545Z error[E0277]: the trait bound `u64: std::convert::From<<T as std::iter::Iterator>::Item>` is not satisfied
2019-12-29T20:12:53.3669840Z    |
2019-12-29T20:12:53.3669840Z    |
2019-12-29T20:12:53.3669879Z LL |     <u64 as From<<T as Iterator>::Item>>::from;
2019-12-29T20:12:53.3669951Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<<T as std::iter::Iterator>::Item>` is not implemented for `u64`
2019-12-29T20:12:53.3670033Z    = note: required by `std::convert::From::from`
2019-12-29T20:12:53.3670076Z 
2019-12-29T20:12:53.3670076Z 
2019-12-29T20:12:53.3670121Z error[E0277]: the trait bound `Misc<_>: std::convert::From<T>` is not satisfied
2019-12-29T20:12:53.3670416Z    |
2019-12-29T20:12:53.3670416Z    |
2019-12-29T20:12:53.3670453Z LL |     <Misc<_> as From<T>>::from;
2019-12-29T20:12:53.3670500Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<T>` is not implemented for `Misc<_>`
2019-12-29T20:12:53.3670607Z    = note: required by `std::convert::From::from`
2019-12-29T20:12:53.3670633Z 
2019-12-29T20:12:53.3670675Z error[E0277]: the size for values of type `[T]` cannot be known at compilation time
2019-12-29T20:12:53.3670921Z   --> /checkout/src/test/ui/traits/trait-suggest-where-clause.rs:32:20
2019-12-29T20:12:53.3670921Z   --> /checkout/src/test/ui/traits/trait-suggest-where-clause.rs:32:20
2019-12-29T20:12:53.3670963Z    |
2019-12-29T20:12:53.3670999Z LL |     mem::size_of::<[T]>();
2019-12-29T20:12:53.3671275Z    | 
2019-12-29T20:12:53.3671314Z   ::: /checkout/src/libcore/mem/mod.rs:275:22
2019-12-29T20:12:53.3671350Z    |
2019-12-29T20:12:53.3671350Z    |
2019-12-29T20:12:53.3671556Z LL | pub const fn size_of<T>() -> usize {
2019-12-29T20:12:53.3671827Z    |
2019-12-29T20:12:53.3671886Z    = help: the trait `std::marker::Sized` is not implemented for `[T]`
2019-12-29T20:12:53.3672247Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-12-29T20:12:53.3672290Z 
2019-12-29T20:12:53.3672290Z 
2019-12-29T20:12:53.3672352Z error[E0277]: the size for values of type `[&U]` cannot be known at compilation time
2019-12-29T20:12:53.3672647Z    |
2019-12-29T20:12:53.3672647Z    |
2019-12-29T20:12:53.3672685Z LL |     mem::size_of::<[&U]>();
2019-12-29T20:12:53.3672963Z    | 
2019-12-29T20:12:53.3673002Z   ::: /checkout/src/libcore/mem/mod.rs:275:22
2019-12-29T20:12:53.3673058Z    |
2019-12-29T20:12:53.3673058Z    |
2019-12-29T20:12:53.3673358Z LL | pub const fn size_of<T>() -> usize {
2019-12-29T20:12:53.3673639Z    |
2019-12-29T20:12:53.3673680Z    = help: the trait `std::marker::Sized` is not implemented for `[&U]`
2019-12-29T20:12:53.3674200Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-12-29T20:12:53.3674255Z 
---
2019-12-29T20:12:53.3694277Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2019-12-29T20:12:53.3694391Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-29T20:12:53.3712662Z 
2019-12-29T20:12:53.3712729Z 
2019-12-29T20:12:53.3715746Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-29T20:12:53.3716053Z 
2019-12-29T20:12:53.3716084Z 
2019-12-29T20:12:53.3722957Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-29T20:12:53.3723255Z Build completed unsuccessfully in 1:03:02
2019-12-29T20:12:53.3723255Z Build completed unsuccessfully in 1:03:02
2019-12-29T20:12:53.3775076Z == clock drift check ==
2019-12-29T20:12:53.3792466Z   local time: Sun Dec 29 20:12:53 UTC 2019
2019-12-29T20:12:53.6841297Z   network time: Sun, 29 Dec 2019 20:12:53 GMT
2019-12-29T20:12:53.6851280Z == end clock drift check ==
2019-12-29T20:12:54.8955446Z 
2019-12-29T20:12:54.9076787Z ##[error]Bash exited with code '1'.
2019-12-29T20:12:54.9118537Z ##[section]Starting: Checkout
2019-12-29T20:12:54.9120162Z ==============================================================================
2019-12-29T20:12:54.9120230Z Task         : Get sources
2019-12-29T20:12:54.9120273Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
