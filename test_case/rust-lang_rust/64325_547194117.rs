plain
2019-10-28T22:34:49.5117977Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-28T22:34:49.5332064Z ##[command]git config gc.auto 0
2019-10-28T22:34:49.5410849Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-28T22:34:49.5479481Z ##[command]git config --get-all http.proxy
2019-10-28T22:34:49.5612159Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64325/merge:refs/remotes/pull/64325/merge
---
2019-10-28T23:35:06.6742775Z .................................................................................................... 1600/9260
2019-10-28T23:35:13.0200494Z .................................................................................................... 1700/9260
2019-10-28T23:35:26.5125550Z ..........................................................i...............i......................... 1800/9260
2019-10-28T23:35:34.6245340Z .................................................................................................... 1900/9260
2019-10-28T23:35:49.6928230Z ................................................iiiii............................................... 2000/9260
2019-10-28T23:36:00.9911185Z .................................................................................................... 2200/9260
2019-10-28T23:36:03.7243856Z .................................................................................................... 2300/9260
2019-10-28T23:36:07.6747120Z .................................................................................................... 2400/9260
2019-10-28T23:36:31.7671129Z .................................................................................................... 2500/9260
---
2019-10-28T23:39:30.6466915Z .................................................i...............i.................................. 4800/9260
2019-10-28T23:39:39.9972382Z .................................................................................................... 4900/9260
2019-10-28T23:39:48.8593923Z .................................................................................................... 5000/9260
2019-10-28T23:39:55.3695070Z .................................................................................................... 5100/9260
2019-10-28T23:40:06.0314040Z ..................................................ii.ii...........i................................. 5200/9260
2019-10-28T23:40:16.1614505Z .................................................................................................... 5400/9260
2019-10-28T23:40:25.6403084Z .................................................................................................... 5500/9260
2019-10-28T23:40:33.7675321Z ....................i............................................................................... 5600/9260
2019-10-28T23:40:39.9409435Z .................................................................................................... 5700/9260
2019-10-28T23:40:39.9409435Z .................................................................................................... 5700/9260
2019-10-28T23:40:52.6584725Z .................................................................................................... 5800/9260
2019-10-28T23:41:04.8573643Z .....ii...i..ii...........i......................................................................... 5900/9260
2019-10-28T23:41:27.6419610Z .................................................................................................... 6100/9260
2019-10-28T23:41:34.2249917Z .................................................................................................... 6200/9260
2019-10-28T23:41:34.2249917Z .................................................................................................... 6200/9260
2019-10-28T23:41:48.5455682Z ........................i..ii....................................................................... 6300/9260
2019-10-28T23:42:09.7355672Z ..........................................................................................i......... 6500/9260
2019-10-28T23:42:12.1963007Z .................................................................................................... 6600/9260
2019-10-28T23:42:14.6759727Z .................................................................i.................................. 6700/9260
2019-10-28T23:42:17.8482074Z .................................................................................................... 6800/9260
---
2019-10-28T23:46:29.7194085Z 1 error[E0038]: the trait `Foo` cannot be made into an object
2019-10-28T23:46:29.7194705Z -   --> $DIR/arbitrary-self-types-not-object-safe.rs:34:13
2019-10-28T23:46:29.7195231Z +   --> $DIR/arbitrary-self-types-not-object-safe.rs:33:13
2019-10-28T23:46:29.7195667Z 3    |
2019-10-28T23:46:29.7196121Z 4 LL |     fn foo(self: &Rc<Self>) -> usize;
2019-10-28T23:46:29.7196586Z 5    |        --- method `foo`'s `self` parameter cannot be dispatched on
2019-10-28T23:46:29.7197012Z 
2019-10-28T23:46:29.7197237Z The actual stderr differed from the expected stderr.
2019-10-28T23:46:29.7197237Z The actual stderr differed from the expected stderr.
2019-10-28T23:46:29.7198382Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary-self-types-not-object-safe.object_safe_for_dispatch/arbitrary-self-types-not-object-safe.object_safe_for_dispatch.stderr
2019-10-28T23:46:29.7199987Z To update references, rerun the tests and pass the `--bless` flag
2019-10-28T23:46:29.7200900Z To only update this specific test, also pass `--test-args self/arbitrary-self-types-not-object-safe.rs`
2019-10-28T23:46:29.7201359Z 
2019-10-28T23:46:29.7201904Z error in revision `object_safe_for_dispatch`: 1 errors occurred comparing output.
2019-10-28T23:46:29.7202039Z status: exit code: 1
2019-10-28T23:46:29.7203746Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/arbitrary-self-types-not-object-safe.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "object_safe_for_dispatch" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary-self-types-not-object-safe.object_safe_for_dispatch" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary-self-types-not-object-safe.object_safe_for_dispatch/auxiliary" "-A" "unused"
2019-10-28T23:46:29.7204513Z ------------------------------------------
2019-10-28T23:46:29.7204684Z 
2019-10-28T23:46:29.7205211Z ------------------------------------------
2019-10-28T23:46:29.7205377Z stderr:
2019-10-28T23:46:29.7205377Z stderr:
2019-10-28T23:46:29.7205694Z ------------------------------------------
2019-10-28T23:46:29.7205886Z error[E0038]: the trait `Foo` cannot be made into an object
2019-10-28T23:46:29.7206294Z   --> /checkout/src/test/ui/self/arbitrary-self-types-not-object-safe.rs:33:13
2019-10-28T23:46:29.7206500Z    |
2019-10-28T23:46:29.7206859Z LL |     fn foo(self: &Rc<Self>) -> usize;
2019-10-28T23:46:29.7207391Z    |        --- method `foo`'s `self` parameter cannot be dispatched on
2019-10-28T23:46:29.7207576Z ...
2019-10-28T23:46:29.7207726Z LL |     let x = Rc::new(5usize) as Rc<dyn Foo>;
2019-10-28T23:46:29.7208315Z    |             ^^^^^^^^^^^^^^^ the trait `Foo` cannot be made into an object
2019-10-28T23:46:29.7208508Z    |
2019-10-28T23:46:29.7208671Z    = note: required because of the requirements on the impl of `std::ops::CoerceUnsized<std::rc::Rc<dyn Foo>>` for `std::rc::Rc<usize>`
2019-10-28T23:46:29.7208867Z    = note: required by cast to type `std::rc::Rc<dyn Foo>`
2019-10-28T23:46:29.7209152Z error: aborting due to previous error
2019-10-28T23:46:29.7209281Z 
2019-10-28T23:46:29.7209727Z For more information about this error, try `rustc --explain E0038`.
2019-10-28T23:46:29.7209892Z 
---
2019-10-28T23:46:29.7211739Z 1 error[E0038]: the trait `Foo` cannot be made into an object
2019-10-28T23:46:29.7212141Z -   --> $DIR/arbitrary-self-types-not-object-safe.rs:34:32
2019-10-28T23:46:29.7212504Z +   --> $DIR/arbitrary-self-types-not-object-safe.rs:33:32
2019-10-28T23:46:29.7212675Z 3    |
2019-10-28T23:46:29.7213055Z 4 LL |     fn foo(self: &Rc<Self>) -> usize;
2019-10-28T23:46:29.7213421Z 5    |        --- method `foo`'s `self` parameter cannot be dispatched on
2019-10-28T23:46:29.7213720Z 8    |                                ^^^^^^^^^^^ the trait `Foo` cannot be made into an object
2019-10-28T23:46:29.7213861Z 9 
2019-10-28T23:46:29.7213997Z 10 error[E0038]: the trait `Foo` cannot be made into an object
2019-10-28T23:46:29.7214365Z -   --> $DIR/arbitrary-self-types-not-object-safe.rs:34:13
2019-10-28T23:46:29.7214365Z -   --> $DIR/arbitrary-self-types-not-object-safe.rs:34:13
2019-10-28T23:46:29.7214735Z +   --> $DIR/arbitrary-self-types-not-object-safe.rs:33:13
2019-10-28T23:46:29.7214897Z 12    |
2019-10-28T23:46:29.7215411Z 13 LL |     fn foo(self: &Rc<Self>) -> usize;
2019-10-28T23:46:29.7215957Z 14    |        --- method `foo`'s `self` parameter cannot be dispatched on
2019-10-28T23:46:29.7216211Z 
2019-10-28T23:46:29.7216337Z The actual stderr differed from the expected stderr.
2019-10-28T23:46:29.7216337Z The actual stderr differed from the expected stderr.
2019-10-28T23:46:29.7217342Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary-self-types-not-object-safe.curr/arbitrary-self-types-not-object-safe.curr.stderr
2019-10-28T23:46:29.7217742Z To update references, rerun the tests and pass the `--bless` flag
2019-10-28T23:46:29.7218569Z To only update this specific test, also pass `--test-args self/arbitrary-self-types-not-object-safe.rs`
2019-10-28T23:46:29.7218803Z 
2019-10-28T23:46:29.7218966Z error in revision `curr`: 1 errors occurred comparing output.
2019-10-28T23:46:29.7219120Z status: exit code: 1
2019-10-28T23:46:29.7220214Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/arbitrary-self-types-not-object-safe.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "curr" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary-self-types-not-object-safe.curr" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary-self-types-not-object-safe.curr/auxiliary" "-A" "unused"
2019-10-28T23:46:29.7220847Z ------------------------------------------
2019-10-28T23:46:29.7221008Z 
2019-10-28T23:46:29.7221363Z ------------------------------------------
2019-10-28T23:46:29.7297618Z stderr:
2019-10-28T23:46:29.7297618Z stderr:
2019-10-28T23:46:29.7298802Z ------------------------------------------
2019-10-28T23:46:29.7299108Z error[E0038]: the trait `Foo` cannot be made into an object
2019-10-28T23:46:29.7299641Z   --> /checkout/src/test/ui/self/arbitrary-self-types-not-object-safe.rs:33:32
2019-10-28T23:46:29.7299881Z    |
2019-10-28T23:46:29.7300290Z LL |     fn foo(self: &Rc<Self>) -> usize;
2019-10-28T23:46:29.7300737Z    |        --- method `foo`'s `self` parameter cannot be dispatched on
2019-10-28T23:46:29.7300953Z ...
2019-10-28T23:46:29.7301129Z LL |     let x = Rc::new(5usize) as Rc<dyn Foo>;
2019-10-28T23:46:29.7301313Z    |                                ^^^^^^^^^^^ the trait `Foo` cannot be made into an object
2019-10-28T23:46:29.7301780Z error[E0038]: the trait `Foo` cannot be made into an object
2019-10-28T23:46:29.7302363Z   --> /checkout/src/test/ui/self/arbitrary-self-types-not-object-safe.rs:33:13
2019-10-28T23:46:29.7302536Z    |
2019-10-28T23:46:29.7302864Z LL |     fn foo(self: &Rc<Self>) -> usize;
2019-10-28T23:46:29.7302864Z LL |     fn foo(self: &Rc<Self>) -> usize;
2019-10-28T23:46:29.7303431Z    |        --- method `foo`'s `self` parameter cannot be dispatched on
2019-10-28T23:46:29.7303601Z ...
2019-10-28T23:46:29.7303741Z LL |     let x = Rc::new(5usize) as Rc<dyn Foo>;
2019-10-28T23:46:29.7304088Z    |             ^^^^^^^^^^^^^^^ the trait `Foo` cannot be made into an object
2019-10-28T23:46:29.7304291Z    |
2019-10-28T23:46:29.7304442Z    = note: required because of the requirements on the impl of `std::ops::CoerceUnsized<std::rc::Rc<dyn Foo>>` for `std::rc::Rc<usize>`
2019-10-28T23:46:29.7304858Z    = note: required by cast to type `std::rc::Rc<dyn Foo>`
2019-10-28T23:46:29.7305288Z error: aborting due to 2 previous errors
2019-10-28T23:46:29.7305426Z 
2019-10-28T23:46:29.7306348Z For more information about this error, try `rustc --explain E0038`.
2019-10-28T23:46:29.7306531Z 
---
2019-10-28T23:46:29.7310654Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-28T23:46:29.7310894Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-28T23:46:29.7310928Z 
2019-10-28T23:46:29.7310953Z 
2019-10-28T23:46:29.7312721Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-28T23:46:29.7313096Z 
2019-10-28T23:46:29.7313120Z 
2019-10-28T23:46:29.7313174Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-28T23:46:29.7313225Z Build completed unsuccessfully in 1:04:57
2019-10-28T23:46:29.7313225Z Build completed unsuccessfully in 1:04:57
2019-10-28T23:46:29.7326952Z == clock drift check ==
2019-10-28T23:46:29.7344389Z   local time: Mon Oct 28 23:46:29 UTC 2019
2019-10-28T23:46:29.8894869Z   network time: Mon, 28 Oct 2019 23:46:29 GMT
2019-10-28T23:46:29.8896034Z == end clock drift check ==
2019-10-28T23:46:30.8316283Z 
2019-10-28T23:46:30.8444636Z ##[error]Bash exited with code '1'.
2019-10-28T23:46:30.8486014Z ##[section]Starting: Checkout
2019-10-28T23:46:30.8487502Z ==============================================================================
2019-10-28T23:46:30.8487545Z Task         : Get sources
2019-10-28T23:46:30.8487581Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
