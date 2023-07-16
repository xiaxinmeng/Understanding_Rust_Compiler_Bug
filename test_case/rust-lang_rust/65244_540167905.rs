plain
2019-10-09T18:53:20.4167744Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-09T18:53:20.4254132Z ##[command]git config gc.auto 0
2019-10-09T18:53:20.4335653Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-09T18:53:20.4397463Z ##[command]git config --get-all http.proxy
2019-10-09T18:53:20.4534636Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65244/merge:refs/remotes/pull/65244/merge
---
2019-10-09T19:51:50.2441944Z running 9143 tests
2019-10-09T19:52:07.2574465Z .................................................................................................... 100/9143
2019-10-09T19:52:17.8250756Z .................................................................................................... 200/9143
2019-10-09T19:52:25.7973488Z .................................................................................................... 300/9143
2019-10-09T19:52:32.8228669Z .......................................F...FF.F.FFF.F.FF.....FF.F...F.F...F.F..FFFFFFFFFF..FF..FF..F 400/9143
2019-10-09T19:52:43.7501735Z FF.......FFFFFFF........FF....FF.F.FF..FF.FFF...................................................i... 500/9143
2019-10-09T19:53:01.3626119Z .................................................................................................... 700/9143
2019-10-09T19:53:06.0638551Z .................................................................................................... 800/9143
2019-10-09T19:53:10.5895352Z ...............................................................................i.................... 900/9143
2019-10-09T19:53:20.4205762Z .................................................................................................... 1000/9143
---
2019-10-09T19:54:01.4783642Z .................................................................................................... 1600/9143
2019-10-09T19:54:08.4965136Z .................................................................................................... 1700/9143
2019-10-09T19:54:20.2500559Z ..................i...............i................................................................. 1800/9143
2019-10-09T19:54:27.4508902Z .................................................................................................... 1900/9143
2019-10-09T19:54:39.7410949Z .........iiiii.................................F.................................................... 2000/9143
2019-10-09T19:54:49.6609391Z .................................................................................................... 2200/9143
2019-10-09T19:54:52.1951472Z .................................................................................................... 2300/9143
2019-10-09T19:54:57.7801773Z .................................................................................................... 2400/9143
2019-10-09T19:55:04.0151487Z .................................................................................................... 2500/9143
---
2019-10-09T19:57:57.9152693Z .................................................................................................... 4700/9143
2019-10-09T19:58:04.9819886Z ..i...............i................................................................................. 4800/9143
2019-10-09T19:58:16.3608958Z .................................................................................................... 4900/9143
2019-10-09T19:58:21.9596200Z .................................................................................................... 5000/9143
2019-10-09T19:58:33.5126740Z ................................................................................................ii.i 5100/9143
2019-10-09T19:58:38.7143394Z i................................................................................................... 5200/9143
2019-10-09T19:58:53.5175090Z .................................................................................................... 5400/9143
2019-10-09T19:59:00.7045001Z ..............................................................i..................................... 5500/9143
2019-10-09T19:59:08.0728670Z .................................................................................................... 5600/9143
2019-10-09T19:59:15.4508849Z .................................................................................................... 5700/9143
2019-10-09T19:59:15.4508849Z .................................................................................................... 5700/9143
2019-10-09T19:59:27.1828916Z ...........................................................ii...i..ii...........i................... 5800/9143
2019-10-09T19:59:52.3514923Z .................................................................................................... 6000/9143
2019-10-09T20:00:01.4777872Z .................................................................................................... 6100/9143
2019-10-09T20:00:01.4777872Z .................................................................................................... 6100/9143
2019-10-09T20:00:10.1000488Z .................................................................i..ii.............................. 6200/9143
2019-10-09T20:00:39.3824825Z .................................................................................................... 6400/9143
2019-10-09T20:00:41.4427185Z .........................i.......................................................................... 6500/9143
2019-10-09T20:00:43.6523369Z ..................................................................................................i. 6600/9143
2019-10-09T20:00:46.3555417Z .................................................................................................... 6700/9143
---
2019-10-09T20:04:56.6463760Z ---- [ui] ui/async-await/async-await.rs stdout ----
2019-10-09T20:04:56.6463852Z 
2019-10-09T20:04:56.6464254Z error: test compilation failed although it shouldn't!
2019-10-09T20:04:56.6464303Z status: exit code: 1
2019-10-09T20:04:56.6465850Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-await.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-await/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-await/auxiliary"
2019-10-09T20:04:56.6466194Z ------------------------------------------
2019-10-09T20:04:56.6466230Z 
2019-10-09T20:04:56.6466443Z ------------------------------------------
2019-10-09T20:04:56.6466490Z stderr:
2019-10-09T20:04:56.6466490Z stderr:
2019-10-09T20:04:56.6466725Z ------------------------------------------
2019-10-09T20:04:56.6466779Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6466885Z error: aborting due to previous error
2019-10-09T20:04:56.6466915Z 
2019-10-09T20:04:56.6467156Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6467192Z 
2019-10-09T20:04:56.6467192Z 
2019-10-09T20:04:56.6467414Z ------------------------------------------
2019-10-09T20:04:56.6467446Z 
2019-10-09T20:04:56.6467472Z 
2019-10-09T20:04:56.6467697Z ---- [ui] ui/async-await/async-error-span.rs stdout ----
2019-10-09T20:04:56.6467763Z diff of stderr:
2019-10-09T20:04:56.6467792Z 
2019-10-09T20:04:56.6468035Z - error[E0698]: type inside `async fn` body must be known in this context
2019-10-09T20:04:56.6468594Z -   --> $DIR/async-error-span.rs:12:9
2019-10-09T20:04:56.6468665Z + error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6468710Z + 
2019-10-09T20:04:56.6469038Z + error[E0277]: the trait bound `(): std::future::Future` is not satisfied
2019-10-09T20:04:56.6469322Z +   --> $DIR/async-error-span.rs:7:20
2019-10-09T20:04:56.6469678Z - LL |     let a;
2019-10-09T20:04:56.6469948Z -    |         ^ cannot infer type
2019-10-09T20:04:56.6469948Z -    |         ^ cannot infer type
2019-10-09T20:04:56.6470506Z + LL | fn get_future() -> impl Future<Output = ()> {
2019-10-09T20:04:56.6470635Z 6    |
2019-10-09T20:04:56.6470635Z 6    |
2019-10-09T20:04:56.6470883Z - note: the type is part of the `async fn` body because of this `await`
2019-10-09T20:04:56.6471610Z -    |
2019-10-09T20:04:56.6471610Z -    |
2019-10-09T20:04:56.6471965Z - LL |     get_future().await;
2019-10-09T20:04:56.6472156Z -    |     ^^^^^^^^^^^^^^^^^^
2019-10-09T20:04:56.6472362Z +    = note: the return type of a function must have a statically known size
2019-10-09T20:04:56.6472815Z - error: aborting due to previous error
2019-10-09T20:04:56.6472863Z + error: aborting due to 2 previous errors
2019-10-09T20:04:56.6472921Z 14 
2019-10-09T20:04:56.6473160Z - For more information about this error, try `rustc --explain E0698`.
2019-10-09T20:04:56.6473160Z - For more information about this error, try `rustc --explain E0698`.
2019-10-09T20:04:56.6473213Z + Some errors have detailed explanations: E0277, E0433.
2019-10-09T20:04:56.6473744Z + For more information about an error, try `rustc --explain E0277`.
2019-10-09T20:04:56.6473788Z 16 
2019-10-09T20:04:56.6473813Z 
2019-10-09T20:04:56.6473837Z 
2019-10-09T20:04:56.6474149Z The actual stderr differed from the expected stderr.
2019-10-09T20:04:56.6475064Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-error-span/async-error-span.stderr
2019-10-09T20:04:56.6475357Z To update references, rerun the tests and pass the `--bless` flag
2019-10-09T20:04:56.6475638Z To only update this specific test, also pass `--test-args async-await/async-error-span.rs`
2019-10-09T20:04:56.6475731Z error: 1 errors occurred comparing output.
2019-10-09T20:04:56.6475776Z status: exit code: 1
2019-10-09T20:04:56.6475776Z status: exit code: 1
2019-10-09T20:04:56.6476556Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-error-span.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-error-span" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-error-span/auxiliary" "-A" "unused"
2019-10-09T20:04:56.6476887Z ------------------------------------------
2019-10-09T20:04:56.6476921Z 
2019-10-09T20:04:56.6477132Z ------------------------------------------
2019-10-09T20:04:56.6477194Z stderr:
2019-10-09T20:04:56.6477194Z stderr:
2019-10-09T20:04:56.6477412Z ------------------------------------------
2019-10-09T20:04:56.6477465Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6477498Z 
2019-10-09T20:04:56.6477572Z error[E0277]: the trait bound `(): std::future::Future` is not satisfied
2019-10-09T20:04:56.6477863Z    |
2019-10-09T20:04:56.6478098Z LL | fn get_future() -> impl Future<Output = ()> {
2019-10-09T20:04:56.6478158Z    |                    ^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::future::Future` is not implemented for `()`
2019-10-09T20:04:56.6478365Z    |
2019-10-09T20:04:56.6478365Z    |
2019-10-09T20:04:56.6478432Z    = note: the return type of a function must have a statically known size
2019-10-09T20:04:56.6478677Z error: aborting due to 2 previous errors
2019-10-09T20:04:56.6478723Z 
2019-10-09T20:04:56.6478947Z Some errors have detailed explanations: E0277, E0433.
2019-10-09T20:04:56.6479826Z For more information about an error, try `rustc --explain E0277`.
---
2019-10-09T20:04:56.6480843Z ---- [ui] ui/async-await/async-closure.rs stdout ----
2019-10-09T20:04:56.6480874Z 
2019-10-09T20:04:56.6481096Z error: test compilation failed although it shouldn't!
2019-10-09T20:04:56.6481141Z status: exit code: 1
2019-10-09T20:04:56.6481832Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-closure/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-closure/auxiliary"
2019-10-09T20:04:56.6482299Z ------------------------------------------
2019-10-09T20:04:56.6482350Z 
2019-10-09T20:04:56.6482563Z ------------------------------------------
2019-10-09T20:04:56.6482609Z stderr:
2019-10-09T20:04:56.6482609Z stderr:
2019-10-09T20:04:56.6482978Z ------------------------------------------
2019-10-09T20:04:56.6483028Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6483264Z error: aborting due to previous error
2019-10-09T20:04:56.6483310Z 
2019-10-09T20:04:56.6483525Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6483556Z 
2019-10-09T20:04:56.6483556Z 
2019-10-09T20:04:56.6483742Z ------------------------------------------
2019-10-09T20:04:56.6483788Z 
2019-10-09T20:04:56.6483811Z 
2019-10-09T20:04:56.6484181Z ---- [ui] ui/async-await/async-fn-nonsend.rs stdout ----
2019-10-09T20:04:56.6484228Z diff of stderr:
2019-10-09T20:04:56.6484332Z 
2019-10-09T20:04:56.6484986Z - error[E0277]: `std::rc::Rc<()>` cannot be sent between threads safely
2019-10-09T20:04:56.6485406Z -    |
2019-10-09T20:04:56.6485406Z -    |
2019-10-09T20:04:56.6485623Z - LL | fn assert_send(_: impl Send) {}
2019-10-09T20:04:56.6486046Z - ...
2019-10-09T20:04:56.6486046Z - ...
2019-10-09T20:04:56.6486285Z - LL |     assert_send(local_dropped_before_await());
2019-10-09T20:04:56.6486532Z -    |     ^^^^^^^^^^^ `std::rc::Rc<()>` cannot be sent between threads safely
2019-10-09T20:04:56.6486712Z -    |
2019-10-09T20:04:56.6487018Z -    = help: within `impl std::future::Future`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<()>`
2019-10-09T20:04:56.6487273Z -    = note: required because it appears within the type `impl std::fmt::Debug`
2019-10-09T20:04:56.6487635Z -    = note: required because it appears within the type `{impl std::fmt::Debug, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, ()}`
2019-10-09T20:04:56.6488744Z -    = note: required because it appears within the type `[static generator@$DIR/async-fn-nonsend.rs:21:39: 26:2 {impl std::fmt::Debug, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, ()}]`
2019-10-09T20:04:56.6506880Z -    = note: required because it appears within the type `std::future::GenFuture<[static generator@$DIR/async-fn-nonsend.rs:21:39: 26:2 {impl std::fmt::Debug, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, ()}]>`
2019-10-09T20:04:56.6507510Z -    = note: required because it appears within the type `impl std::future::Future`
2019-10-09T20:04:56.6507826Z -    = note: required because it appears within the type `impl std::future::Future`
2019-10-09T20:04:56.6507889Z + error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6507935Z 17 
2019-10-09T20:04:56.6508342Z - error[E0277]: `std::rc::Rc<()>` cannot be sent between threads safely
2019-10-09T20:04:56.6509199Z -    |
2019-10-09T20:04:56.6509199Z -    |
2019-10-09T20:04:56.6509508Z - LL | fn assert_send(_: impl Send) {}
2019-10-09T20:04:56.6509989Z - ...
2019-10-09T20:04:56.6509989Z - ...
2019-10-09T20:04:56.6510205Z - LL |     assert_send(non_send_temporary_in_match());
2019-10-09T20:04:56.6510462Z -    |     ^^^^^^^^^^^ `std::rc::Rc<()>` cannot be sent between threads safely
2019-10-09T20:04:56.6510639Z -    |
2019-10-09T20:04:56.6510911Z -    = help: within `impl std::future::Future`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<()>`
2019-10-09T20:04:56.6511172Z -    = note: required because it appears within the type `impl std::fmt::Debug`
2019-10-09T20:04:56.6511708Z -    = note: required because it appears within the type `{fn(impl std::fmt::Debug) -> std::option::Option<impl std::fmt::Debug> {std::option::Option::<impl std::fmt::Debug>::Some}, fn() -> impl std::fmt::Debug {non_send}, impl std::fmt::Debug, std::option::Option<impl std::fmt::Debug>, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, ()}`
2019-10-09T20:04:56.6512338Z -    = note: required because it appears within the type `[static generator@$DIR/async-fn-nonsend.rs:28:40: 37:2 {fn(impl std::fmt::Debug) -> std::option::Option<impl std::fmt::Debug> {std::option::Option::<impl std::fmt::Debug>::Some}, fn() -> impl std::fmt::Debug {non_send}, impl std::fmt::Debug, std::option::Option<impl std::fmt::Debug>, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, ()}]`
2019-10-09T20:04:56.6512961Z -    = note: required because it appears within the type `std::future::GenFuture<[static generator@$DIR/async-fn-nonsend.rs:28:40: 37:2 {fn(impl std::fmt::Debug) -> std::option::Option<impl std::fmt::Debug> {std::option::Option::<impl std::fmt::Debug>::Some}, fn() -> impl std::fmt::Debug {non_send}, impl std::fmt::Debug, std::option::Option<impl std::fmt::Debug>, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, ()}]>`
2019-10-09T20:04:56.6513272Z -    = note: required because it appears within the type `impl std::future::Future`
2019-10-09T20:04:56.6513520Z -    = note: required because it appears within the type `impl std::future::Future`
2019-10-09T20:04:56.6513636Z 34 
2019-10-09T20:04:56.6513636Z 34 
2019-10-09T20:04:56.6513871Z - error[E0277]: `dyn std::fmt::Write` cannot be sent between threads safely
2019-10-09T20:04:56.6514266Z -    |
2019-10-09T20:04:56.6514266Z -    |
2019-10-09T20:04:56.6514937Z - LL | fn assert_send(_: impl Send) {}
2019-10-09T20:04:56.6515389Z - ...
2019-10-09T20:04:56.6515389Z - ...
2019-10-09T20:04:56.6515612Z - LL |     assert_send(non_sync_with_method_call());
2019-10-09T20:04:56.6515897Z -    |     ^^^^^^^^^^^ `dyn std::fmt::Write` cannot be sent between threads safely
2019-10-09T20:04:56.6516341Z -    = help: the trait `std::marker::Send` is not implemented for `dyn std::fmt::Write`
2019-10-09T20:04:56.6516656Z -    = note: required because of the requirements on the impl of `std::marker::Send` for `&mut dyn std::fmt::Write`
2019-10-09T20:04:56.6516922Z -    = note: required because it appears within the type `std::fmt::Formatter<'_>`
2019-10-09T20:04:56.6517214Z -    = note: required because of the requirements on the impl of `std::marker::Send` for `&mut std::fmt::Formatter<'_>`
2019-10-09T20:04:56.6517214Z -    = note: required because of the requirements on the impl of `std::marker::Send` for `&mut std::fmt::Formatter<'_>`
2019-10-09T20:04:56.6517617Z -    = note: required because it appears within the type `for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, ()}`
2019-10-09T20:04:56.6518323Z -    = note: required because it appears within the type `[static generator@$DIR/async-fn-nonsend.rs:39:38: 45:2 for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, ()}]`
2019-10-09T20:04:56.6519068Z -    = note: required because it appears within the type `std::future::GenFuture<[static generator@$DIR/async-fn-nonsend.rs:39:38: 45:2 for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, ()}]>`
2019-10-09T20:04:56.6519390Z -    = note: required because it appears within the type `impl std::future::Future`
2019-10-09T20:04:56.6519642Z -    = note: required because it appears within the type `impl std::future::Future`
2019-10-09T20:04:56.6519814Z - 
2019-10-09T20:04:56.6520080Z - error[E0277]: `*mut (dyn std::ops::Fn() + 'static)` cannot be shared between threads safely
2019-10-09T20:04:56.6520460Z -    |
2019-10-09T20:04:56.6520460Z -    |
2019-10-09T20:04:56.6520656Z - LL | fn assert_send(_: impl Send) {}
2019-10-09T20:04:56.6521094Z - ...
2019-10-09T20:04:56.6521094Z - ...
2019-10-09T20:04:56.6521313Z - LL |     assert_send(non_sync_with_method_call());
2019-10-09T20:04:56.6521601Z -    |     ^^^^^^^^^^^ `*mut (dyn std::ops::Fn() + 'static)` cannot be shared between threads safely
2019-10-09T20:04:56.6521778Z -    |
2019-10-09T20:04:56.6522078Z -    = help: within `std::fmt::ArgumentV1<'_>`, the trait `std::marker::Sync` is not implemented for `*mut (dyn std::ops::Fn() + 'static)`
2019-10-09T20:04:56.6522383Z -    = note: required because it appears within the type `std::marker::PhantomData<*mut (dyn std::ops::Fn() + 'static)>`
2019-10-09T20:04:56.6522625Z -    = note: required because it appears within the type `core::fmt::Void`
2019-10-09T20:04:56.6522861Z -    = note: required because it appears within the type `&core::fmt::Void`
2019-10-09T20:04:56.6523125Z -    = note: required because it appears within the type `std::fmt::ArgumentV1<'_>`
2019-10-09T20:04:56.6523434Z -    = note: required because of the requirements on the impl of `std::marker::Send` for `std::slice::Iter<'_, std::fmt::ArgumentV1<'_>>`
2019-10-09T20:04:56.6523707Z -    = note: required because it appears within the type `std::fmt::Formatter<'_>`
2019-10-09T20:04:56.6523987Z -    = note: required because of the requirements on the impl of `std::marker::Send` for `&mut std::fmt::Formatter<'_>`
2019-10-09T20:04:56.6524355Z -    = note: required because it appears within the type `for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, ()}`
2019-10-09T20:04:56.6525264Z -    = note: required because it appears within the type `[static generator@$DIR/async-fn-nonsend.rs:39:38: 45:2 for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, ()}]`
2019-10-09T20:04:56.6525762Z -    = note: required because it appears within the type `std::future::GenFuture<[static generator@$DIR/async-fn-nonsend.rs:39:38: 45:2 for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, ()}]>`
2019-10-09T20:04:56.6526055Z -    = note: required because it appears within the type `impl std::future::Future`
2019-10-09T20:04:56.6526309Z -    = note: required because it appears within the type `impl std::future::Future`
2019-10-09T20:04:56.6526713Z - error: aborting due to 4 previous errors
2019-10-09T20:04:56.6526893Z - 
2019-10-09T20:04:56.6527130Z - For more information about this error, try `rustc --explain E0277`.
2019-10-09T20:04:56.6527391Z + For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6527391Z + For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6527440Z 80 
2019-10-09T20:04:56.6527471Z 
2019-10-09T20:04:56.6527497Z 
2019-10-09T20:04:56.6527541Z The actual stderr differed from the expected stderr.
2019-10-09T20:04:56.6528177Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-nonsend/async-fn-nonsend.stderr
2019-10-09T20:04:56.6528486Z To update references, rerun the tests and pass the `--bless` flag
2019-10-09T20:04:56.6528772Z To only update this specific test, also pass `--test-args async-await/async-fn-nonsend.rs`
2019-10-09T20:04:56.6528872Z error: 1 errors occurred comparing output.
2019-10-09T20:04:56.6528916Z status: exit code: 1
2019-10-09T20:04:56.6528916Z status: exit code: 1
2019-10-09T20:04:56.6529677Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-fn-nonsend.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-nonsend" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--crate-type" "lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-nonsend/auxiliary" "-A" "unused"
2019-10-09T20:04:56.6530010Z ------------------------------------------
2019-10-09T20:04:56.6530044Z 
2019-10-09T20:04:56.6530250Z ------------------------------------------
2019-10-09T20:04:56.6530312Z stderr:
2019-10-09T20:04:56.6530312Z stderr:
2019-10-09T20:04:56.6530732Z ------------------------------------------
2019-10-09T20:04:56.6530786Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6530982Z error: aborting due to previous error
2019-10-09T20:04:56.6531012Z 
2019-10-09T20:04:56.6531251Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6531285Z 
2019-10-09T20:04:56.6531285Z 
2019-10-09T20:04:56.6531511Z ------------------------------------------
2019-10-09T20:04:56.6531543Z 
2019-10-09T20:04:56.6531569Z 
2019-10-09T20:04:56.6531800Z ---- [ui] ui/async-await/async-fn-send-uses-nonsend.rs stdout ----
2019-10-09T20:04:56.6531861Z 
2019-10-09T20:04:56.6532085Z error: test compilation failed although it shouldn't!
2019-10-09T20:04:56.6532133Z status: exit code: 1
2019-10-09T20:04:56.6532949Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-fn-send-uses-nonsend.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-send-uses-nonsend" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--crate-type" "lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-send-uses-nonsend/auxiliary" "-A" "unused"
2019-10-09T20:04:56.6533274Z ------------------------------------------
2019-10-09T20:04:56.6533307Z 
2019-10-09T20:04:56.6533529Z ------------------------------------------
2019-10-09T20:04:56.6533596Z stderr:
2019-10-09T20:04:56.6533596Z stderr:
2019-10-09T20:04:56.6533805Z ------------------------------------------
2019-10-09T20:04:56.6533866Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6533962Z error: aborting due to previous error
2019-10-09T20:04:56.6533995Z 
2019-10-09T20:04:56.6534234Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6534269Z 
2019-10-09T20:04:56.6534269Z 
2019-10-09T20:04:56.6534789Z ------------------------------------------
2019-10-09T20:04:56.6534826Z 
2019-10-09T20:04:56.6534852Z 
2019-10-09T20:04:56.6535084Z ---- [ui] ui/async-await/async-fn-size-moved-locals.rs stdout ----
2019-10-09T20:04:56.6535137Z 
2019-10-09T20:04:56.6535359Z error: test compilation failed although it shouldn't!
2019-10-09T20:04:56.6535407Z status: exit code: 1
2019-10-09T20:04:56.6536806Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-fn-size-moved-locals.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-size-moved-locals/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-size-moved-locals/auxiliary"
2019-10-09T20:04:56.6537535Z ------------------------------------------
2019-10-09T20:04:56.6537576Z 
2019-10-09T20:04:56.6538496Z ------------------------------------------
2019-10-09T20:04:56.6538553Z stderr:
2019-10-09T20:04:56.6538553Z stderr:
2019-10-09T20:04:56.6538825Z ------------------------------------------
2019-10-09T20:04:56.6538879Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6538977Z error: aborting due to previous error
2019-10-09T20:04:56.6539020Z 
2019-10-09T20:04:56.6539265Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6539301Z 
2019-10-09T20:04:56.6539301Z 
2019-10-09T20:04:56.6539533Z ------------------------------------------
2019-10-09T20:04:56.6539974Z 
2019-10-09T20:04:56.6540009Z 
2019-10-09T20:04:56.6540474Z ---- [ui] ui/async-await/async-fn-size-uninit-locals.rs stdout ----
2019-10-09T20:04:56.6540514Z 
2019-10-09T20:04:56.6541350Z error: test compilation failed although it shouldn't!
2019-10-09T20:04:56.6541406Z status: exit code: 1
2019-10-09T20:04:56.6545184Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-fn-size-uninit-locals.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-size-uninit-locals/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-size-uninit-locals/auxiliary"
2019-10-09T20:04:56.6545830Z ------------------------------------------
2019-10-09T20:04:56.6545892Z 
2019-10-09T20:04:56.6546188Z ------------------------------------------
2019-10-09T20:04:56.6546235Z stderr:
2019-10-09T20:04:56.6546235Z stderr:
2019-10-09T20:04:56.6546646Z ------------------------------------------
2019-10-09T20:04:56.6546714Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6546824Z error: aborting due to previous error
2019-10-09T20:04:56.6546854Z 
2019-10-09T20:04:56.6547166Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6547203Z 
2019-10-09T20:04:56.6547203Z 
2019-10-09T20:04:56.6547658Z ------------------------------------------
2019-10-09T20:04:56.6547702Z 
2019-10-09T20:04:56.6547727Z 
2019-10-09T20:04:56.6548327Z ---- [ui] ui/async-await/async-fn-size.rs stdout ----
2019-10-09T20:04:56.6548370Z 
2019-10-09T20:04:56.6548638Z error: test compilation failed although it shouldn't!
2019-10-09T20:04:56.6548698Z status: exit code: 1
2019-10-09T20:04:56.6549621Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-fn-size.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-size/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-size/auxiliary"
2019-10-09T20:04:56.6550159Z ------------------------------------------
2019-10-09T20:04:56.6550213Z 
2019-10-09T20:04:56.6550586Z ------------------------------------------
2019-10-09T20:04:56.6550843Z stderr:
2019-10-09T20:04:56.6550843Z stderr:
2019-10-09T20:04:56.6551175Z ------------------------------------------
2019-10-09T20:04:56.6551417Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6551811Z error: aborting due to previous error
2019-10-09T20:04:56.6551864Z 
2019-10-09T20:04:56.6552439Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6552485Z 
2019-10-09T20:04:56.6552485Z 
2019-10-09T20:04:56.6552706Z ------------------------------------------
2019-10-09T20:04:56.6552761Z 
2019-10-09T20:04:56.6552787Z 
2019-10-09T20:04:56.6553240Z ---- [ui] ui/async-await/async-with-closure.rs stdout ----
2019-10-09T20:04:56.6553284Z 
2019-10-09T20:04:56.6553534Z error: test compilation failed although it shouldn't!
2019-10-09T20:04:56.6553585Z status: exit code: 1
2019-10-09T20:04:56.6554942Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-with-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-with-closure" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-with-closure/auxiliary" "-A" "unused"
2019-10-09T20:04:56.6556624Z ------------------------------------------
2019-10-09T20:04:56.6556697Z 
2019-10-09T20:04:56.6556928Z ------------------------------------------
2019-10-09T20:04:56.6556976Z stderr:
2019-10-09T20:04:56.6556976Z stderr:
2019-10-09T20:04:56.6557415Z ------------------------------------------
2019-10-09T20:04:56.6557505Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6557586Z error: aborting due to previous error
2019-10-09T20:04:56.6557631Z 
2019-10-09T20:04:56.6558080Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6558125Z 
2019-10-09T20:04:56.6558125Z 
2019-10-09T20:04:56.6558392Z ------------------------------------------
2019-10-09T20:04:56.6558459Z 
2019-10-09T20:04:56.6558488Z 
2019-10-09T20:04:56.6558905Z ---- [ui] ui/async-await/await-into-future.rs stdout ----
2019-10-09T20:04:56.6558948Z 
2019-10-09T20:04:56.6559219Z error: test compilation failed although it shouldn't!
2019-10-09T20:04:56.6559292Z status: exit code: 1
2019-10-09T20:04:56.6560562Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/await-into-future.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/await-into-future" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/await-into-future/auxiliary" "-A" "unused"
2019-10-09T20:04:56.6560970Z ------------------------------------------
2019-10-09T20:04:56.6561005Z 
2019-10-09T20:04:56.6561495Z ------------------------------------------
2019-10-09T20:04:56.6561542Z stderr:
2019-10-09T20:04:56.6561542Z stderr:
2019-10-09T20:04:56.6561745Z ------------------------------------------
2019-10-09T20:04:56.6561815Z error[E0432]: unresolved import `std::future::IntoFuture`
2019-10-09T20:04:56.6562316Z   --> /checkout/src/test/ui/async-await/await-into-future.rs:5:28
2019-10-09T20:04:56.6562372Z    |
2019-10-09T20:04:56.6562440Z LL | use std::{future::{Future, IntoFuture}, pin::Pin};
2019-10-09T20:04:56.6562489Z    |                            ^^^^^^^^^^ no `IntoFuture` in `future`
2019-10-09T20:04:56.6562520Z 
2019-10-09T20:04:56.6562582Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6563006Z error: aborting due to 2 previous errors
2019-10-09T20:04:56.6563034Z 
2019-10-09T20:04:56.6563098Z Some errors have detailed explanations: E0432, E0433.
2019-10-09T20:04:56.6563707Z For more information about an error, try `rustc --explain E0432`.
---
2019-10-09T20:04:56.6565215Z 
2019-10-09T20:04:56.6565256Z 137 LL | }
2019-10-09T20:04:56.6565299Z 138    | ^ unexpected token
2019-10-09T20:04:56.6565341Z 139 
2019-10-09T20:04:56.6565407Z + error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6565625Z + 
2019-10-09T20:04:56.6565674Z 140 error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6566075Z 142    |
2019-10-09T20:04:56.6566105Z 
2019-10-09T20:04:56.6566161Z 242    = help: the trait `std::ops::Try` is not implemented for `impl std::future::Future`
2019-10-09T20:04:56.6566409Z 243    = note: required by `std::ops::Try::into_result`
---
2019-10-09T20:04:56.6567755Z 
2019-10-09T20:04:56.6567799Z 
2019-10-09T20:04:56.6568162Z The actual stderr differed from the expected stderr.
2019-10-09T20:04:56.6568577Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/await-keyword/incorrect-syntax-suggestions/incorrect-syntax-suggestions.stderr
2019-10-09T20:04:56.6569071Z To update references, rerun the tests and pass the `--bless` flag
2019-10-09T20:04:56.6569423Z To only update this specific test, also pass `--test-args async-await/await-keyword/incorrect-syntax-suggestions.rs`
2019-10-09T20:04:56.6569536Z error: 1 errors occurred comparing output.
2019-10-09T20:04:56.6569759Z status: exit code: 1
2019-10-09T20:04:56.6569759Z status: exit code: 1
2019-10-09T20:04:56.6570840Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/await-keyword/incorrect-syntax-suggestions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/await-keyword/incorrect-syntax-suggestions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/await-keyword/incorrect-syntax-suggestions/auxiliary" "-A" "unused"
2019-10-09T20:04:56.6571486Z ------------------------------------------
2019-10-09T20:04:56.6571559Z 
2019-10-09T20:04:56.6571803Z ------------------------------------------
2019-10-09T20:04:56.6571850Z stderr:
2019-10-09T20:04:56.6571850Z stderr:
2019-10-09T20:04:56.6572318Z ------------------------------------------
2019-10-09T20:04:56.6572380Z error: incorrect use of `await`
2019-10-09T20:04:56.6572663Z   --> /checkout/src/test/ui/async-await/await-keyword/incorrect-syntax-suggestions.rs:8:13
2019-10-09T20:04:56.6572880Z    |
2019-10-09T20:04:56.6572963Z LL |     let _ = await bar(); //~ ERROR incorrect use of `await`
2019-10-09T20:04:56.6573017Z    |             ^^^^^^^^^^^ help: `await` is a postfix operation: `bar().await`
2019-10-09T20:04:56.6573266Z error: incorrect use of `await`
2019-10-09T20:04:56.6573888Z   --> /checkout/src/test/ui/async-await/await-keyword/incorrect-syntax-suggestions.rs:12:13
2019-10-09T20:04:56.6573952Z    |
2019-10-09T20:04:56.6573952Z    |
2019-10-09T20:04:56.6574156Z LL |     let _ = await? bar(); //~ ERROR incorrect use of `await`
2019-10-09T20:04:56.6574229Z    |             ^^^^^^^^^^^^ help: `await` is a postfix operation: `bar().await?`
2019-10-09T20:04:56.6574305Z error: incorrect use of `await`
2019-10-09T20:04:56.6575653Z   --> /checkout/src/test/ui/async-await/await-keyword/incorrect-syntax-suggestions.rs:16:13
2019-10-09T20:04:56.6575712Z    |
2019-10-09T20:04:56.6575712Z    |
2019-10-09T20:04:56.6575759Z LL |     let _ = await bar()?; //~ ERROR incorrect use of `await`
2019-10-09T20:04:56.6576000Z    |             ^^^^^^^^^^^^ help: `await` is a postfix operation: `bar()?.await`
2019-10-09T20:04:56.6576097Z error: incorrect use of `await`
2019-10-09T20:04:56.6576463Z   --> /checkout/src/test/ui/async-await/await-keyword/incorrect-syntax-suggestions.rs:21:13
2019-10-09T20:04:56.6576530Z    |
2019-10-09T20:04:56.6576530Z    |
2019-10-09T20:04:56.6576577Z LL |     let _ = await { bar() }; //~ ERROR incorrect use of `await`
2019-10-09T20:04:56.6576820Z    |             ^^^^^^^^^^^^^^^ help: `await` is a postfix operation: `{ bar() }.await`
2019-10-09T20:04:56.6576918Z error: incorrect use of `await`
2019-10-09T20:04:56.6577258Z   --> /checkout/src/test/ui/async-await/await-keyword/incorrect-syntax-suggestions.rs:25:13
2019-10-09T20:04:56.6577328Z    |
2019-10-09T20:04:56.6577328Z    |
2019-10-09T20:04:56.6577374Z LL |     let _ = await(bar()); //~ ERROR incorrect use of `await`
2019-10-09T20:04:56.6577586Z    |             ^^^^^^^^^^^^ help: `await` is a postfix operation: `(bar()).await`
2019-10-09T20:04:56.6577704Z error: incorrect use of `await`
2019-10-09T20:04:56.6578196Z   --> /checkout/src/test/ui/async-await/await-keyword/incorrect-syntax-suggestions.rs:29:13
2019-10-09T20:04:56.6578409Z    |
2019-10-09T20:04:56.6578409Z    |
2019-10-09T20:04:56.6578524Z LL |     let _ = await { bar() }?; //~ ERROR incorrect use of `await`
2019-10-09T20:04:56.6578577Z    |             ^^^^^^^^^^^^^^^ help: `await` is a postfix operation: `{ bar() }.await`
2019-10-09T20:04:56.6578675Z error: incorrect use of `await`
2019-10-09T20:04:56.6579145Z   --> /checkout/src/test/ui/async-await/await-keyword/incorrect-syntax-suggestions.rs:33:14
2019-10-09T20:04:56.6579228Z    |
2019-10-09T20:04:56.6579228Z    |
2019-10-09T20:04:56.6579295Z LL |     let _ = (await bar())?; //~ ERROR incorrect use of `await`
2019-10-09T20:04:56.6579348Z    |              ^^^^^^^^^^^ help: `await` is a postfix operation: `bar().await`
2019-10-09T20:04:56.6579420Z error: incorrect use of `await`
2019-10-09T20:04:56.6579758Z   --> /checkout/src/test/ui/async-await/await-keyword/incorrect-syntax-suggestions.rs:37:24
2019-10-09T20:04:56.6579960Z    |
2019-10-09T20:04:56.6579960Z    |
2019-10-09T20:04:56.6580025Z LL |     let _ = bar().await(); //~ ERROR incorrect use of `await`
2019-10-09T20:04:56.6580100Z    |                        ^^ help: `await` is not a method call, remove the parentheses
2019-10-09T20:04:56.6580183Z error: incorrect use of `await`
2019-10-09T20:04:56.6580516Z   --> /checkout/src/test/ui/async-await/await-keyword/incorrect-syntax-suggestions.rs:41:24
2019-10-09T20:04:56.6580583Z    |
2019-10-09T20:04:56.6580583Z    |
2019-10-09T20:04:56.6580798Z LL |     let _ = bar().await()?; //~ ERROR incorrect use of `await`
2019-10-09T20:04:56.6580855Z    |                        ^^ help: `await` is not a method call, remove the parentheses
2019-10-09T20:04:56.6580949Z error: incorrect use of `await`
2019-10-09T20:04:56.6581272Z   --> /checkout/src/test/ui/async-await/await-keyword/incorrect-syntax-suggestions.rs:53:13
2019-10-09T20:04:56.6581338Z    |
2019-10-09T20:04:56.6581338Z    |
2019-10-09T20:04:56.6581519Z LL |     let _ = await bar(); //~ ERROR `await` is only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6581604Z    |             ^^^^^^^^^^^ help: `await` is a postfix operation: `bar().await`
2019-10-09T20:04:56.6581846Z error: incorrect use of `await`
2019-10-09T20:04:56.6582581Z   --> /checkout/src/test/ui/async-await/await-keyword/incorrect-syntax-suggestions.rs:58:13
2019-10-09T20:04:56.6582780Z    |
2019-10-09T20:04:56.6582780Z    |
2019-10-09T20:04:56.6582869Z LL |     let _ = await? bar(); //~ ERROR `await` is only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6582924Z    |             ^^^^^^^^^^^^ help: `await` is a postfix operation: `bar().await?`
2019-10-09T20:04:56.6582997Z error: incorrect use of `await`
2019-10-09T20:04:56.6583664Z   --> /checkout/src/test/ui/async-await/await-keyword/incorrect-syntax-suggestions.rs:63:13
2019-10-09T20:04:56.6583727Z    |
2019-10-09T20:04:56.6583727Z    |
2019-10-09T20:04:56.6583775Z LL |     let _ = await bar()?; //~ ERROR `await` is only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6583852Z    |             ^^^^^^^^^^^^ help: `await` is a postfix operation: `bar()?.await`
2019-10-09T20:04:56.6584118Z error: incorrect use of `await`
2019-10-09T20:04:56.6585059Z   --> /checkout/src/test/ui/async-await/await-keyword/incorrect-syntax-suggestions.rs:68:14
2019-10-09T20:04:56.6585125Z    |
2019-10-09T20:04:56.6585125Z    |
2019-10-09T20:04:56.6585190Z LL |     let _ = (await bar())?; //~ ERROR `await` is only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6585263Z    |              ^^^^^^^^^^^ help: `await` is a postfix operation: `bar().await`
2019-10-09T20:04:56.6585339Z error: incorrect use of `await`
2019-10-09T20:04:56.6585900Z   --> /checkout/src/test/ui/async-await/await-keyword/incorrect-syntax-suggestions.rs:73:24
2019-10-09T20:04:56.6585986Z    |
2019-10-09T20:04:56.6585986Z    |
2019-10-09T20:04:56.6586037Z LL |     let _ = bar().await(); //~ ERROR `await` is only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6586092Z    |                        ^^ help: `await` is not a method call, remove the parentheses
2019-10-09T20:04:56.6586183Z error: incorrect use of `await`
2019-10-09T20:04:56.6586696Z   --> /checkout/src/test/ui/async-await/await-keyword/incorrect-syntax-suggestions.rs:78:24
2019-10-09T20:04:56.6586779Z    |
2019-10-09T20:04:56.6586779Z    |
2019-10-09T20:04:56.6586841Z LL |     let _ = bar().await()?; //~ ERROR `await` is only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6586897Z    |                        ^^ help: `await` is not a method call, remove the parentheses
2019-10-09T20:04:56.6586990Z error: incorrect use of `await`
2019-10-09T20:04:56.6587480Z   --> /checkout/src/test/ui/async-await/await-keyword/incorrect-syntax-suggestions.rs:106:13
2019-10-09T20:04:56.6587543Z    |
2019-10-09T20:04:56.6587543Z    |
2019-10-09T20:04:56.6587612Z LL |     let _ = await!(bar()); //~ ERROR incorrect use of `await`
2019-10-09T20:04:56.6587666Z    |             ^^^^^^^^^^^^^ help: `await` is a postfix operation: `bar().await`
2019-10-09T20:04:56.6587739Z error: incorrect use of `await`
2019-10-09T20:04:56.6588234Z   --> /checkout/src/test/ui/async-await/await-keyword/incorrect-syntax-suggestions.rs:110:13
2019-10-09T20:04:56.6588306Z    |
2019-10-09T20:04:56.6588306Z    |
2019-10-09T20:04:56.6588565Z LL |     let _ = await!(bar())?; //~ ERROR incorrect use of `await`
2019-10-09T20:04:56.6588721Z    |             ^^^^^^^^^^^^^ help: `await` is a postfix operation: `bar().await`
2019-10-09T20:04:56.6588970Z error: incorrect use of `await`
2019-10-09T20:04:56.6589373Z   --> /checkout/src/test/ui/async-await/await-keyword/incorrect-syntax-suggestions.rs:115:17
2019-10-09T20:04:56.6589425Z    |
2019-10-09T20:04:56.6589425Z    |
2019-10-09T20:04:56.6589472Z LL |         let _ = await!(bar())?; //~ ERROR incorrect use of `await`
2019-10-09T20:04:56.6589712Z    |                 ^^^^^^^^^^^^^ help: `await` is a postfix operation: `bar().await`
2019-10-09T20:04:56.6590002Z error: incorrect use of `await`
2019-10-09T20:04:56.6590891Z   --> /checkout/src/test/ui/async-await/await-keyword/incorrect-syntax-suggestions.rs:123:17
2019-10-09T20:04:56.6590981Z    |
2019-10-09T20:04:56.6590981Z    |
2019-10-09T20:04:56.6591195Z LL |         let _ = await!(bar())?; //~ ERROR incorrect use of `await`
2019-10-09T20:04:56.6591247Z    |                 ^^^^^^^^^^^^^ help: `await` is a postfix operation: `bar().await`
2019-10-09T20:04:56.6591649Z error: expected expression, found `=>`
2019-10-09T20:04:56.6592009Z   --> /checkout/src/test/ui/async-await/await-keyword/incorrect-syntax-suggestions.rs:131:25
2019-10-09T20:04:56.6592058Z    |
2019-10-09T20:04:56.6592058Z    |
2019-10-09T20:04:56.6592324Z LL |     match await { await => () }
2019-10-09T20:04:56.6592629Z    |                   ----- ^^ expected expression
2019-10-09T20:04:56.6592748Z    |                   while parsing this incorrect await expression
2019-10-09T20:04:56.6592781Z 
2019-10-09T20:04:56.6592821Z error: incorrect use of `await`
2019-10-09T20:04:56.6593302Z   --> /checkout/src/test/ui/async-await/await-keyword/incorrect-syntax-suggestions.rs:131:11
2019-10-09T20:04:56.6593302Z   --> /checkout/src/test/ui/async-await/await-keyword/incorrect-syntax-suggestions.rs:131:11
2019-10-09T20:04:56.6593385Z    |
2019-10-09T20:04:56.6593438Z LL |     match await { await => () }
2019-10-09T20:04:56.6593488Z    |           ^^^^^^^^^^^^^^^^^^^^^ help: `await` is a postfix operation: `{ await => () }.await`
2019-10-09T20:04:56.6593542Z 
2019-10-09T20:04:56.6593593Z error: expected one of `.`, `?`, `{`, or an operator, found `}`
2019-10-09T20:04:56.6595015Z    |
2019-10-09T20:04:56.6595015Z    |
2019-10-09T20:04:56.6595132Z LL |     match await { await => () }
2019-10-09T20:04:56.6595987Z    |     -----                      - expected one of `.`, `?`, `{`, or an operator here
2019-10-09T20:04:56.6613517Z    |     while parsing this match expression
2019-10-09T20:04:56.6613577Z ...
2019-10-09T20:04:56.6613577Z ...
2019-10-09T20:04:56.6613775Z LL | } //~ ERROR expected one of `.`, `?`, `{`, or an operator, found `}`
2019-10-09T20:04:56.6613882Z 
2019-10-09T20:04:56.6613882Z 
2019-10-09T20:04:56.6613926Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6613990Z 
2019-10-09T20:04:56.6614035Z error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6614791Z    |
2019-10-09T20:04:56.6614791Z    |
2019-10-09T20:04:56.6615052Z LL | fn foo9() -> Result<(), ()> {
2019-10-09T20:04:56.6615442Z    |    ---- this is not `async`
2019-10-09T20:04:56.6615506Z LL |     let _ = await bar(); //~ ERROR `await` is only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6615588Z    |             ^^^^^^^^^^^ only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6615620Z 
2019-10-09T20:04:56.6615663Z error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6617194Z    |
2019-10-09T20:04:56.6617194Z    |
2019-10-09T20:04:56.6617688Z LL | fn foo10() -> Result<(), ()> {
2019-10-09T20:04:56.6617935Z    |    ----- this is not `async`
2019-10-09T20:04:56.6618306Z LL |     let _ = await? bar(); //~ ERROR `await` is only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6618375Z    |             ^^^^^^^^^^^^ only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6618409Z 
2019-10-09T20:04:56.6618481Z error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6619072Z    |
2019-10-09T20:04:56.6619072Z    |
2019-10-09T20:04:56.6619378Z LL | fn foo11() -> Result<(), ()> {
2019-10-09T20:04:56.6619592Z    |    ----- this is not `async`
2019-10-09T20:04:56.6619647Z LL |     let _ = await bar()?; //~ ERROR `await` is only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6619901Z    |             ^^^^^^^^^^^^ only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6619939Z 
2019-10-09T20:04:56.6620192Z error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6621166Z    |
2019-10-09T20:04:56.6621166Z    |
2019-10-09T20:04:56.6622000Z LL | fn foo12() -> Result<(), ()> {
2019-10-09T20:04:56.6625503Z    |    ----- this is not `async`
2019-10-09T20:04:56.6625620Z LL |     let _ = (await bar())?; //~ ERROR `await` is only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6625676Z    |              ^^^^^^^^^^^ only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6625711Z 
2019-10-09T20:04:56.6625774Z error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6631274Z    |
2019-10-09T20:04:56.6631274Z    |
2019-10-09T20:04:56.6631649Z LL | fn foo13() -> Result<(), ()> {
2019-10-09T20:04:56.6632174Z    |    ----- this is not `async`
2019-10-09T20:04:56.6632258Z LL |     let _ = bar().await(); //~ ERROR `await` is only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6632317Z    |             ^^^^^^^^^^^ only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6632373Z 
2019-10-09T20:04:56.6632418Z error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6633383Z    |
2019-10-09T20:04:56.6633383Z    |
2019-10-09T20:04:56.6634174Z LL | fn foo14() -> Result<(), ()> {
2019-10-09T20:04:56.6634402Z    |    ----- this is not `async`
2019-10-09T20:04:56.6634945Z LL |     let _ = bar().await()?; //~ ERROR `await` is only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6635010Z    |             ^^^^^^^^^^^ only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6635044Z 
2019-10-09T20:04:56.6635090Z error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6653565Z    |
2019-10-09T20:04:56.6653565Z    |
2019-10-09T20:04:56.6677071Z LL | fn foo15() -> Result<(), ()> {
2019-10-09T20:04:56.6677329Z    |    ----- this is not `async`
2019-10-09T20:04:56.6677391Z LL |     let _ = bar().await; //~ ERROR `await` is only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6677468Z    |             ^^^^^^^^^^^ only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6677505Z 
2019-10-09T20:04:56.6677554Z error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6678053Z    |
2019-10-09T20:04:56.6678053Z    |
2019-10-09T20:04:56.6678254Z LL | fn foo16() -> Result<(), ()> {
2019-10-09T20:04:56.6678448Z    |    ----- this is not `async`
2019-10-09T20:04:56.6678521Z LL |     let _ = bar().await?; //~ ERROR `await` is only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6678585Z    |             ^^^^^^^^^^^ only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6678618Z 
2019-10-09T20:04:56.6678670Z error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6678995Z    |
2019-10-09T20:04:56.6678995Z    |
2019-10-09T20:04:56.6679189Z LL |     fn foo() -> Result<(), ()> {
2019-10-09T20:04:56.6679404Z    |        --- this is not `async`
2019-10-09T20:04:56.6679616Z LL |         let _ = bar().await?; //~ ERROR `await` is only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6679668Z    |                 ^^^^^^^^^^^ only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6679718Z 
2019-10-09T20:04:56.6679761Z error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6680261Z    |
2019-10-09T20:04:56.6680301Z LL |     let foo = || {
2019-10-09T20:04:56.6680541Z    |               -- this is not `async`
2019-10-09T20:04:56.6680541Z    |               -- this is not `async`
2019-10-09T20:04:56.6680686Z LL |         let _ = bar().await?; //~ ERROR `await` is only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6680771Z    |                 ^^^^^^^^^^^ only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6680801Z 
2019-10-09T20:04:56.6680845Z error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6681188Z    |
2019-10-09T20:04:56.6681188Z    |
2019-10-09T20:04:56.6681376Z LL |     fn foo() -> Result<(), ()> {
2019-10-09T20:04:56.6681583Z    |        --- this is not `async`
2019-10-09T20:04:56.6681632Z LL |         let _ = await!(bar())?; //~ ERROR incorrect use of `await`
2019-10-09T20:04:56.6681683Z    |                 ^^^^^^^^^^^^^ only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6681721Z 
2019-10-09T20:04:56.6681782Z error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6682082Z    |
2019-10-09T20:04:56.6682139Z LL |     let foo = || {
2019-10-09T20:04:56.6682338Z    |               -- this is not `async`
2019-10-09T20:04:56.6682338Z    |               -- this is not `async`
2019-10-09T20:04:56.6682387Z LL |         let _ = await!(bar())?; //~ ERROR incorrect use of `await`
2019-10-09T20:04:56.6682455Z    |                 ^^^^^^^^^^^^^ only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6682529Z error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
2019-10-09T20:04:56.6682792Z   --> /checkout/src/test/ui/async-await/await-keyword/incorrect-syntax-suggestions.rs:16:19
2019-10-09T20:04:56.6682838Z    |
2019-10-09T20:04:56.6682838Z    |
2019-10-09T20:04:56.6682888Z LL |     let _ = await bar()?; //~ ERROR incorrect use of `await`
2019-10-09T20:04:56.6683007Z    |
2019-10-09T20:04:56.6683052Z    = help: the trait `std::ops::Try` is not implemented for `impl std::future::Future`
2019-10-09T20:04:56.6683099Z    = note: required by `std::ops::Try::into_result`
2019-10-09T20:04:56.6683146Z 
---
2019-10-09T20:04:56.6683994Z ---- [ui] ui/async-await/await-unsize.rs stdout ----
2019-10-09T20:04:56.6684025Z 
2019-10-09T20:04:56.6684234Z error: test compilation failed although it shouldn't!
2019-10-09T20:04:56.6684718Z status: exit code: 1
2019-10-09T20:04:56.6685543Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/await-unsize.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/await-unsize" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/await-unsize/auxiliary" "-A" "unused"
2019-10-09T20:04:56.6685878Z ------------------------------------------
2019-10-09T20:04:56.6685914Z 
2019-10-09T20:04:56.6686145Z ------------------------------------------
2019-10-09T20:04:56.6686191Z stderr:
2019-10-09T20:04:56.6686191Z stderr:
2019-10-09T20:04:56.6686568Z ------------------------------------------
2019-10-09T20:04:56.6686643Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6686800Z error: aborting due to previous error
2019-10-09T20:04:56.6686840Z 
2019-10-09T20:04:56.6687127Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6687161Z 
2019-10-09T20:04:56.6687161Z 
2019-10-09T20:04:56.6687369Z ------------------------------------------
2019-10-09T20:04:56.6687401Z 
2019-10-09T20:04:56.6687445Z 
2019-10-09T20:04:56.6687691Z ---- [ui] ui/async-await/conditional-and-guaranteed-initialization.rs stdout ----
2019-10-09T20:04:56.6687726Z 
2019-10-09T20:04:56.6688121Z error: test compilation failed although it shouldn't!
2019-10-09T20:04:56.6688188Z status: exit code: 1
2019-10-09T20:04:56.6689246Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/conditional-and-guaranteed-initialization.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/conditional-and-guaranteed-initialization" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--crate-type" "lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/conditional-and-guaranteed-initialization/auxiliary" "-A" "unused"
2019-10-09T20:04:56.6689905Z ------------------------------------------
2019-10-09T20:04:56.6689936Z 
2019-10-09T20:04:56.6690429Z ------------------------------------------
2019-10-09T20:04:56.6690479Z stderr:
2019-10-09T20:04:56.6690479Z stderr:
2019-10-09T20:04:56.6690680Z ------------------------------------------
2019-10-09T20:04:56.6690906Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6690984Z error: aborting due to previous error
2019-10-09T20:04:56.6691022Z 
2019-10-09T20:04:56.6691274Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6691308Z 
2019-10-09T20:04:56.6691308Z 
2019-10-09T20:04:56.6691515Z ------------------------------------------
2019-10-09T20:04:56.6691546Z 
2019-10-09T20:04:56.6691589Z 
2019-10-09T20:04:56.6691832Z ---- [ui] ui/async-await/drop-order/drop-order-for-locals-when-cancelled.rs stdout ----
2019-10-09T20:04:56.6691867Z 
2019-10-09T20:04:56.6692074Z error: test compilation failed although it shouldn't!
2019-10-09T20:04:56.6692140Z status: exit code: 1
2019-10-09T20:04:56.6692935Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/drop-order/drop-order-for-locals-when-cancelled.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-order/drop-order-for-locals-when-cancelled/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-order/drop-order-for-locals-when-cancelled/auxiliary"
2019-10-09T20:04:56.6693260Z ------------------------------------------
2019-10-09T20:04:56.6693292Z 
2019-10-09T20:04:56.6693510Z ------------------------------------------
2019-10-09T20:04:56.6693553Z stderr:
2019-10-09T20:04:56.6693553Z stderr:
2019-10-09T20:04:56.6693752Z ------------------------------------------
2019-10-09T20:04:56.6693821Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6693894Z error: aborting due to previous error
2019-10-09T20:04:56.6693922Z 
2019-10-09T20:04:56.6694167Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6694199Z 
2019-10-09T20:04:56.6694199Z 
2019-10-09T20:04:56.6694800Z ------------------------------------------
2019-10-09T20:04:56.6694973Z 
2019-10-09T20:04:56.6695021Z 
2019-10-09T20:04:56.6695322Z ---- [ui] ui/async-await/drop-order/drop-order-for-temporary-in-tail-return-expr.rs stdout ----
2019-10-09T20:04:56.6695362Z 
2019-10-09T20:04:56.6695666Z error: test compilation failed although it shouldn't!
2019-10-09T20:04:56.6695747Z status: exit code: 1
2019-10-09T20:04:56.6696619Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/drop-order/drop-order-for-temporary-in-tail-return-expr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-order/drop-order-for-temporary-in-tail-return-expr/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-order/drop-order-for-temporary-in-tail-return-expr/auxiliary"
2019-10-09T20:04:56.6696967Z ------------------------------------------
2019-10-09T20:04:56.6697020Z 
2019-10-09T20:04:56.6697239Z ------------------------------------------
2019-10-09T20:04:56.6697285Z stderr:
2019-10-09T20:04:56.6697285Z stderr:
2019-10-09T20:04:56.6697494Z ------------------------------------------
2019-10-09T20:04:56.6697565Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6697642Z error: aborting due to previous error
2019-10-09T20:04:56.6697670Z 
2019-10-09T20:04:56.6698075Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6698109Z 
2019-10-09T20:04:56.6698109Z 
2019-10-09T20:04:56.6698307Z ------------------------------------------
2019-10-09T20:04:56.6698337Z 
2019-10-09T20:04:56.6698381Z 
2019-10-09T20:04:56.6698615Z ---- [ui] ui/async-await/drop-order/drop-order-when-cancelled.rs stdout ----
2019-10-09T20:04:56.6698650Z 
2019-10-09T20:04:56.6698868Z error: test compilation failed although it shouldn't!
2019-10-09T20:04:56.6698932Z status: exit code: 1
2019-10-09T20:04:56.6699697Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/drop-order/drop-order-when-cancelled.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-order/drop-order-when-cancelled/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-order/drop-order-when-cancelled/auxiliary"
2019-10-09T20:04:56.6700015Z ------------------------------------------
2019-10-09T20:04:56.6700048Z 
2019-10-09T20:04:56.6700271Z ------------------------------------------
2019-10-09T20:04:56.6700314Z stderr:
2019-10-09T20:04:56.6700314Z stderr:
2019-10-09T20:04:56.6700523Z ------------------------------------------
2019-10-09T20:04:56.6700592Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6700673Z error: aborting due to previous error
2019-10-09T20:04:56.6700700Z 
2019-10-09T20:04:56.6700945Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6700977Z 
2019-10-09T20:04:56.6700977Z 
2019-10-09T20:04:56.6701173Z ------------------------------------------
2019-10-09T20:04:56.6701204Z 
2019-10-09T20:04:56.6701247Z 
2019-10-09T20:04:56.6701457Z ---- [ui] ui/async-await/issue-60709.rs stdout ----
2019-10-09T20:04:56.6701490Z 
2019-10-09T20:04:56.6701698Z error: test compilation failed although it shouldn't!
2019-10-09T20:04:56.6701763Z status: exit code: 1
2019-10-09T20:04:56.6702888Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-60709.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-60709/a" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Copt-level=z" "-Cdebuginfo=2" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-60709/auxiliary"
2019-10-09T20:04:56.6703322Z ------------------------------------------
2019-10-09T20:04:56.6703355Z 
2019-10-09T20:04:56.6703581Z ------------------------------------------
2019-10-09T20:04:56.6703624Z stderr:
2019-10-09T20:04:56.6703624Z stderr:
2019-10-09T20:04:56.6703822Z ------------------------------------------
2019-10-09T20:04:56.6703891Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6703966Z error: aborting due to previous error
2019-10-09T20:04:56.6703994Z 
2019-10-09T20:04:56.6704238Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6704727Z 
2019-10-09T20:04:56.6704727Z 
2019-10-09T20:04:56.6705003Z ------------------------------------------
2019-10-09T20:04:56.6705036Z 
2019-10-09T20:04:56.6705062Z 
2019-10-09T20:04:56.6705318Z ---- [ui] ui/async-await/issue-61793.rs stdout ----
2019-10-09T20:04:56.6705351Z 
2019-10-09T20:04:56.6705568Z error: test compilation failed although it shouldn't!
2019-10-09T20:04:56.6705636Z status: exit code: 1
2019-10-09T20:04:56.6706375Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-61793.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-61793" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-61793/auxiliary" "-A" "unused"
2019-10-09T20:04:56.6706705Z ------------------------------------------
2019-10-09T20:04:56.6706738Z 
2019-10-09T20:04:56.6706964Z ------------------------------------------
2019-10-09T20:04:56.6707019Z stderr:
2019-10-09T20:04:56.6707019Z stderr:
2019-10-09T20:04:56.6707230Z ------------------------------------------
2019-10-09T20:04:56.6707282Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6707377Z error: aborting due to previous error
2019-10-09T20:04:56.6707407Z 
2019-10-09T20:04:56.6707658Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6707693Z 
2019-10-09T20:04:56.6707693Z 
2019-10-09T20:04:56.6708052Z ------------------------------------------
2019-10-09T20:04:56.6708082Z 
2019-10-09T20:04:56.6708107Z 
2019-10-09T20:04:56.6708403Z ---- [ui] ui/async-await/issue-61949-self-return-type.rs stdout ----
2019-10-09T20:04:56.6708450Z diff of stderr:
2019-10-09T20:04:56.6708478Z 
2019-10-09T20:04:56.6708522Z + error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6708651Z 1 error: `async fn` return type cannot contain a projection or `Self` that references lifetimes from a parent scope
2019-10-09T20:04:56.6708917Z 2   --> $DIR/issue-61949-self-return-type.rs:11:40
2019-10-09T20:04:56.6708986Z 3    |
2019-10-09T20:04:56.6709015Z 
2019-10-09T20:04:56.6709015Z 
2019-10-09T20:04:56.6709249Z 4 LL |     pub async fn new(_bar: &'a i32) -> Self {
2019-10-09T20:04:56.6709364Z 6 
2019-10-09T20:04:56.6709583Z - error: aborting due to previous error
2019-10-09T20:04:56.6709651Z + error: aborting due to 2 previous errors
2019-10-09T20:04:56.6709694Z 8 
2019-10-09T20:04:56.6709694Z 8 
2019-10-09T20:04:56.6709941Z + For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6709988Z 9 
2019-10-09T20:04:56.6710034Z 
2019-10-09T20:04:56.6710062Z 
2019-10-09T20:04:56.6710107Z The actual stderr differed from the expected stderr.
2019-10-09T20:04:56.6710593Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-61949-self-return-type/issue-61949-self-return-type.stderr
2019-10-09T20:04:56.6710957Z To update references, rerun the tests and pass the `--bless` flag
2019-10-09T20:04:56.6711290Z To only update this specific test, also pass `--test-args async-await/issue-61949-self-return-type.rs`
2019-10-09T20:04:56.6711394Z error: 1 errors occurred comparing output.
2019-10-09T20:04:56.6711440Z status: exit code: 1
2019-10-09T20:04:56.6711440Z status: exit code: 1
2019-10-09T20:04:56.6712217Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-61949-self-return-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-61949-self-return-type" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-61949-self-return-type/auxiliary" "-A" "unused"
2019-10-09T20:04:56.6712588Z ------------------------------------------
2019-10-09T20:04:56.6712624Z 
2019-10-09T20:04:56.6712843Z ------------------------------------------
2019-10-09T20:04:56.6712890Z stderr:
2019-10-09T20:04:56.6712890Z stderr:
2019-10-09T20:04:56.6713124Z ------------------------------------------
2019-10-09T20:04:56.6713178Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6713283Z error: `async fn` return type cannot contain a projection or `Self` that references lifetimes from a parent scope
2019-10-09T20:04:56.6713554Z   --> /checkout/src/test/ui/async-await/issue-61949-self-return-type.rs:11:40
2019-10-09T20:04:56.6713605Z    |
2019-10-09T20:04:56.6713605Z    |
2019-10-09T20:04:56.6713848Z LL |     pub async fn new(_bar: &'a i32) -> Self {
2019-10-09T20:04:56.6713940Z 
2019-10-09T20:04:56.6713983Z error: aborting due to 2 previous errors
2019-10-09T20:04:56.6714031Z 
2019-10-09T20:04:56.6714636Z For more information about this error, try `rustc --explain E0433`.
---
2019-10-09T20:04:56.6715337Z ---- [ui] ui/async-await/issue-63832-await-short-temporary-lifetime-1.rs stdout ----
2019-10-09T20:04:56.6715373Z 
2019-10-09T20:04:56.6715609Z error: test compilation failed although it shouldn't!
2019-10-09T20:04:56.6715660Z status: exit code: 1
2019-10-09T20:04:56.6716519Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-63832-await-short-temporary-lifetime-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-63832-await-short-temporary-lifetime-1" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-63832-await-short-temporary-lifetime-1/auxiliary" "-A" "unused"
2019-10-09T20:04:56.6716866Z ------------------------------------------
2019-10-09T20:04:56.6716919Z 
2019-10-09T20:04:56.6717131Z ------------------------------------------
2019-10-09T20:04:56.6717177Z stderr:
2019-10-09T20:04:56.6717177Z stderr:
2019-10-09T20:04:56.6717400Z ------------------------------------------
2019-10-09T20:04:56.6717454Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6717531Z error: aborting due to previous error
2019-10-09T20:04:56.6717578Z 
2019-10-09T20:04:56.6718188Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6718222Z 
2019-10-09T20:04:56.6718222Z 
2019-10-09T20:04:56.6718423Z ------------------------------------------
2019-10-09T20:04:56.6718474Z 
2019-10-09T20:04:56.6718588Z 
2019-10-09T20:04:56.6718846Z ---- [ui] ui/async-await/issue-62658.rs stdout ----
2019-10-09T20:04:56.6718877Z 
2019-10-09T20:04:56.6719105Z error: test compilation failed although it shouldn't!
2019-10-09T20:04:56.6719153Z status: exit code: 1
2019-10-09T20:04:56.6720019Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-62658.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-62658" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-62658/auxiliary" "-A" "unused"
2019-10-09T20:04:56.6720332Z ------------------------------------------
2019-10-09T20:04:56.6720381Z 
2019-10-09T20:04:56.6720583Z ------------------------------------------
2019-10-09T20:04:56.6720625Z stderr:
2019-10-09T20:04:56.6720625Z stderr:
2019-10-09T20:04:56.6720819Z ------------------------------------------
2019-10-09T20:04:56.6720886Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6720957Z error: aborting due to previous error
2019-10-09T20:04:56.6720983Z 
2019-10-09T20:04:56.6721220Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6721252Z 
2019-10-09T20:04:56.6721252Z 
2019-10-09T20:04:56.6721443Z ------------------------------------------
2019-10-09T20:04:56.6721472Z 
2019-10-09T20:04:56.6721513Z 
2019-10-09T20:04:56.6721743Z ---- [ui] ui/async-await/issue-63832-await-short-temporary-lifetime.rs stdout ----
2019-10-09T20:04:56.6721776Z 
2019-10-09T20:04:56.6721989Z error: test compilation failed although it shouldn't!
2019-10-09T20:04:56.6722051Z status: exit code: 1
2019-10-09T20:04:56.6722840Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-63832-await-short-temporary-lifetime.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-63832-await-short-temporary-lifetime" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-63832-await-short-temporary-lifetime/auxiliary" "-A" "unused"
2019-10-09T20:04:56.6723149Z ------------------------------------------
2019-10-09T20:04:56.6723200Z 
2019-10-09T20:04:56.6723393Z ------------------------------------------
2019-10-09T20:04:56.6723443Z stderr:
2019-10-09T20:04:56.6723443Z stderr:
2019-10-09T20:04:56.6723637Z ------------------------------------------
2019-10-09T20:04:56.6723713Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6723784Z error: aborting due to previous error
2019-10-09T20:04:56.6723810Z 
2019-10-09T20:04:56.6724047Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6724078Z 
2019-10-09T20:04:56.6724078Z 
2019-10-09T20:04:56.6724420Z ------------------------------------------
2019-10-09T20:04:56.6724461Z 
2019-10-09T20:04:56.6724507Z 
2019-10-09T20:04:56.6724959Z ---- [ui] ui/async-await/issue-64130-non-send-future-diags.rs stdout ----
2019-10-09T20:04:56.6725009Z diff of stderr:
2019-10-09T20:04:56.6725037Z 
2019-10-09T20:04:56.6725308Z - error[E0277]: `std::sync::MutexGuard<'_, u32>` cannot be sent between threads safely
2019-10-09T20:04:56.6725866Z -    |
2019-10-09T20:04:56.6725866Z -    |
2019-10-09T20:04:56.6726091Z - LL | fn is_send<T: Send>(t: T) {
2019-10-09T20:04:56.6726321Z -    |    -------    ---- required by this bound in `is_send`
2019-10-09T20:04:56.6726499Z - ...
2019-10-09T20:04:56.6726784Z - LL |     is_send(foo());
2019-10-09T20:04:56.6727084Z -    |     ^^^^^^^ `std::sync::MutexGuard<'_, u32>` cannot be sent between threads safely
2019-10-09T20:04:56.6727265Z -    |
2019-10-09T20:04:56.6727581Z -    = help: within `impl std::future::Future`, the trait `std::marker::Send` is not implemented for `std::sync::MutexGuard<'_, u32>`
2019-10-09T20:04:56.6728054Z - note: future does not implement `std::marker::Send` as this value is used across an await
2019-10-09T20:04:56.6728456Z -    |
2019-10-09T20:04:56.6728456Z -    |
2019-10-09T20:04:56.6728644Z - LL |     let g = x.lock().unwrap();
2019-10-09T20:04:56.6728857Z -    |         - has type `std::sync::MutexGuard<'_, u32>`
2019-10-09T20:04:56.6729055Z - LL |     baz().await;
2019-10-09T20:04:56.6729455Z - LL | }
2019-10-09T20:04:56.6729638Z -    | - `g` is later dropped here
2019-10-09T20:04:56.6729638Z -    | - `g` is later dropped here
2019-10-09T20:04:56.6729715Z + error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6729798Z 21 error: aborting due to previous error
2019-10-09T20:04:56.6729853Z 22 
2019-10-09T20:04:56.6729879Z 
2019-10-09T20:04:56.6730268Z - For more information about this error, try `rustc --explain E0277`.
2019-10-09T20:04:56.6730268Z - For more information about this error, try `rustc --explain E0277`.
2019-10-09T20:04:56.6730517Z + For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6730562Z 24 
2019-10-09T20:04:56.6730587Z 
2019-10-09T20:04:56.6730611Z 
2019-10-09T20:04:56.6730654Z The actual stderr differed from the expected stderr.
2019-10-09T20:04:56.6730989Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-64130-non-send-future-diags/issue-64130-non-send-future-diags.stderr
2019-10-09T20:04:56.6731233Z To update references, rerun the tests and pass the `--bless` flag
2019-10-09T20:04:56.6731525Z To only update this specific test, also pass `--test-args async-await/issue-64130-non-send-future-diags.rs`
2019-10-09T20:04:56.6731606Z error: 1 errors occurred comparing output.
2019-10-09T20:04:56.6731649Z status: exit code: 1
2019-10-09T20:04:56.6731649Z status: exit code: 1
2019-10-09T20:04:56.6732437Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-64130-non-send-future-diags.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-64130-non-send-future-diags" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-64130-non-send-future-diags/auxiliary" "-A" "unused"
2019-10-09T20:04:56.6732759Z ------------------------------------------
2019-10-09T20:04:56.6732792Z 
2019-10-09T20:04:56.6733002Z ------------------------------------------
2019-10-09T20:04:56.6733064Z stderr:
2019-10-09T20:04:56.6733064Z stderr:
2019-10-09T20:04:56.6733267Z ------------------------------------------
2019-10-09T20:04:56.6733318Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6733412Z error: aborting due to previous error
2019-10-09T20:04:56.6733440Z 
2019-10-09T20:04:56.6733663Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6733714Z 
2019-10-09T20:04:56.6733714Z 
2019-10-09T20:04:56.6733911Z ------------------------------------------
2019-10-09T20:04:56.6733941Z 
2019-10-09T20:04:56.6733966Z 
2019-10-09T20:04:56.6734175Z ---- [ui] ui/async-await/issue-64391.rs stdout ----
2019-10-09T20:04:56.6734228Z 
2019-10-09T20:04:56.6735170Z error: test compilation failed although it shouldn't!
2019-10-09T20:04:56.6735382Z status: exit code: 1
2019-10-09T20:04:56.6736304Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-64391.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-64391" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-64391/auxiliary" "-A" "unused"
2019-10-09T20:04:56.6736667Z ------------------------------------------
2019-10-09T20:04:56.6736703Z 
2019-10-09T20:04:56.6736915Z ------------------------------------------
2019-10-09T20:04:56.6736979Z stderr:
2019-10-09T20:04:56.6736979Z stderr:
2019-10-09T20:04:56.6737190Z ------------------------------------------
2019-10-09T20:04:56.6737252Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6737348Z error: aborting due to previous error
2019-10-09T20:04:56.6737384Z 
2019-10-09T20:04:56.6737804Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6737843Z 
---
2019-10-09T20:04:56.6738723Z 6    |                |
2019-10-09T20:04:56.6738766Z 7    |                this is not `async`
2019-10-09T20:04:56.6738824Z 8 
2019-10-09T20:04:56.6739023Z - error: aborting due to previous error
2019-10-09T20:04:56.6739074Z + error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6739190Z + error: aborting due to 2 previous errors
2019-10-09T20:04:56.6739399Z + 
2019-10-09T20:04:56.6739638Z + For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6739690Z 11 
2019-10-09T20:04:56.6739690Z 11 
2019-10-09T20:04:56.6739715Z 
2019-10-09T20:04:56.6739739Z 
2019-10-09T20:04:56.6739798Z The actual stderr differed from the expected stderr.
2019-10-09T20:04:56.6740078Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-51719/issue-51719.stderr
2019-10-09T20:04:56.6740303Z To update references, rerun the tests and pass the `--bless` flag
2019-10-09T20:04:56.6740569Z To only update this specific test, also pass `--test-args async-await/issues/issue-51719.rs`
2019-10-09T20:04:56.6740642Z error: 1 errors occurred comparing output.
2019-10-09T20:04:56.6740682Z status: exit code: 1
2019-10-09T20:04:56.6740682Z status: exit code: 1
2019-10-09T20:04:56.6741412Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-51719.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-51719" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-51719/auxiliary" "-A" "unused"
2019-10-09T20:04:56.6741722Z ------------------------------------------
2019-10-09T20:04:56.6741753Z 
2019-10-09T20:04:56.6741948Z ------------------------------------------
2019-10-09T20:04:56.6742008Z stderr:
2019-10-09T20:04:56.6742008Z stderr:
2019-10-09T20:04:56.6742440Z ------------------------------------------
2019-10-09T20:04:56.6742497Z error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6742752Z   --> /checkout/src/test/ui/async-await/issues/issue-51719.rs:8:19
2019-10-09T20:04:56.6742927Z    |
2019-10-09T20:04:56.6742968Z LL |     let _gen = || foo().await;
2019-10-09T20:04:56.6743324Z    |                -- ^^^^^^^^^^^ only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6743425Z    |                this is not `async`
2019-10-09T20:04:56.6743453Z 
2019-10-09T20:04:56.6743453Z 
2019-10-09T20:04:56.6743515Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6743583Z error: aborting due to 2 previous errors
2019-10-09T20:04:56.6743610Z 
2019-10-09T20:04:56.6743874Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6743906Z 
2019-10-09T20:04:56.6743906Z 
2019-10-09T20:04:56.6744098Z ------------------------------------------
2019-10-09T20:04:56.6744128Z 
2019-10-09T20:04:56.6744170Z 
2019-10-09T20:04:56.6744896Z ---- [ui] ui/async-await/issues/issue-51751.rs stdout ----
2019-10-09T20:04:56.6744962Z diff of stderr:
2019-10-09T20:04:56.6745002Z 
2019-10-09T20:04:56.6745071Z 7 LL |     let finished = result.await;
2019-10-09T20:04:56.6745123Z 8    |                    ^^^^^^^^^^^^ only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6745449Z - error: aborting due to previous error
2019-10-09T20:04:56.6745449Z - error: aborting due to previous error
2019-10-09T20:04:56.6745503Z + error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6745609Z + error: aborting due to 2 previous errors
2019-10-09T20:04:56.6745651Z + 
2019-10-09T20:04:56.6745892Z + For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6745939Z 12 
2019-10-09T20:04:56.6745939Z 12 
2019-10-09T20:04:56.6745983Z 
2019-10-09T20:04:56.6746009Z 
2019-10-09T20:04:56.6746053Z The actual stderr differed from the expected stderr.
2019-10-09T20:04:56.6746350Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-51751/issue-51751.stderr
2019-10-09T20:04:56.6746612Z To update references, rerun the tests and pass the `--bless` flag
2019-10-09T20:04:56.6746913Z To only update this specific test, also pass `--test-args async-await/issues/issue-51751.rs`
2019-10-09T20:04:56.6747027Z error: 1 errors occurred comparing output.
2019-10-09T20:04:56.6747075Z status: exit code: 1
2019-10-09T20:04:56.6747075Z status: exit code: 1
2019-10-09T20:04:56.6747856Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-51751.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-51751" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-51751/auxiliary" "-A" "unused"
2019-10-09T20:04:56.6748316Z ------------------------------------------
2019-10-09T20:04:56.6748374Z 
2019-10-09T20:04:56.6748572Z ------------------------------------------
2019-10-09T20:04:56.6748614Z stderr:
2019-10-09T20:04:56.6748614Z stderr:
2019-10-09T20:04:56.6748831Z ------------------------------------------
2019-10-09T20:04:56.6748881Z error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6749109Z   --> /checkout/src/test/ui/async-await/issues/issue-51751.rs:9:20
2019-10-09T20:04:56.6749212Z LL | fn main() {
2019-10-09T20:04:56.6749400Z    |    ---- this is not `async`
2019-10-09T20:04:56.6749462Z LL |     let result = inc(10000);
2019-10-09T20:04:56.6749462Z LL |     let result = inc(10000);
2019-10-09T20:04:56.6749505Z LL |     let finished = result.await;
2019-10-09T20:04:56.6749552Z    |                    ^^^^^^^^^^^^ only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6749583Z 
2019-10-09T20:04:56.6749646Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6749714Z error: aborting due to 2 previous errors
2019-10-09T20:04:56.6749848Z 
2019-10-09T20:04:56.6750118Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6750151Z 
2019-10-09T20:04:56.6750151Z 
2019-10-09T20:04:56.6750414Z ------------------------------------------
2019-10-09T20:04:56.6750453Z 
2019-10-09T20:04:56.6750496Z 
2019-10-09T20:04:56.6750732Z ---- [ui] ui/async-await/issues/issue-53249.rs stdout ----
2019-10-09T20:04:56.6750764Z 
2019-10-09T20:04:56.6750965Z error: test compilation failed although it shouldn't!
2019-10-09T20:04:56.6751029Z status: exit code: 1
2019-10-09T20:04:56.6751734Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-53249.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-53249" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-53249/auxiliary" "-A" "unused"
2019-10-09T20:04:56.6752053Z ------------------------------------------
2019-10-09T20:04:56.6752084Z 
2019-10-09T20:04:56.6752299Z ------------------------------------------
2019-10-09T20:04:56.6752341Z stderr:
2019-10-09T20:04:56.6752341Z stderr:
2019-10-09T20:04:56.6752532Z ------------------------------------------
2019-10-09T20:04:56.6752598Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6752670Z error: aborting due to previous error
2019-10-09T20:04:56.6752697Z 
2019-10-09T20:04:56.6752933Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6752965Z 
2019-10-09T20:04:56.6752965Z 
2019-10-09T20:04:56.6753156Z ------------------------------------------
2019-10-09T20:04:56.6753186Z 
2019-10-09T20:04:56.6753209Z 
2019-10-09T20:04:56.6753436Z ---- [ui] ui/async-await/issues/issue-55324.rs stdout ----
2019-10-09T20:04:56.6753475Z 
2019-10-09T20:04:56.6753678Z error: test compilation failed although it shouldn't!
2019-10-09T20:04:56.6753742Z status: exit code: 1
2019-10-09T20:04:56.6754792Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-55324.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-55324" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-55324/auxiliary" "-A" "unused"
2019-10-09T20:04:56.6755169Z ------------------------------------------
2019-10-09T20:04:56.6755203Z 
2019-10-09T20:04:56.6755431Z ------------------------------------------
2019-10-09T20:04:56.6755487Z stderr:
2019-10-09T20:04:56.6755487Z stderr:
2019-10-09T20:04:56.6755695Z ------------------------------------------
2019-10-09T20:04:56.6755756Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6755853Z error: aborting due to previous error
2019-10-09T20:04:56.6755882Z 
2019-10-09T20:04:56.6756120Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6756173Z 
2019-10-09T20:04:56.6756173Z 
2019-10-09T20:04:56.6756379Z ------------------------------------------
2019-10-09T20:04:56.6756411Z 
2019-10-09T20:04:56.6756436Z 
2019-10-09T20:04:56.6756678Z ---- [ui] ui/async-await/issues/issue-55809.rs stdout ----
2019-10-09T20:04:56.6756710Z 
2019-10-09T20:04:56.6756928Z error: test compilation failed although it shouldn't!
2019-10-09T20:04:56.6756977Z status: exit code: 1
2019-10-09T20:04:56.6757834Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-55809.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-55809/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-55809/auxiliary"
2019-10-09T20:04:56.6758395Z ------------------------------------------
2019-10-09T20:04:56.6758428Z 
2019-10-09T20:04:56.6758624Z ------------------------------------------
2019-10-09T20:04:56.6758685Z stderr:
2019-10-09T20:04:56.6758685Z stderr:
2019-10-09T20:04:56.6758877Z ------------------------------------------
2019-10-09T20:04:56.6758926Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6759015Z error: aborting due to previous error
2019-10-09T20:04:56.6759050Z 
2019-10-09T20:04:56.6759274Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6759325Z 
2019-10-09T20:04:56.6759325Z 
2019-10-09T20:04:56.6759516Z ------------------------------------------
2019-10-09T20:04:56.6759552Z 
2019-10-09T20:04:56.6759577Z 
2019-10-09T20:04:56.6759806Z ---- [ui] ui/async-await/issues/issue-59972.rs stdout ----
2019-10-09T20:04:56.6759838Z 
2019-10-09T20:04:56.6760039Z error: test compilation failed although it shouldn't!
2019-10-09T20:04:56.6760084Z status: exit code: 1
2019-10-09T20:04:56.6760930Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-59972.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-59972/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-Aunused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-59972/auxiliary"
2019-10-09T20:04:56.6761230Z ------------------------------------------
2019-10-09T20:04:56.6761267Z 
2019-10-09T20:04:56.6761456Z ------------------------------------------
2019-10-09T20:04:56.6761514Z stderr:
2019-10-09T20:04:56.6761514Z stderr:
2019-10-09T20:04:56.6761701Z ------------------------------------------
2019-10-09T20:04:56.6761749Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6761836Z error: aborting due to previous error
2019-10-09T20:04:56.6761862Z 
2019-10-09T20:04:56.6762074Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6762104Z 
2019-10-09T20:04:56.6762104Z 
2019-10-09T20:04:56.6762304Z ------------------------------------------
2019-10-09T20:04:56.6762332Z 
2019-10-09T20:04:56.6762355Z 
2019-10-09T20:04:56.6762571Z ---- [ui] ui/async-await/issues/issue-60655-latebound-regions.rs stdout ----
2019-10-09T20:04:56.6762629Z 
2019-10-09T20:04:56.6762828Z error: test compilation failed although it shouldn't!
2019-10-09T20:04:56.6762872Z status: exit code: 1
2019-10-09T20:04:56.6763615Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-60655-latebound-regions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-60655-latebound-regions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-60655-latebound-regions/auxiliary" "-A" "unused"
2019-10-09T20:04:56.6763911Z ------------------------------------------
2019-10-09T20:04:56.6763941Z 
2019-10-09T20:04:56.6764237Z ------------------------------------------
2019-10-09T20:04:56.6764444Z stderr:
2019-10-09T20:04:56.6764444Z stderr:
2019-10-09T20:04:56.6764880Z ------------------------------------------
2019-10-09T20:04:56.6765033Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6765147Z error: aborting due to previous error
2019-10-09T20:04:56.6765175Z 
2019-10-09T20:04:56.6765448Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6765483Z 
2019-10-09T20:04:56.6765483Z 
2019-10-09T20:04:56.6765709Z ------------------------------------------
2019-10-09T20:04:56.6765740Z 
2019-10-09T20:04:56.6765766Z 
2019-10-09T20:04:56.6765988Z ---- [ui] ui/async-await/issues/issue-61986.rs stdout ----
2019-10-09T20:04:56.6766037Z 
2019-10-09T20:04:56.6766255Z error: test compilation failed although it shouldn't!
2019-10-09T20:04:56.6766337Z status: exit code: 1
2019-10-09T20:04:56.6767126Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-61986.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-61986" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-61986/auxiliary" "-A" "unused"
2019-10-09T20:04:56.6767459Z ------------------------------------------
2019-10-09T20:04:56.6767492Z 
2019-10-09T20:04:56.6767700Z ------------------------------------------
2019-10-09T20:04:56.6767745Z stderr:
2019-10-09T20:04:56.6767745Z stderr:
2019-10-09T20:04:56.6768115Z ------------------------------------------
2019-10-09T20:04:56.6768162Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6768249Z error: aborting due to previous error
2019-10-09T20:04:56.6768283Z 
2019-10-09T20:04:56.6768497Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6768528Z 
2019-10-09T20:04:56.6768528Z 
2019-10-09T20:04:56.6768731Z ------------------------------------------
2019-10-09T20:04:56.6768766Z 
2019-10-09T20:04:56.6768789Z 
2019-10-09T20:04:56.6768994Z ---- [ui] ui/async-await/issues/issue-62009-1.rs stdout ----
2019-10-09T20:04:56.6769056Z diff of stderr:
2019-10-09T20:04:56.6769081Z 
2019-10-09T20:04:56.6769118Z 6 LL |     async { let (); }.await;
2019-10-09T20:04:56.6769163Z 7    |     ^^^^^^^^^^^^^^^^^^^^^^^ only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6769222Z 8 
2019-10-09T20:04:56.6769264Z + error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6769303Z + 
2019-10-09T20:04:56.6769362Z 9 error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6769559Z 10   --> $DIR/issue-62009-1.rs:10:5
2019-10-09T20:04:56.6769631Z 
2019-10-09T20:04:56.6769631Z 
2019-10-09T20:04:56.6769687Z 27 LL |     (|_| 2333).await;
2019-10-09T20:04:56.6769732Z 28    |     ^^^^^^^^^^^^^^^^ only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6769772Z 29 
2019-10-09T20:04:56.6770062Z - error[E0277]: the trait bound `[closure@$DIR/issue-62009-1.rs:14:5: 14:15]: std::future::Future` is not satisfied
2019-10-09T20:04:56.6770465Z -   --> $DIR/issue-62009-1.rs:14:5
2019-10-09T20:04:56.6770634Z -    |
2019-10-09T20:04:56.6770838Z - LL |     (|_| 2333).await;
2019-10-09T20:04:56.6771117Z -    |     ^^^^^^^^^^^^^^^^ the trait `std::future::Future` is not implemented for `[closure@$DIR/issue-62009-1.rs:14:5: 14:15]`
2019-10-09T20:04:56.6771293Z -    | 
2019-10-09T20:04:56.6771504Z -   ::: $SRC_DIR/libstd/future.rs:LL:COL
2019-10-09T20:04:56.6771843Z - LL |     F: Future
2019-10-09T20:04:56.6771843Z - LL |     F: Future
2019-10-09T20:04:56.6772094Z -    |        ------ required by this bound in `std::future::poll_with_tls_context`
2019-10-09T20:04:56.6772418Z 41 error: aborting due to 4 previous errors
2019-10-09T20:04:56.6772458Z 42 
2019-10-09T20:04:56.6772733Z - For more information about this error, try `rustc --explain E0277`.
2019-10-09T20:04:56.6773085Z + For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6773085Z + For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6773143Z 44 
2019-10-09T20:04:56.6773188Z 
2019-10-09T20:04:56.6773212Z 
2019-10-09T20:04:56.6773254Z The actual stderr differed from the expected stderr.
2019-10-09T20:04:56.6773567Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62009-1/issue-62009-1.stderr
2019-10-09T20:04:56.6773815Z To update references, rerun the tests and pass the `--bless` flag
2019-10-09T20:04:56.6774113Z To only update this specific test, also pass `--test-args async-await/issues/issue-62009-1.rs`
2019-10-09T20:04:56.6774301Z error: 1 errors occurred comparing output.
2019-10-09T20:04:56.6774343Z status: exit code: 1
2019-10-09T20:04:56.6774343Z status: exit code: 1
2019-10-09T20:04:56.6776182Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-62009-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62009-1" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62009-1/auxiliary" "-A" "unused"
2019-10-09T20:04:56.6776738Z ------------------------------------------
2019-10-09T20:04:56.6776789Z 
2019-10-09T20:04:56.6777002Z ------------------------------------------
2019-10-09T20:04:56.6777047Z stderr:
2019-10-09T20:04:56.6777047Z stderr:
2019-10-09T20:04:56.6777254Z ------------------------------------------
2019-10-09T20:04:56.6777325Z error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6777580Z   --> /checkout/src/test/ui/async-await/issues/issue-62009-1.rs:8:5
2019-10-09T20:04:56.6777690Z LL | fn main() {
2019-10-09T20:04:56.6777899Z    |    ---- this is not `async`
2019-10-09T20:04:56.6777899Z    |    ---- this is not `async`
2019-10-09T20:04:56.6777947Z LL |     async { let (); }.await;
2019-10-09T20:04:56.6778016Z    |     ^^^^^^^^^^^^^^^^^^^^^^^ only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6778049Z 
2019-10-09T20:04:56.6778096Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6778126Z 
2019-10-09T20:04:56.6778192Z error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6778596Z   --> /checkout/src/test/ui/async-await/issues/issue-62009-1.rs:10:5
2019-10-09T20:04:56.6778696Z LL |   fn main() {
2019-10-09T20:04:56.6778886Z    |      ---- this is not `async`
2019-10-09T20:04:56.6778928Z ...
2019-10-09T20:04:56.6778984Z LL | /     async {
2019-10-09T20:04:56.6778984Z LL | /     async {
2019-10-09T20:04:56.6779038Z LL | |     //~^ ERROR `await` is only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6779085Z LL | |         let task1 = print_dur().await;
2019-10-09T20:04:56.6779126Z LL | |     }.await;
2019-10-09T20:04:56.6779196Z    | |___________^ only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6779226Z 
2019-10-09T20:04:56.6779268Z error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6779518Z   --> /checkout/src/test/ui/async-await/issues/issue-62009-1.rs:14:5
2019-10-09T20:04:56.6779600Z LL | fn main() {
2019-10-09T20:04:56.6779805Z    |    ---- this is not `async`
2019-10-09T20:04:56.6779847Z ...
2019-10-09T20:04:56.6779847Z ...
2019-10-09T20:04:56.6779885Z LL |     (|_| 2333).await;
2019-10-09T20:04:56.6779930Z    |     ^^^^^^^^^^^^^^^^ only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6780016Z error: aborting due to 4 previous errors
2019-10-09T20:04:56.6780043Z 
2019-10-09T20:04:56.6780398Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6780450Z 
2019-10-09T20:04:56.6780450Z 
2019-10-09T20:04:56.6780645Z ------------------------------------------
2019-10-09T20:04:56.6780675Z 
2019-10-09T20:04:56.6780769Z 
2019-10-09T20:04:56.6781034Z ---- [ui] ui/async-await/issues/issue-62009-2.rs stdout ----
2019-10-09T20:04:56.6781082Z diff of stderr:
2019-10-09T20:04:56.6781107Z 
2019-10-09T20:04:56.6781146Z 6 LL |     (async || 2333)().await;
2019-10-09T20:04:56.6781210Z 7    |     ^^^^^^^^^^^^^^^^^^^^^^^ only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6781445Z - error: aborting due to previous error
2019-10-09T20:04:56.6781445Z - error: aborting due to previous error
2019-10-09T20:04:56.6781513Z + error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6781593Z + error: aborting due to 2 previous errors
2019-10-09T20:04:56.6781631Z + 
2019-10-09T20:04:56.6781872Z + For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6781924Z 11 
2019-10-09T20:04:56.6781924Z 11 
2019-10-09T20:04:56.6781949Z 
2019-10-09T20:04:56.6781973Z 
2019-10-09T20:04:56.6782033Z The actual stderr differed from the expected stderr.
2019-10-09T20:04:56.6782562Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62009-2/issue-62009-2.stderr
2019-10-09T20:04:56.6782807Z To update references, rerun the tests and pass the `--bless` flag
2019-10-09T20:04:56.6783105Z To only update this specific test, also pass `--test-args async-await/issues/issue-62009-2.rs`
2019-10-09T20:04:56.6783185Z error: 1 errors occurred comparing output.
2019-10-09T20:04:56.6783247Z status: exit code: 1
2019-10-09T20:04:56.6783247Z status: exit code: 1
2019-10-09T20:04:56.6783989Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-62009-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62009-2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62009-2/auxiliary" "-A" "unused"
2019-10-09T20:04:56.6784498Z ------------------------------------------
2019-10-09T20:04:56.6784785Z 
2019-10-09T20:04:56.6785051Z ------------------------------------------
2019-10-09T20:04:56.6785097Z stderr:
2019-10-09T20:04:56.6785097Z stderr:
2019-10-09T20:04:56.6785306Z ------------------------------------------
2019-10-09T20:04:56.6785377Z error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6785621Z   --> /checkout/src/test/ui/async-await/issues/issue-62009-2.rs:8:5
2019-10-09T20:04:56.6785732Z LL | fn main() {
2019-10-09T20:04:56.6785934Z    |    ---- this is not `async`
2019-10-09T20:04:56.6785934Z    |    ---- this is not `async`
2019-10-09T20:04:56.6785982Z LL |     (async || 2333)().await;
2019-10-09T20:04:56.6786043Z    |     ^^^^^^^^^^^^^^^^^^^^^^^ only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6786096Z 
2019-10-09T20:04:56.6786149Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6786223Z error: aborting due to 2 previous errors
2019-10-09T20:04:56.6786271Z 
2019-10-09T20:04:56.6786511Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6786544Z 
2019-10-09T20:04:56.6786544Z 
2019-10-09T20:04:56.6786751Z ------------------------------------------
2019-10-09T20:04:56.6786801Z 
2019-10-09T20:04:56.6786827Z 
2019-10-09T20:04:56.6787053Z ---- [ui] ui/async-await/issues/issue-64391-2.rs stdout ----
2019-10-09T20:04:56.6787087Z 
2019-10-09T20:04:56.6787323Z error: test compilation failed although it shouldn't!
2019-10-09T20:04:56.6787372Z status: exit code: 1
2019-10-09T20:04:56.6788422Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-64391-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-64391-2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-64391-2/auxiliary" "-A" "unused"
2019-10-09T20:04:56.6788837Z ------------------------------------------
2019-10-09T20:04:56.6788892Z 
2019-10-09T20:04:56.6789091Z ------------------------------------------
2019-10-09T20:04:56.6789133Z stderr:
2019-10-09T20:04:56.6789133Z stderr:
2019-10-09T20:04:56.6789343Z ------------------------------------------
2019-10-09T20:04:56.6789393Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6789474Z error: aborting due to previous error
2019-10-09T20:04:56.6789519Z 
2019-10-09T20:04:56.6789740Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6789772Z 
2019-10-09T20:04:56.6789772Z 
2019-10-09T20:04:56.6789974Z ------------------------------------------
2019-10-09T20:04:56.6790022Z 
2019-10-09T20:04:56.6790045Z 
2019-10-09T20:04:56.6790258Z ---- [ui] ui/async-await/issues/issue-64477.rs stdout ----
2019-10-09T20:04:56.6790289Z 
2019-10-09T20:04:56.6790509Z error: test compilation failed although it shouldn't!
2019-10-09T20:04:56.6790553Z status: exit code: 1
2019-10-09T20:04:56.6791275Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-64477.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-64477" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-64477/auxiliary" "-A" "unused"
2019-10-09T20:04:56.6791594Z ------------------------------------------
2019-10-09T20:04:56.6791644Z 
2019-10-09T20:04:56.6791839Z ------------------------------------------
2019-10-09T20:04:56.6791881Z stderr:
2019-10-09T20:04:56.6791881Z stderr:
2019-10-09T20:04:56.6792073Z ------------------------------------------
2019-10-09T20:04:56.6792142Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6792213Z error: aborting due to previous error
2019-10-09T20:04:56.6792258Z 
2019-10-09T20:04:56.6792481Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6792514Z 
2019-10-09T20:04:56.6792514Z 
2019-10-09T20:04:56.6792705Z ------------------------------------------
2019-10-09T20:04:56.6792751Z 
2019-10-09T20:04:56.6792784Z 
2019-10-09T20:04:56.6792994Z ---- [ui] ui/async-await/issues/issue-64433.rs stdout ----
2019-10-09T20:04:56.6793026Z 
2019-10-09T20:04:56.6793227Z error: test compilation failed although it shouldn't!
2019-10-09T20:04:56.6793298Z status: exit code: 1
2019-10-09T20:04:56.6794020Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-64433.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-64433" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-64433/auxiliary" "-A" "unused"
2019-10-09T20:04:56.6794670Z ------------------------------------------
2019-10-09T20:04:56.6794818Z 
2019-10-09T20:04:56.6795108Z ------------------------------------------
2019-10-09T20:04:56.6795154Z stderr:
2019-10-09T20:04:56.6795154Z stderr:
2019-10-09T20:04:56.6795362Z ------------------------------------------
2019-10-09T20:04:56.6795509Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6795597Z error: aborting due to previous error
2019-10-09T20:04:56.6795625Z 
2019-10-09T20:04:56.6795914Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6795949Z 
2019-10-09T20:04:56.6795949Z 
2019-10-09T20:04:56.6796157Z ------------------------------------------
2019-10-09T20:04:56.6796188Z 
2019-10-09T20:04:56.6796233Z 
2019-10-09T20:04:56.6796467Z ---- [ui] ui/async-await/issues/non-async-enclosing-span.rs stdout ----
2019-10-09T20:04:56.6796517Z diff of stderr:
2019-10-09T20:04:56.6796545Z 
2019-10-09T20:04:56.6796606Z 7 LL |     let y = do_the_thing().await;
2019-10-09T20:04:56.6796658Z 8    |             ^^^^^^^^^^^^^^^^^^^^ only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6796940Z - error: aborting due to previous error
2019-10-09T20:04:56.6796940Z - error: aborting due to previous error
2019-10-09T20:04:56.6797002Z + error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6797108Z + error: aborting due to 2 previous errors
2019-10-09T20:04:56.6797150Z + 
2019-10-09T20:04:56.6797387Z + For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6797435Z 12 
2019-10-09T20:04:56.6797435Z 12 
2019-10-09T20:04:56.6797481Z 
2019-10-09T20:04:56.6797506Z 
2019-10-09T20:04:56.6797550Z The actual stderr differed from the expected stderr.
2019-10-09T20:04:56.6798027Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/non-async-enclosing-span/non-async-enclosing-span.stderr
2019-10-09T20:04:56.6798274Z To update references, rerun the tests and pass the `--bless` flag
2019-10-09T20:04:56.6798532Z To only update this specific test, also pass `--test-args async-await/issues/non-async-enclosing-span.rs`
2019-10-09T20:04:56.6798635Z error: 1 errors occurred comparing output.
2019-10-09T20:04:56.6798675Z status: exit code: 1
2019-10-09T20:04:56.6798675Z status: exit code: 1
2019-10-09T20:04:56.6799434Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/non-async-enclosing-span.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/non-async-enclosing-span" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/non-async-enclosing-span/auxiliary" "-A" "unused"
2019-10-09T20:04:56.6799737Z ------------------------------------------
2019-10-09T20:04:56.6799768Z 
2019-10-09T20:04:56.6799973Z ------------------------------------------
2019-10-09T20:04:56.6800015Z stderr:
2019-10-09T20:04:56.6800015Z stderr:
2019-10-09T20:04:56.6800395Z ------------------------------------------
2019-10-09T20:04:56.6800450Z error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6800742Z    |
2019-10-09T20:04:56.6800742Z    |
2019-10-09T20:04:56.6800783Z LL | fn main() {  //~ NOTE this is not `async`
2019-10-09T20:04:56.6800967Z    |    ---- this is not `async`
2019-10-09T20:04:56.6801028Z LL |     let x = move || {};
2019-10-09T20:04:56.6801077Z LL |     let y = do_the_thing().await; //~ ERROR `await` is only allowed inside `async` functions
2019-10-09T20:04:56.6801130Z    |             ^^^^^^^^^^^^^^^^^^^^ only allowed inside `async` functions and blocks
2019-10-09T20:04:56.6801179Z 
2019-10-09T20:04:56.6801220Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6801370Z error: aborting due to 2 previous errors
2019-10-09T20:04:56.6801415Z 
2019-10-09T20:04:56.6801657Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6801688Z 
2019-10-09T20:04:56.6801688Z 
2019-10-09T20:04:56.6801953Z ------------------------------------------
2019-10-09T20:04:56.6802012Z 
2019-10-09T20:04:56.6802035Z 
2019-10-09T20:04:56.6802270Z ---- [ui] ui/async-await/issues/issue-64964.rs stdout ----
2019-10-09T20:04:56.6802300Z 
2019-10-09T20:04:56.6802517Z error: test compilation failed although it shouldn't!
2019-10-09T20:04:56.6802562Z status: exit code: 1
2019-10-09T20:04:56.6803286Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-64964.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-64964" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-64964/auxiliary" "-A" "unused"
2019-10-09T20:04:56.6803593Z ------------------------------------------
2019-10-09T20:04:56.6803641Z 
2019-10-09T20:04:56.6803829Z ------------------------------------------
2019-10-09T20:04:56.6803870Z stderr:
2019-10-09T20:04:56.6803870Z stderr:
2019-10-09T20:04:56.6804054Z ------------------------------------------
2019-10-09T20:04:56.6804118Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6804148Z 
2019-10-09T20:04:56.6804190Z error[E0698]: type inside `async fn` body must be known in this context
2019-10-09T20:04:56.6804807Z   --> /checkout/src/test/ui/async-await/issues/issue-64964.rs:17:14
2019-10-09T20:04:56.6804904Z LL |     for x in 0..10 {
2019-10-09T20:04:56.6804904Z LL |     for x in 0..10 {
2019-10-09T20:04:56.6804971Z    |              ^^^^^ cannot infer type for `{integer}`
2019-10-09T20:04:56.6805027Z    |
2019-10-09T20:04:56.6805075Z note: the type is part of the `async fn` body because of this `await`
2019-10-09T20:04:56.6805356Z   --> /checkout/src/test/ui/async-await/issues/issue-64964.rs:18:9
2019-10-09T20:04:56.6805450Z LL |         async { Some(x) }.await.unwrap();
2019-10-09T20:04:56.6805497Z    |         ^^^^^^^^^^^^^^^^^^^^^^^
2019-10-09T20:04:56.6805545Z 
2019-10-09T20:04:56.6805545Z 
2019-10-09T20:04:56.6805591Z error[E0698]: type inside `async fn` body must be known in this context
2019-10-09T20:04:56.6805836Z   --> /checkout/src/test/ui/async-await/issues/issue-64964.rs:17:14
2019-10-09T20:04:56.6805945Z LL |     for x in 0..10 {
2019-10-09T20:04:56.6805945Z LL |     for x in 0..10 {
2019-10-09T20:04:56.6805991Z    |              ^ cannot infer type for `{integer}`
2019-10-09T20:04:56.6806034Z    |
2019-10-09T20:04:56.6806099Z note: the type is part of the `async fn` body because of this `await`
2019-10-09T20:04:56.6806341Z   --> /checkout/src/test/ui/async-await/issues/issue-64964.rs:18:9
2019-10-09T20:04:56.6806461Z LL |         async { Some(x) }.await.unwrap();
2019-10-09T20:04:56.6806515Z    |         ^^^^^^^^^^^^^^^^^^^^^^^
2019-10-09T20:04:56.6806545Z 
2019-10-09T20:04:56.6806545Z 
2019-10-09T20:04:56.6806608Z error[E0698]: type inside `async fn` body must be known in this context
2019-10-09T20:04:56.6806853Z   --> /checkout/src/test/ui/async-await/issues/issue-64964.rs:17:17
2019-10-09T20:04:56.6806942Z LL |     for x in 0..10 {
2019-10-09T20:04:56.6806942Z LL |     for x in 0..10 {
2019-10-09T20:04:56.6807008Z    |                 ^^ cannot infer type for `{integer}`
2019-10-09T20:04:56.6807052Z    |
2019-10-09T20:04:56.6807098Z note: the type is part of the `async fn` body because of this `await`
2019-10-09T20:04:56.6807358Z   --> /checkout/src/test/ui/async-await/issues/issue-64964.rs:18:9
2019-10-09T20:04:56.6807449Z LL |         async { Some(x) }.await.unwrap();
2019-10-09T20:04:56.6807513Z    |         ^^^^^^^^^^^^^^^^^^^^^^^
2019-10-09T20:04:56.6807655Z 
2019-10-09T20:04:56.6807655Z 
2019-10-09T20:04:56.6807702Z error[E0698]: type inside `async fn` body must be known in this context
2019-10-09T20:04:56.6808201Z   --> /checkout/src/test/ui/async-await/issues/issue-64964.rs:17:9
2019-10-09T20:04:56.6808312Z LL |     for x in 0..10 {
---
2019-10-09T20:04:56.6838462Z ---- [ui] ui/async-await/move-part-await-return-rest-struct.rs stdout ----
2019-10-09T20:04:56.6838497Z 
2019-10-09T20:04:56.6838694Z error: test compilation failed although it shouldn't!
2019-10-09T20:04:56.6838745Z status: exit code: 1
2019-10-09T20:04:56.6839629Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/move-part-await-return-rest-struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/move-part-await-return-rest-struct" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--crate-type" "lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/move-part-await-return-rest-struct/auxiliary" "-A" "unused"
2019-10-09T20:04:56.6840038Z ------------------------------------------
2019-10-09T20:04:56.6840069Z 
2019-10-09T20:04:56.6840264Z ------------------------------------------
2019-10-09T20:04:56.6840303Z stderr:
2019-10-09T20:04:56.6840303Z stderr:
2019-10-09T20:04:56.6840483Z ------------------------------------------
2019-10-09T20:04:56.6840538Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6840616Z error: aborting due to previous error
2019-10-09T20:04:56.6840640Z 
2019-10-09T20:04:56.6840862Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6840892Z 
2019-10-09T20:04:56.6840892Z 
2019-10-09T20:04:56.6841073Z ------------------------------------------
2019-10-09T20:04:56.6841100Z 
2019-10-09T20:04:56.6841127Z 
2019-10-09T20:04:56.6841492Z ---- [ui] ui/async-await/move-part-await-return-rest-tuple.rs stdout ----
2019-10-09T20:04:56.6841521Z 
2019-10-09T20:04:56.6841788Z error: test compilation failed although it shouldn't!
2019-10-09T20:04:56.6841836Z status: exit code: 1
2019-10-09T20:04:56.6842529Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/move-part-await-return-rest-tuple.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/move-part-await-return-rest-tuple" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--crate-type" "lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/move-part-await-return-rest-tuple/auxiliary" "-A" "unused"
2019-10-09T20:04:56.6842805Z ------------------------------------------
2019-10-09T20:04:56.6842840Z 
2019-10-09T20:04:56.6843020Z ------------------------------------------
2019-10-09T20:04:56.6843057Z stderr:
2019-10-09T20:04:56.6843057Z stderr:
2019-10-09T20:04:56.6843231Z ------------------------------------------
2019-10-09T20:04:56.6843287Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6843316Z 
2019-10-09T20:04:56.6843356Z error[E0698]: type inside `async fn` body must be known in this context
2019-10-09T20:04:56.6843631Z    |
2019-10-09T20:04:56.6843631Z    |
2019-10-09T20:04:56.6843667Z LL |     let x = (vec![3], vec![4, 4]);
2019-10-09T20:04:56.6843722Z    |         ^ cannot infer type for `{integer}`
2019-10-09T20:04:56.6843765Z    |
2019-10-09T20:04:56.6843804Z note: the type is part of the `async fn` body because of this `await`
2019-10-09T20:04:56.6844071Z    |
2019-10-09T20:04:56.6844071Z    |
2019-10-09T20:04:56.6844106Z LL |     echo(x.0[0]).await;
2019-10-09T20:04:56.6844172Z 
2019-10-09T20:04:56.6844172Z 
2019-10-09T20:04:56.6844210Z error[E0698]: type inside `async fn` body must be known in this context
2019-10-09T20:04:56.6844911Z    |
2019-10-09T20:04:56.6844911Z    |
2019-10-09T20:04:56.6844953Z LL |     echo(x.0[0]).await;
2019-10-09T20:04:56.6844999Z    |          ^ cannot infer type for `{integer}`
2019-10-09T20:04:56.6845169Z    |
2019-10-09T20:04:56.6845228Z note: the type is part of the `async fn` body because of this `await`
2019-10-09T20:04:56.6845653Z    |
2019-10-09T20:04:56.6845653Z    |
2019-10-09T20:04:56.6845704Z LL |     echo(x.0[0]).await;
2019-10-09T20:04:56.6845778Z 
2019-10-09T20:04:56.6845820Z error: aborting due to 3 previous errors
2019-10-09T20:04:56.6845861Z 
2019-10-09T20:04:56.6845905Z Some errors have detailed explanations: E0433, E0698.
---
2019-10-09T20:04:56.6846814Z 
2019-10-09T20:04:56.6847044Z - error[E0506]: cannot assign to `a` because it is borrowed
2019-10-09T20:04:56.6847263Z -   --> $DIR/ret-ref.rs:16:5
2019-10-09T20:04:56.6847444Z -    |
2019-10-09T20:04:56.6847677Z - LL |     let future = multiple_named_lifetimes(&a, &b);
2019-10-09T20:04:56.6848147Z -    |                                           -- borrow of `a` occurs here
2019-10-09T20:04:56.6848330Z - LL |     a += 1;
2019-10-09T20:04:56.6848518Z -    |     ^^^^^^ assignment to borrowed `a` occurs here
2019-10-09T20:04:56.6848679Z - LL |     b += 1;
2019-10-09T20:04:56.6848856Z - LL |     let p = future.await;
2019-10-09T20:04:56.6849041Z -    |             ------ borrow later used here
2019-10-09T20:04:56.6849087Z + error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6849324Z - error[E0506]: cannot assign to `b` because it is borrowed
2019-10-09T20:04:56.6849495Z -   --> $DIR/ret-ref.rs:17:5
2019-10-09T20:04:56.6849652Z -    |
2019-10-09T20:04:56.6849652Z -    |
2019-10-09T20:04:56.6849854Z - LL |     let future = multiple_named_lifetimes(&a, &b);
2019-10-09T20:04:56.6850065Z -    |                                               -- borrow of `b` occurs here
2019-10-09T20:04:56.6850234Z - LL |     a += 1;
2019-10-09T20:04:56.6850401Z - LL |     b += 1;
2019-10-09T20:04:56.6850588Z -    |     ^^^^^^ assignment to borrowed `b` occurs here
2019-10-09T20:04:56.6850760Z - LL |     let p = future.await;
2019-10-09T20:04:56.6850954Z -    |             ------ borrow later used here
2019-10-09T20:04:56.6851031Z 22 
2019-10-09T20:04:56.6851235Z - error[E0506]: cannot assign to `a` because it is borrowed
2019-10-09T20:04:56.6851406Z -   --> $DIR/ret-ref.rs:28:5
2019-10-09T20:04:56.6851554Z -    |
2019-10-09T20:04:56.6851554Z -    |
2019-10-09T20:04:56.6851756Z - LL |     let future = multiple_named_lifetimes(&a, &b);
2019-10-09T20:04:56.6851967Z -    |                                           -- borrow of `a` occurs here
2019-10-09T20:04:56.6852149Z - LL |     let p = future.await;
2019-10-09T20:04:56.6852309Z - LL |     a += 1;
2019-10-09T20:04:56.6852507Z -    |     ^^^^^^ assignment to borrowed `a` occurs here
2019-10-09T20:04:56.6852667Z - LL |     b += 1;
2019-10-09T20:04:56.6852833Z - LL |     drop(p);
2019-10-09T20:04:56.6853019Z -    |          - borrow later used here
2019-10-09T20:04:56.6853344Z - error: aborting due to 3 previous errors
2019-10-09T20:04:56.6853508Z - 
2019-10-09T20:04:56.6853710Z - For more information about this error, try `rustc --explain E0506`.
2019-10-09T20:04:56.6853914Z + For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6853914Z + For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6853962Z 38 
2019-10-09T20:04:56.6853986Z 
2019-10-09T20:04:56.6854008Z 
2019-10-09T20:04:56.6854045Z The actual stderr differed from the expected stderr.
2019-10-09T20:04:56.6854635Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/ret-ref/ret-ref.stderr
2019-10-09T20:04:56.6855082Z To update references, rerun the tests and pass the `--bless` flag
2019-10-09T20:04:56.6855395Z To only update this specific test, also pass `--test-args async-await/multiple-lifetimes/ret-ref.rs`
2019-10-09T20:04:56.6855590Z error: 1 errors occurred comparing output.
2019-10-09T20:04:56.6855637Z status: exit code: 1
2019-10-09T20:04:56.6855637Z status: exit code: 1
2019-10-09T20:04:56.6856471Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/multiple-lifetimes/ret-ref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/ret-ref" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/ret-ref/auxiliary" "-A" "unused"
2019-10-09T20:04:56.6857581Z ------------------------------------------
2019-10-09T20:04:56.6857629Z 
2019-10-09T20:04:56.6858236Z ------------------------------------------
2019-10-09T20:04:56.6858286Z stderr:
2019-10-09T20:04:56.6858286Z stderr:
2019-10-09T20:04:56.6858482Z ------------------------------------------
2019-10-09T20:04:56.6858527Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6858607Z error: aborting due to previous error
2019-10-09T20:04:56.6858631Z 
2019-10-09T20:04:56.6858833Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6858861Z 
---
2019-10-09T20:04:56.6859399Z 
2019-10-09T20:04:56.6859602Z - error[E0733]: recursion in an `async fn` requires boxing
2019-10-09T20:04:56.6863706Z -   --> $DIR/mutually-recursive-async-impl-trait-type.rs:5:18
2019-10-09T20:04:56.6863907Z -    |
2019-10-09T20:04:56.6864257Z - LL | async fn rec_1() {
2019-10-09T20:04:56.6865246Z -    |                  ^ recursive `async fn`
2019-10-09T20:04:56.6865445Z -    |
2019-10-09T20:04:56.6865701Z -    = note: a recursive `async fn` must be rewritten to return a boxed `dyn Future`.
2019-10-09T20:04:56.6865759Z + error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6866049Z - error[E0733]: recursion in an `async fn` requires boxing
2019-10-09T20:04:56.6866284Z -   --> $DIR/mutually-recursive-async-impl-trait-type.rs:9:18
2019-10-09T20:04:56.6866463Z -    |
2019-10-09T20:04:56.6866463Z -    |
2019-10-09T20:04:56.6866676Z - LL | async fn rec_2() {
2019-10-09T20:04:56.6866891Z -    |                  ^ recursive `async fn`
2019-10-09T20:04:56.6867070Z -    |
2019-10-09T20:04:56.6867335Z -    = note: a recursive `async fn` must be rewritten to return a boxed `dyn Future`.
2019-10-09T20:04:56.6867442Z 16 
2019-10-09T20:04:56.6867673Z - error: aborting due to 2 previous errors
2019-10-09T20:04:56.6867853Z - 
2019-10-09T20:04:56.6868090Z - For more information about this error, try `rustc --explain E0733`.
2019-10-09T20:04:56.6868090Z - For more information about this error, try `rustc --explain E0733`.
2019-10-09T20:04:56.6868831Z + For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6868880Z 20 
2019-10-09T20:04:56.6868905Z 
2019-10-09T20:04:56.6868926Z 
2019-10-09T20:04:56.6868964Z The actual stderr differed from the expected stderr.
2019-10-09T20:04:56.6869420Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/mutually-recursive-async-impl-trait-type/mutually-recursive-async-impl-trait-type.stderr
2019-10-09T20:04:56.6869625Z To update references, rerun the tests and pass the `--bless` flag
2019-10-09T20:04:56.6869896Z To only update this specific test, also pass `--test-args async-await/mutually-recursive-async-impl-trait-type.rs`
2019-10-09T20:04:56.6870134Z error: 1 errors occurred comparing output.
2019-10-09T20:04:56.6870172Z status: exit code: 1
2019-10-09T20:04:56.6870172Z status: exit code: 1
2019-10-09T20:04:56.6870993Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/mutually-recursive-async-impl-trait-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/mutually-recursive-async-impl-trait-type" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/mutually-recursive-async-impl-trait-type/auxiliary" "-A" "unused"
2019-10-09T20:04:56.6871321Z ------------------------------------------
2019-10-09T20:04:56.6871361Z 
2019-10-09T20:04:56.6871712Z ------------------------------------------
2019-10-09T20:04:56.6871759Z stderr:
2019-10-09T20:04:56.6871759Z stderr:
2019-10-09T20:04:56.6871935Z ------------------------------------------
2019-10-09T20:04:56.6871987Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6872062Z error: aborting due to previous error
2019-10-09T20:04:56.6872085Z 
2019-10-09T20:04:56.6872289Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6872327Z 
2019-10-09T20:04:56.6872327Z 
2019-10-09T20:04:56.6872503Z ------------------------------------------
2019-10-09T20:04:56.6872529Z 
2019-10-09T20:04:56.6872550Z 
2019-10-09T20:04:56.6872758Z ---- [ui] ui/async-await/no-move-across-await-struct.rs stdout ----
2019-10-09T20:04:56.6872800Z diff of stderr:
2019-10-09T20:04:56.6872823Z 
2019-10-09T20:04:56.6873000Z - error[E0382]: use of moved value: `s.x`
2019-10-09T20:04:56.6873361Z -    |
2019-10-09T20:04:56.6873361Z -    |
2019-10-09T20:04:56.6873529Z - LL |     needs_vec(s.x).await;
2019-10-09T20:04:56.6873717Z -    |               --- value moved here
2019-10-09T20:04:56.6873873Z - LL |     s.x
2019-10-09T20:04:56.6874055Z -    |     ^^^ value used here after move
2019-10-09T20:04:56.6874205Z -    |
2019-10-09T20:04:56.6874858Z -    = note: move occurs because `s.x` has type `std::vec::Vec<usize>`, which does not implement the `Copy` trait
2019-10-09T20:04:56.6874927Z + error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6875031Z 11 error: aborting due to previous error
2019-10-09T20:04:56.6875073Z 12 
2019-10-09T20:04:56.6875101Z 
2019-10-09T20:04:56.6875359Z - For more information about this error, try `rustc --explain E0382`.
2019-10-09T20:04:56.6875359Z - For more information about this error, try `rustc --explain E0382`.
2019-10-09T20:04:56.6875602Z + For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6875650Z 14 
2019-10-09T20:04:56.6875677Z 
2019-10-09T20:04:56.6875717Z 
2019-10-09T20:04:56.6875774Z The actual stderr differed from the expected stderr.
2019-10-09T20:04:56.6876105Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-move-across-await-struct/no-move-across-await-struct.stderr
2019-10-09T20:04:56.6876367Z To update references, rerun the tests and pass the `--bless` flag
2019-10-09T20:04:56.6876640Z To only update this specific test, also pass `--test-args async-await/no-move-across-await-struct.rs`
2019-10-09T20:04:56.6876736Z error: 1 errors occurred comparing output.
2019-10-09T20:04:56.6876783Z status: exit code: 1
2019-10-09T20:04:56.6876783Z status: exit code: 1
2019-10-09T20:04:56.6877695Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/no-move-across-await-struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-move-across-await-struct" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--crate-type" "lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-move-across-await-struct/auxiliary" "-A" "unused"
2019-10-09T20:04:56.6878232Z ------------------------------------------
2019-10-09T20:04:56.6878271Z 
2019-10-09T20:04:56.6878452Z ------------------------------------------
2019-10-09T20:04:56.6878490Z stderr:
2019-10-09T20:04:56.6878490Z stderr:
2019-10-09T20:04:56.6878665Z ------------------------------------------
2019-10-09T20:04:56.6878721Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6878786Z error: aborting due to previous error
2019-10-09T20:04:56.6878820Z 
2019-10-09T20:04:56.6879021Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6879051Z 
2019-10-09T20:04:56.6879051Z 
2019-10-09T20:04:56.6879224Z ------------------------------------------
2019-10-09T20:04:56.6879259Z 
2019-10-09T20:04:56.6879292Z 
2019-10-09T20:04:56.6879487Z ---- [ui] ui/async-await/no-move-across-await-tuple.rs stdout ----
2019-10-09T20:04:56.6879528Z diff of stderr:
2019-10-09T20:04:56.6879559Z 
2019-10-09T20:04:56.6879747Z - error[E0382]: use of moved value: `x.1`
2019-10-09T20:04:56.6880085Z -   --> $DIR/no-move-across-await-tuple.rs:9:5
2019-10-09T20:04:56.6880129Z + error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6880177Z + 
2019-10-09T20:04:56.6880216Z + error[E0698]: type inside `async fn` body must be known in this context
2019-10-09T20:04:56.6880395Z +   --> $DIR/no-move-across-await-tuple.rs:6:9
2019-10-09T20:04:56.6880439Z 3    |
2019-10-09T20:04:56.6880594Z - LL |     drop(x.1);
2019-10-09T20:04:56.6880761Z -    |          --- value moved here
2019-10-09T20:04:56.6880933Z - LL |     nothing().await;
2019-10-09T20:04:56.6881087Z - LL |     x.1
2019-10-09T20:04:56.6881259Z -    |     ^^^ value used here after move
2019-10-09T20:04:56.6881307Z + LL |     let x = (vec![3], vec![4, 4]);
2019-10-09T20:04:56.6881362Z +    |         ^ cannot infer type for `{integer}`
2019-10-09T20:04:56.6881398Z 9    |
2019-10-09T20:04:56.6881637Z -    = note: move occurs because `x.1` has type `std::vec::Vec<usize>`, which does not implement the `Copy` trait
2019-10-09T20:04:56.6881701Z + note: the type is part of the `async fn` body because of this `await`
2019-10-09T20:04:56.6881886Z +   --> $DIR/no-move-across-await-tuple.rs:8:5
2019-10-09T20:04:56.6881923Z +    |
2019-10-09T20:04:56.6881966Z + LL |     nothing().await;
2019-10-09T20:04:56.6882035Z 11 
2019-10-09T20:04:56.6882210Z - error: aborting due to previous error
2019-10-09T20:04:56.6882250Z + error: aborting due to 2 previous errors
2019-10-09T20:04:56.6882284Z 13 
2019-10-09T20:04:56.6882284Z 13 
2019-10-09T20:04:56.6882477Z - For more information about this error, try `rustc --explain E0382`.
2019-10-09T20:04:56.6882527Z + Some errors have detailed explanations: E0433, E0698.
2019-10-09T20:04:56.6882729Z + For more information about an error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6882767Z 15 
2019-10-09T20:04:56.6882796Z 
2019-10-09T20:04:56.6882823Z 
2019-10-09T20:04:56.6882860Z The actual stderr differed from the expected stderr.
2019-10-09T20:04:56.6883121Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-move-across-await-tuple/no-move-across-await-tuple.stderr
2019-10-09T20:04:56.6883337Z To update references, rerun the tests and pass the `--bless` flag
2019-10-09T20:04:56.6883557Z To only update this specific test, also pass `--test-args async-await/no-move-across-await-tuple.rs`
2019-10-09T20:04:56.6883638Z error: 1 errors occurred comparing output.
2019-10-09T20:04:56.6883674Z status: exit code: 1
2019-10-09T20:04:56.6883674Z status: exit code: 1
2019-10-09T20:04:56.6884757Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/no-move-across-await-tuple.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-move-across-await-tuple" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--crate-type" "lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-move-across-await-tuple/auxiliary" "-A" "unused"
2019-10-09T20:04:56.6885204Z ------------------------------------------
2019-10-09T20:04:56.6885241Z 
2019-10-09T20:04:56.6885453Z ------------------------------------------
2019-10-09T20:04:56.6885498Z stderr:
2019-10-09T20:04:56.6885498Z stderr:
2019-10-09T20:04:56.6885723Z ------------------------------------------
2019-10-09T20:04:56.6885776Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6885809Z 
2019-10-09T20:04:56.6885868Z error[E0698]: type inside `async fn` body must be known in this context
2019-10-09T20:04:56.6886180Z    |
2019-10-09T20:04:56.6886180Z    |
2019-10-09T20:04:56.6886232Z LL |     let x = (vec![3], vec![4, 4]);
2019-10-09T20:04:56.6886288Z    |         ^ cannot infer type for `{integer}`
2019-10-09T20:04:56.6886331Z    |
2019-10-09T20:04:56.6886378Z note: the type is part of the `async fn` body because of this `await`
2019-10-09T20:04:56.6886685Z    |
2019-10-09T20:04:56.6886685Z    |
2019-10-09T20:04:56.6886726Z LL |     nothing().await;
2019-10-09T20:04:56.6886805Z 
2019-10-09T20:04:56.6886848Z error: aborting due to 2 previous errors
2019-10-09T20:04:56.6886877Z 
2019-10-09T20:04:56.6886928Z Some errors have detailed explanations: E0433, E0698.
---
2019-10-09T20:04:56.6887829Z 
2019-10-09T20:04:56.6888360Z - error[E0381]: use of possibly-uninitialized variable: `y`
2019-10-09T20:04:56.6888546Z -   --> $DIR/no-non-guaranteed-initialization.rs:10:5
2019-10-09T20:04:56.6888703Z -    |
2019-10-09T20:04:56.6888854Z - LL |     y
2019-10-09T20:04:56.6889030Z -    |     ^ use of possibly-uninitialized `y`
2019-10-09T20:04:56.6889072Z + error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6889151Z 7 error: aborting due to previous error
2019-10-09T20:04:56.6889185Z 8 
2019-10-09T20:04:56.6889206Z 
2019-10-09T20:04:56.6889409Z - For more information about this error, try `rustc --explain E0381`.
2019-10-09T20:04:56.6889409Z - For more information about this error, try `rustc --explain E0381`.
2019-10-09T20:04:56.6889614Z + For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6889651Z 10 
2019-10-09T20:04:56.6889679Z 
2019-10-09T20:04:56.6889700Z 
2019-10-09T20:04:56.6889742Z The actual stderr differed from the expected stderr.
2019-10-09T20:04:56.6890013Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-non-guaranteed-initialization/no-non-guaranteed-initialization.stderr
2019-10-09T20:04:56.6890228Z To update references, rerun the tests and pass the `--bless` flag
2019-10-09T20:04:56.6890474Z To only update this specific test, also pass `--test-args async-await/no-non-guaranteed-initialization.rs`
2019-10-09T20:04:56.6890558Z error: 1 errors occurred comparing output.
2019-10-09T20:04:56.6890597Z status: exit code: 1
2019-10-09T20:04:56.6890597Z status: exit code: 1
2019-10-09T20:04:56.6891364Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/no-non-guaranteed-initialization.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-non-guaranteed-initialization" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--crate-type" "lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-non-guaranteed-initialization/auxiliary" "-A" "unused"
2019-10-09T20:04:56.6891751Z ------------------------------------------
2019-10-09T20:04:56.6891783Z 
2019-10-09T20:04:56.6891976Z ------------------------------------------
2019-10-09T20:04:56.6892014Z stderr:
2019-10-09T20:04:56.6892014Z stderr:
2019-10-09T20:04:56.6892208Z ------------------------------------------
2019-10-09T20:04:56.6892252Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6892334Z error: aborting due to previous error
2019-10-09T20:04:56.6892368Z 
2019-10-09T20:04:56.6892583Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6892613Z 
---
2019-10-09T20:04:56.6893150Z 
2019-10-09T20:04:56.6893352Z - error[E0733]: recursion in an `async fn` requires boxing
2019-10-09T20:04:56.6893552Z -   --> $DIR/recursive-async-impl-trait-type.rs:5:40
2019-10-09T20:04:56.6893722Z -    |
2019-10-09T20:04:56.6893916Z - LL | async fn recursive_async_function() -> () {
2019-10-09T20:04:56.6894124Z -    |                                        ^^ recursive `async fn`
2019-10-09T20:04:56.6894455Z -    |
2019-10-09T20:04:56.6894913Z -    = note: a recursive `async fn` must be rewritten to return a boxed `dyn Future`.
2019-10-09T20:04:56.6894984Z + error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6895082Z 9 error: aborting due to previous error
2019-10-09T20:04:56.6895131Z 10 
2019-10-09T20:04:56.6895160Z 
2019-10-09T20:04:56.6895415Z - For more information about this error, try `rustc --explain E0733`.
2019-10-09T20:04:56.6895415Z - For more information about this error, try `rustc --explain E0733`.
2019-10-09T20:04:56.6895657Z + For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6895704Z 12 
2019-10-09T20:04:56.6895730Z 
2019-10-09T20:04:56.6895768Z 
2019-10-09T20:04:56.6895813Z The actual stderr differed from the expected stderr.
2019-10-09T20:04:56.6896141Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/recursive-async-impl-trait-type/recursive-async-impl-trait-type.stderr
2019-10-09T20:04:56.6896405Z To update references, rerun the tests and pass the `--bless` flag
2019-10-09T20:04:56.6896686Z To only update this specific test, also pass `--test-args async-await/recursive-async-impl-trait-type.rs`
2019-10-09T20:04:56.6896789Z error: 1 errors occurred comparing output.
2019-10-09T20:04:56.6896834Z status: exit code: 1
2019-10-09T20:04:56.6896834Z status: exit code: 1
2019-10-09T20:04:56.6897638Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/recursive-async-impl-trait-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/recursive-async-impl-trait-type" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/recursive-async-impl-trait-type/auxiliary" "-A" "unused"
2019-10-09T20:04:56.6898124Z ------------------------------------------
2019-10-09T20:04:56.6898158Z 
2019-10-09T20:04:56.6898331Z ------------------------------------------
2019-10-09T20:04:56.6898469Z stderr:
2019-10-09T20:04:56.6898469Z stderr:
2019-10-09T20:04:56.6898664Z ------------------------------------------
2019-10-09T20:04:56.6898781Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6898856Z error: aborting due to previous error
2019-10-09T20:04:56.6898879Z 
2019-10-09T20:04:56.6899108Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6899137Z 
---
2019-10-09T20:04:56.6899622Z 
2019-10-09T20:04:56.6899829Z - error[E0381]: assign to part of possibly-uninitialized variable: `t`
2019-10-09T20:04:56.6900018Z -   --> $DIR/partial-initialization-across-await.rs:13:5
2019-10-09T20:04:56.6900175Z -    |
2019-10-09T20:04:56.6900498Z - LL |     t.0 = 42;
2019-10-09T20:04:56.6900675Z -    |     ^^^^^^^^ use of possibly-uninitialized `t`
2019-10-09T20:04:56.6900724Z + error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6900960Z - error[E0381]: assign to part of possibly-uninitialized variable: `t`
2019-10-09T20:04:56.6901140Z -   --> $DIR/partial-initialization-across-await.rs:22:5
2019-10-09T20:04:56.6901298Z -    |
2019-10-09T20:04:56.6901298Z -    |
2019-10-09T20:04:56.6901450Z - LL |     t.0 = 42;
2019-10-09T20:04:56.6901625Z -    |     ^^^^^^^^ use of possibly-uninitialized `t`
2019-10-09T20:04:56.6901704Z 12 
2019-10-09T20:04:56.6901892Z - error[E0381]: assign to part of possibly-uninitialized variable: `t`
2019-10-09T20:04:56.6902074Z -   --> $DIR/partial-initialization-across-await.rs:31:5
2019-10-09T20:04:56.6902377Z -    |
2019-10-09T20:04:56.6902377Z -    |
2019-10-09T20:04:56.6902537Z - LL |     t.x = 42;
2019-10-09T20:04:56.6902725Z -    |     ^^^^^^^^ use of possibly-uninitialized `t`
2019-10-09T20:04:56.6903037Z - error: aborting due to 3 previous errors
2019-10-09T20:04:56.6903173Z - 
2019-10-09T20:04:56.6903366Z - For more information about this error, try `rustc --explain E0381`.
2019-10-09T20:04:56.6903575Z + For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6903575Z + For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6903612Z 22 
2019-10-09T20:04:56.6903633Z 
2019-10-09T20:04:56.6903654Z 
2019-10-09T20:04:56.6903696Z The actual stderr differed from the expected stderr.
2019-10-09T20:04:56.6903961Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/partial-initialization-across-await/partial-initialization-across-await.stderr
2019-10-09T20:04:56.6904162Z To update references, rerun the tests and pass the `--bless` flag
2019-10-09T20:04:56.6904841Z To only update this specific test, also pass `--test-args async-await/partial-initialization-across-await.rs`
2019-10-09T20:04:56.6904943Z error: 1 errors occurred comparing output.
2019-10-09T20:04:56.6905003Z status: exit code: 1
2019-10-09T20:04:56.6905003Z status: exit code: 1
2019-10-09T20:04:56.6905823Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/partial-initialization-across-await.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/partial-initialization-across-await" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/partial-initialization-across-await/auxiliary" "-A" "unused"
2019-10-09T20:04:56.6906153Z ------------------------------------------
2019-10-09T20:04:56.6906186Z 
2019-10-09T20:04:56.6906409Z ------------------------------------------
2019-10-09T20:04:56.6906592Z stderr:
2019-10-09T20:04:56.6906592Z stderr:
2019-10-09T20:04:56.6906835Z ------------------------------------------
2019-10-09T20:04:56.6906899Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6907073Z error: aborting due to previous error
2019-10-09T20:04:56.6907102Z 
2019-10-09T20:04:56.6907377Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6907412Z 
2019-10-09T20:04:56.6907412Z 
2019-10-09T20:04:56.6907620Z ------------------------------------------
2019-10-09T20:04:56.6907652Z 
2019-10-09T20:04:56.6907685Z 
2019-10-09T20:04:56.6908235Z ---- [ui] ui/async-await/suggest-missing-await-closure.rs stdout ----
2019-10-09T20:04:56.6908469Z 
2019-10-09T20:04:56.6908505Z error: failed to compile fixed code
2019-10-09T20:04:56.6908710Z status: exit code: 1
2019-10-09T20:04:56.6909344Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/suggest-missing-await-closure.fixed" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/suggest-missing-await-closure/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/suggest-missing-await-closure/auxiliary"
2019-10-09T20:04:56.6909768Z ------------------------------------------
2019-10-09T20:04:56.6909794Z 
2019-10-09T20:04:56.6909979Z ------------------------------------------
2019-10-09T20:04:56.6910015Z stderr:
2019-10-09T20:04:56.6910015Z stderr:
2019-10-09T20:04:56.6910177Z ------------------------------------------
2019-10-09T20:04:56.6910232Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6910293Z error: aborting due to previous error
2019-10-09T20:04:56.6910323Z 
2019-10-09T20:04:56.6910521Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6910549Z 
2019-10-09T20:04:56.6910549Z 
2019-10-09T20:04:56.6910882Z ------------------------------------------
2019-10-09T20:04:56.6910915Z 
2019-10-09T20:04:56.6910935Z 
2019-10-09T20:04:56.6911140Z ---- [ui] ui/async-await/suggest-missing-await.rs stdout ----
2019-10-09T20:04:56.6911168Z 
2019-10-09T20:04:56.6911202Z error: failed to compile fixed code
2019-10-09T20:04:56.6911248Z status: exit code: 1
2019-10-09T20:04:56.6912006Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/suggest-missing-await.fixed" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/suggest-missing-await/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/suggest-missing-await/auxiliary"
2019-10-09T20:04:56.6912259Z ------------------------------------------
2019-10-09T20:04:56.6912292Z 
2019-10-09T20:04:56.6912465Z ------------------------------------------
2019-10-09T20:04:56.6912501Z stderr:
2019-10-09T20:04:56.6912501Z stderr:
2019-10-09T20:04:56.6912666Z ------------------------------------------
2019-10-09T20:04:56.6912706Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6912772Z error: aborting due to previous error
2019-10-09T20:04:56.6912795Z 
2019-10-09T20:04:56.6912989Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6913017Z 
2019-10-09T20:04:56.6913017Z 
2019-10-09T20:04:56.6913181Z ------------------------------------------
2019-10-09T20:04:56.6913205Z 
2019-10-09T20:04:56.6913225Z 
2019-10-09T20:04:56.6913413Z ---- [ui] ui/async-await/unreachable-lint-1.rs stdout ----
2019-10-09T20:04:56.6913530Z diff of stderr:
2019-10-09T20:04:56.6913552Z 
2019-10-09T20:04:56.6913589Z + error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6913719Z 1 error: unreachable statement
2019-10-09T20:04:56.6913927Z 2   --> $DIR/unreachable-lint-1.rs:5:13
2019-10-09T20:04:56.6913973Z 3    |
2019-10-09T20:04:56.6913996Z 
---
2019-10-09T20:04:56.6915078Z 17 
2019-10-09T20:04:56.6915106Z 
2019-10-09T20:04:56.6915139Z 
2019-10-09T20:04:56.6915184Z The actual stderr differed from the expected stderr.
2019-10-09T20:04:56.6915491Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/unreachable-lint-1/unreachable-lint-1.stderr
2019-10-09T20:04:56.6915765Z To update references, rerun the tests and pass the `--bless` flag
2019-10-09T20:04:56.6916040Z To only update this specific test, also pass `--test-args async-await/unreachable-lint-1.rs`
2019-10-09T20:04:56.6916136Z error: 1 errors occurred comparing output.
2019-10-09T20:04:56.6916180Z status: exit code: 1
2019-10-09T20:04:56.6916180Z status: exit code: 1
2019-10-09T20:04:56.6916937Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/unreachable-lint-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/unreachable-lint-1" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/unreachable-lint-1/auxiliary" "-A" "unused"
2019-10-09T20:04:56.6917270Z ------------------------------------------
2019-10-09T20:04:56.6917312Z 
2019-10-09T20:04:56.6917533Z ------------------------------------------
2019-10-09T20:04:56.6917578Z stderr:
2019-10-09T20:04:56.6917578Z stderr:
2019-10-09T20:04:56.6917783Z ------------------------------------------
2019-10-09T20:04:56.6917843Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6917919Z error: unreachable statement
2019-10-09T20:04:56.6918326Z   --> /checkout/src/test/ui/async-await/unreachable-lint-1.rs:5:13
2019-10-09T20:04:56.6918365Z    |
2019-10-09T20:04:56.6918365Z    |
2019-10-09T20:04:56.6918400Z LL |     return; bar().await;
2019-10-09T20:04:56.6918574Z    |     ------  ^^^^^^^^^^^^ unreachable statement
2019-10-09T20:04:56.6918660Z    |     any code following this expression is unreachable
2019-10-09T20:04:56.6918701Z    |
2019-10-09T20:04:56.6918739Z note: lint level defined here
2019-10-09T20:04:56.6918929Z   --> /checkout/src/test/ui/async-await/unreachable-lint-1.rs:2:9
---
2019-10-09T20:04:56.6919736Z ---- [ui] ui/async-await/unreachable-lint.rs stdout ----
2019-10-09T20:04:56.6919762Z 
2019-10-09T20:04:56.6919941Z error: test compilation failed although it shouldn't!
2019-10-09T20:04:56.6919980Z status: exit code: 1
2019-10-09T20:04:56.6920672Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/unreachable-lint.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/unreachable-lint" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/unreachable-lint/auxiliary" "-A" "unused"
2019-10-09T20:04:56.6921012Z ------------------------------------------
2019-10-09T20:04:56.6921202Z 
2019-10-09T20:04:56.6921368Z ------------------------------------------
2019-10-09T20:04:56.6921403Z stderr:
2019-10-09T20:04:56.6921403Z stderr:
2019-10-09T20:04:56.6921561Z ------------------------------------------
2019-10-09T20:04:56.6921607Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6921674Z error: aborting due to previous error
2019-10-09T20:04:56.6921696Z 
2019-10-09T20:04:56.6921886Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6921919Z 
2019-10-09T20:04:56.6921919Z 
2019-10-09T20:04:56.6922078Z ------------------------------------------
2019-10-09T20:04:56.6922102Z 
2019-10-09T20:04:56.6922128Z 
2019-10-09T20:04:56.6922303Z ---- [ui] ui/async-await/unresolved_type_param.rs stdout ----
2019-10-09T20:04:56.6922339Z diff of stderr:
2019-10-09T20:04:56.6922360Z 
2019-10-09T20:04:56.6922549Z - error[E0698]: type inside `async fn` body must be known in this context
2019-10-09T20:04:56.6922849Z -    |
2019-10-09T20:04:56.6922849Z -    |
2019-10-09T20:04:56.6923002Z - LL |     bar().await;
2019-10-09T20:04:56.6923297Z -    |
2019-10-09T20:04:56.6923297Z -    |
2019-10-09T20:04:56.6923484Z - note: the type is part of the `async fn` body because of this `await`
2019-10-09T20:04:56.6923799Z -    |
2019-10-09T20:04:56.6923799Z -    |
2019-10-09T20:04:56.6923945Z - LL |     bar().await;
2019-10-09T20:04:56.6924099Z -    |     ^^^^^^^^^^^
2019-10-09T20:04:56.6924146Z + error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6924219Z 13 error: aborting due to previous error
2019-10-09T20:04:56.6924251Z 14 
2019-10-09T20:04:56.6924395Z 
2019-10-09T20:04:56.6924860Z - For more information about this error, try `rustc --explain E0698`.
2019-10-09T20:04:56.6924860Z - For more information about this error, try `rustc --explain E0698`.
2019-10-09T20:04:56.6925115Z + For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6925162Z 16 
2019-10-09T20:04:56.6925189Z 
2019-10-09T20:04:56.6925215Z 
2019-10-09T20:04:56.6925266Z The actual stderr differed from the expected stderr.
2019-10-09T20:04:56.6925576Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/unresolved_type_param/unresolved_type_param.stderr
2019-10-09T20:04:56.6925835Z To update references, rerun the tests and pass the `--bless` flag
2019-10-09T20:04:56.6926127Z To only update this specific test, also pass `--test-args async-await/unresolved_type_param.rs`
2019-10-09T20:04:56.6926223Z error: 1 errors occurred comparing output.
2019-10-09T20:04:56.6926276Z status: exit code: 1
2019-10-09T20:04:56.6926276Z status: exit code: 1
2019-10-09T20:04:56.6927060Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/unresolved_type_param.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/unresolved_type_param" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/unresolved_type_param/auxiliary" "-A" "unused"
2019-10-09T20:04:56.6927544Z ------------------------------------------
2019-10-09T20:04:56.6927582Z 
2019-10-09T20:04:56.6927820Z ------------------------------------------
2019-10-09T20:04:56.6928111Z stderr:
2019-10-09T20:04:56.6928111Z stderr:
2019-10-09T20:04:56.6928319Z ------------------------------------------
2019-10-09T20:04:56.6928369Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6928436Z error: aborting due to previous error
2019-10-09T20:04:56.6928459Z 
2019-10-09T20:04:56.6928662Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6928689Z 
2019-10-09T20:04:56.6928689Z 
2019-10-09T20:04:56.6928859Z ------------------------------------------
2019-10-09T20:04:56.6928884Z 
2019-10-09T20:04:56.6928905Z 
2019-10-09T20:04:56.6929086Z ---- [ui] ui/drop/dynamic-drop-async.rs stdout ----
2019-10-09T20:04:56.6929113Z 
2019-10-09T20:04:56.6929291Z error: test compilation failed although it shouldn't!
2019-10-09T20:04:56.6929336Z status: exit code: 1
2019-10-09T20:04:56.6930101Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/drop/dynamic-drop-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dynamic-drop-async/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dynamic-drop-async/auxiliary"
2019-10-09T20:04:56.6930499Z ------------------------------------------
2019-10-09T20:04:56.6930525Z 
2019-10-09T20:04:56.6930693Z ------------------------------------------
2019-10-09T20:04:56.6930728Z stderr:
2019-10-09T20:04:56.6930728Z stderr:
2019-10-09T20:04:56.6930887Z ------------------------------------------
2019-10-09T20:04:56.6930927Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6930966Z 
2019-10-09T20:04:56.6931003Z error[E0698]: type inside `async fn` body must be known in this context
2019-10-09T20:04:56.6931243Z    |
2019-10-09T20:04:56.6931276Z LL |     for i in 0..2 {
2019-10-09T20:04:56.6931276Z LL |     for i in 0..2 {
2019-10-09T20:04:56.6931311Z    |              ^^^^ cannot infer type for `{integer}`
2019-10-09T20:04:56.6931350Z    |
2019-10-09T20:04:56.6931386Z note: the type is part of the `async fn` body because of this `await`
2019-10-09T20:04:56.6931608Z    |
2019-10-09T20:04:56.6931608Z    |
2019-10-09T20:04:56.6931647Z LL |                 a.alloc().await;
2019-10-09T20:04:56.6931705Z 
2019-10-09T20:04:56.6931705Z 
2019-10-09T20:04:56.6931745Z error[E0698]: type inside `async fn` body must be known in this context
2019-10-09T20:04:56.6931971Z    |
2019-10-09T20:04:56.6932007Z LL |     for i in 0..2 {
2019-10-09T20:04:56.6932007Z LL |     for i in 0..2 {
2019-10-09T20:04:56.6932043Z    |              ^ cannot infer type for `{integer}`
2019-10-09T20:04:56.6932076Z    |
2019-10-09T20:04:56.6932117Z note: the type is part of the `async fn` body because of this `await`
2019-10-09T20:04:56.6932343Z    |
2019-10-09T20:04:56.6932343Z    |
2019-10-09T20:04:56.6932376Z LL |                 a.alloc().await;
2019-10-09T20:04:56.6932439Z 
2019-10-09T20:04:56.6932439Z 
2019-10-09T20:04:56.6932474Z error[E0698]: type inside `async fn` body must be known in this context
2019-10-09T20:04:56.6932698Z    |
2019-10-09T20:04:56.6932729Z LL |     for i in 0..2 {
2019-10-09T20:04:56.6932729Z LL |     for i in 0..2 {
2019-10-09T20:04:56.6932764Z    |                 ^ cannot infer type for `{integer}`
2019-10-09T20:04:56.6932802Z    |
2019-10-09T20:04:56.6932838Z note: the type is part of the `async fn` body because of this `await`
2019-10-09T20:04:56.6933166Z    |
2019-10-09T20:04:56.6933166Z    |
2019-10-09T20:04:56.6933259Z LL |                 a.alloc().await;
2019-10-09T20:04:56.6933327Z 
2019-10-09T20:04:56.6933327Z 
2019-10-09T20:04:56.6933369Z error[E0698]: type inside `async fn` body must be known in this context
2019-10-09T20:04:56.6933612Z    |
2019-10-09T20:04:56.6933648Z LL |     for i in 0..2 {
2019-10-09T20:04:56.6933648Z LL |     for i in 0..2 {
2019-10-09T20:04:56.6933683Z    |         ^ cannot infer type for `{integer}`
2019-10-09T20:04:56.6933716Z    |
2019-10-09T20:04:56.6933755Z note: the type is part of the `async fn` body because of this `await`
2019-10-09T20:04:56.6933976Z    |
2019-10-09T20:04:56.6933976Z    |
2019-10-09T20:04:56.6934181Z LL |                 a.alloc().await;
2019-10-09T20:04:56.6934251Z 
2019-10-09T20:04:56.6934251Z 
2019-10-09T20:04:56.6934288Z error[E0698]: type inside `async fn` body must be known in this context
2019-10-09T20:04:56.6934986Z    |
2019-10-09T20:04:56.6934986Z    |
2019-10-09T20:04:56.6935028Z LL |     let (x, y, z);
2019-10-09T20:04:56.6935079Z    |             ^ cannot infer type for `{integer}`
2019-10-09T20:04:56.6935123Z    |
2019-10-09T20:04:56.6935169Z note: the type is part of the `async fn` body because of this `await`
2019-10-09T20:04:56.6935462Z    |
2019-10-09T20:04:56.6935462Z    |
2019-10-09T20:04:56.6935504Z LL |     z = a.alloc().await;
2019-10-09T20:04:56.6935582Z 
2019-10-09T20:04:56.6935626Z error: aborting due to 6 previous errors
2019-10-09T20:04:56.6935655Z 
2019-10-09T20:04:56.6935698Z Some errors have detailed explanations: E0433, E0698.
---
2019-10-09T20:04:56.6936504Z ---- [ui] ui/generator/issue-61442-stmt-expr-with-drop.rs stdout ----
2019-10-09T20:04:56.6936538Z 
2019-10-09T20:04:56.6936755Z error: test compilation failed although it shouldn't!
2019-10-09T20:04:56.6936810Z status: exit code: 1
2019-10-09T20:04:56.6937615Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/issue-61442-stmt-expr-with-drop.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/issue-61442-stmt-expr-with-drop" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/issue-61442-stmt-expr-with-drop/auxiliary" "-A" "unused"
2019-10-09T20:04:56.6938247Z ------------------------------------------
2019-10-09T20:04:56.6938274Z 
2019-10-09T20:04:56.6938447Z ------------------------------------------
2019-10-09T20:04:56.6938483Z stderr:
2019-10-09T20:04:56.6938483Z stderr:
2019-10-09T20:04:56.6938647Z ------------------------------------------
2019-10-09T20:04:56.6938693Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6938754Z error: aborting due to previous error
2019-10-09T20:04:56.6938776Z 
2019-10-09T20:04:56.6938969Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6938996Z 
2019-10-09T20:04:56.6938996Z 
2019-10-09T20:04:56.6939161Z ------------------------------------------
2019-10-09T20:04:56.6939186Z 
2019-10-09T20:04:56.6939206Z 
2019-10-09T20:04:56.6939523Z ---- [ui] ui/generator/issue-62506-two_awaits.rs stdout ----
2019-10-09T20:04:56.6939551Z 
2019-10-09T20:04:56.6939877Z error: test compilation failed although it shouldn't!
2019-10-09T20:04:56.6939998Z status: exit code: 1
2019-10-09T20:04:56.6941150Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/issue-62506-two_awaits.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/issue-62506-two_awaits" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/issue-62506-two_awaits/auxiliary" "-A" "unused"
2019-10-09T20:04:56.6941578Z ------------------------------------------
2019-10-09T20:04:56.6941614Z 
2019-10-09T20:04:56.6941794Z ------------------------------------------
2019-10-09T20:04:56.6941831Z stderr:
2019-10-09T20:04:56.6941831Z stderr:
2019-10-09T20:04:56.6942000Z ------------------------------------------
2019-10-09T20:04:56.6942049Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6942463Z error: aborting due to previous error
2019-10-09T20:04:56.6942500Z 
2019-10-09T20:04:56.6942757Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6942787Z 
---
2019-10-09T20:04:56.6943789Z -    |
2019-10-09T20:04:56.6943962Z - LL | /     if true {
2019-10-09T20:04:56.6944338Z - LL | |         thing_one()
2019-10-09T20:04:56.6945349Z -    | |         ----------- expected because of this
2019-10-09T20:04:56.6945561Z - LL | |     } else {
2019-10-09T20:04:56.6945771Z - LL | |         thing_two()
2019-10-09T20:04:56.6946022Z -    | |         ^^^^^^^^^^^ expected opaque type, found a different opaque type
2019-10-09T20:04:56.6946215Z - LL | |     }.await
2019-10-09T20:04:56.6946441Z -    | |_____- if and else have incompatible types
2019-10-09T20:04:56.6946624Z -    |
2019-10-09T20:04:56.6946898Z -    = note: expected type `impl std::future::Future` (opaque type at <$DIR/opaque-type-error.rs:8:19>)
2019-10-09T20:04:56.6947187Z -               found type `impl std::future::Future` (opaque type at <$DIR/opaque-type-error.rs:12:19>)
2019-10-09T20:04:56.6947442Z -    = note: distinct uses of `impl Trait` result in different opaque types
2019-10-09T20:04:56.6947710Z -    = help: if both `Future`s have the same `Output` type, consider `.await`ing on both of them
2019-10-09T20:04:56.6947785Z + error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6947881Z 18 error: aborting due to previous error
2019-10-09T20:04:56.6947928Z 19 
2019-10-09T20:04:56.6947957Z 
2019-10-09T20:04:56.6948199Z - For more information about this error, try `rustc --explain E0308`.
2019-10-09T20:04:56.6948199Z - For more information about this error, try `rustc --explain E0308`.
2019-10-09T20:04:56.6948568Z + For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6948609Z 21 
2019-10-09T20:04:56.6948630Z 
2019-10-09T20:04:56.6948650Z 
2019-10-09T20:04:56.6948684Z The actual stderr differed from the expected stderr.
2019-10-09T20:04:56.6948918Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/opaque-type-error/opaque-type-error.stderr
2019-10-09T20:04:56.6949106Z To update references, rerun the tests and pass the `--bless` flag
2019-10-09T20:04:56.6949478Z To only update this specific test, also pass `--test-args suggestions/opaque-type-error.rs`
2019-10-09T20:04:56.6949731Z error: 1 errors occurred comparing output.
2019-10-09T20:04:56.6949766Z status: exit code: 1
2019-10-09T20:04:56.6949766Z status: exit code: 1
2019-10-09T20:04:56.6950475Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/opaque-type-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/opaque-type-error" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/opaque-type-error/auxiliary" "-A" "unused"
2019-10-09T20:04:56.6950916Z ------------------------------------------
2019-10-09T20:04:56.6950943Z 
2019-10-09T20:04:56.6951108Z ------------------------------------------
2019-10-09T20:04:56.6951150Z stderr:
2019-10-09T20:04:56.6951150Z stderr:
2019-10-09T20:04:56.6951317Z ------------------------------------------
2019-10-09T20:04:56.6951363Z error[E0433]: failed to resolve: could not find `IntoFuture` in `future`
2019-10-09T20:04:56.6951428Z error: aborting due to previous error
2019-10-09T20:04:56.6951450Z 
2019-10-09T20:04:56.6951636Z For more information about this error, try `rustc --explain E0433`.
2019-10-09T20:04:56.6951663Z 
---
2019-10-09T20:04:56.6965284Z test result: FAILED. 9046 passed; 57 failed; 40 ignored; 0 measured; 0 filtered out
2019-10-09T20:04:56.6965325Z 
2019-10-09T20:04:56.6965354Z 
2019-10-09T20:04:56.6965387Z 
2019-10-09T20:04:56.6966852Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-09T20:04:56.6967089Z 
2019-10-09T20:04:56.6967116Z 
2019-10-09T20:04:56.6967162Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-09T20:04:56.6967211Z Build completed unsuccessfully in 1:04:57
2019-10-09T20:04:56.6967211Z Build completed unsuccessfully in 1:04:57
2019-10-09T20:04:56.6967374Z == clock drift check ==
2019-10-09T20:04:56.6967421Z   local time: Wed Oct  9 20:04:56 UTC 2019
2019-10-09T20:04:56.6967803Z   network time: thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-09T20:04:56.6968056Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-09T20:04:56.8240125Z Wed, 09 Oct 2019 20:04:56 GMT
2019-10-09T20:04:56.8241357Z == end clock drift check ==
2019-10-09T20:04:57.4930385Z ##[error]Bash exited with code '1'.
2019-10-09T20:04:57.4989922Z ##[section]Starting: Checkout
2019-10-09T20:04:57.4991653Z ==============================================================================
2019-10-09T20:04:57.4991699Z Task         : Get sources
2019-10-09T20:04:57.4991756Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
