plain
2019-10-09T20:13:47.6584932Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-09T20:13:47.6667373Z ##[command]git config gc.auto 0
2019-10-09T20:13:47.6753092Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-09T20:13:47.6816425Z ##[command]git config --get-all http.proxy
2019-10-09T20:13:47.6959726Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65244/merge:refs/remotes/pull/65244/merge
---
2019-10-09T21:14:00.9160674Z .................................................................................................... 1600/9143
2019-10-09T21:14:07.7964727Z .................................................................................................... 1700/9143
2019-10-09T21:14:19.6234359Z ..................i...............i................................................................. 1800/9143
2019-10-09T21:14:26.7683310Z .................................................................................................... 1900/9143
2019-10-09T21:14:42.7264130Z .........iiiii...................................................................................... 2000/9143
2019-10-09T21:14:52.6670459Z .................................................................................................... 2200/9143
2019-10-09T21:14:55.2978812Z .................................................................................................... 2300/9143
2019-10-09T21:15:01.0230866Z .................................................................................................... 2400/9143
2019-10-09T21:15:07.2382123Z .................................................................................................... 2500/9143
---
2019-10-09T21:18:00.3642587Z .................................................................................................... 4700/9143
2019-10-09T21:18:07.4714399Z ..i...............i................................................................................. 4800/9143
2019-10-09T21:18:18.7019659Z .................................................................................................... 4900/9143
2019-10-09T21:18:24.2907157Z .................................................................................................... 5000/9143
2019-10-09T21:18:35.7028914Z ................................................................................................ii.i 5100/9143
2019-10-09T21:18:40.8837536Z i................................................................................................... 5200/9143
2019-10-09T21:18:55.6917280Z .................................................................................................... 5400/9143
2019-10-09T21:19:02.8643200Z ..............................................................i..................................... 5500/9143
2019-10-09T21:19:10.2412378Z .................................................................................................... 5600/9143
2019-10-09T21:19:17.5656182Z .................................................................................................... 5700/9143
2019-10-09T21:19:17.5656182Z .................................................................................................... 5700/9143
2019-10-09T21:19:29.1531002Z ...........................................................ii...i..ii...........i................... 5800/9143
2019-10-09T21:19:54.6449404Z .................................................................................................... 6000/9143
2019-10-09T21:20:03.8555964Z .................................................................................................... 6100/9143
2019-10-09T21:20:03.8555964Z .................................................................................................... 6100/9143
2019-10-09T21:20:13.9561393Z .................................................................i..ii.............................. 6200/9143
2019-10-09T21:20:43.0528873Z .................................................................................................... 6400/9143
2019-10-09T21:20:45.1479232Z .........................i.......................................................................... 6500/9143
2019-10-09T21:20:47.3282353Z ..................................................................................................i. 6600/9143
2019-10-09T21:20:50.0655760Z .................................................................................................... 6700/9143
---
2019-10-09T21:25:01.5052443Z ---- [ui] ui/async-await/async-fn-nonsend.rs stdout ----
2019-10-09T21:25:01.5052790Z diff of stderr:
2019-10-09T21:25:01.5052944Z 
2019-10-09T21:25:01.5053089Z 9    |
2019-10-09T21:25:01.5053275Z 10    = help: within `impl std::future::Future`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<()>`
2019-10-09T21:25:01.5053437Z 11    = note: required because it appears within the type `impl std::fmt::Debug`
2019-10-09T21:25:01.5054111Z -    = note: required because it appears within the type `{impl std::fmt::Debug, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, ()}`
2019-10-09T21:25:01.5054620Z -    = note: required because it appears within the type `[static generator@$DIR/async-fn-nonsend.rs:21:39: 26:2 {impl std::fmt::Debug, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, ()}]`
2019-10-09T21:25:01.5055747Z -    = note: required because it appears within the type `std::future::GenFuture<[static generator@$DIR/async-fn-nonsend.rs:21:39: 26:2 {impl std::fmt::Debug, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, ()}]>`
2019-10-09T21:25:01.5056489Z +    = note: required because it appears within the type `{impl std::fmt::Debug, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}`
2019-10-09T21:25:01.5057358Z +    = note: required because it appears within the type `[static generator@$DIR/async-fn-nonsend.rs:21:39: 26:2 {impl std::fmt::Debug, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]`
2019-10-09T21:25:01.5058446Z +    = note: required because it appears within the type `std::future::GenFuture<[static generator@$DIR/async-fn-nonsend.rs:21:39: 26:2 {impl std::fmt::Debug, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]>`
2019-10-09T21:25:01.5058718Z 15    = note: required because it appears within the type `impl std::future::Future`
2019-10-09T21:25:01.5058859Z 16    = note: required because it appears within the type `impl std::future::Future`
2019-10-09T21:25:01.5059115Z 
2019-10-09T21:25:01.5059239Z 26    |
2019-10-09T21:25:01.5059239Z 26    |
2019-10-09T21:25:01.5059371Z 27    = help: within `impl std::future::Future`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<()>`
2019-10-09T21:25:01.5059539Z 28    = note: required because it appears within the type `impl std::fmt::Debug`
2019-10-09T21:25:01.5060760Z -    = note: required because it appears within the type `{fn(impl std::fmt::Debug) -> std::option::Option<impl std::fmt::Debug> {std::option::Option::<impl std::fmt::Debug>::Some}, fn() -> impl std::fmt::Debug {non_send}, impl std::fmt::Debug, std::option::Option<impl std::fmt::Debug>, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, ()}`
2019-10-09T21:25:01.5061664Z -    = note: required because it appears within the type `[static generator@$DIR/async-fn-nonsend.rs:28:40: 37:2 {fn(impl std::fmt::Debug) -> std::option::Option<impl std::fmt::Debug> {std::option::Option::<impl std::fmt::Debug>::Some}, fn() -> impl std::fmt::Debug {non_send}, impl std::fmt::Debug, std::option::Option<impl std::fmt::Debug>, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, ()}]`
2019-10-09T21:25:01.5062749Z -    = note: required because it appears within the type `std::future::GenFuture<[static generator@$DIR/async-fn-nonsend.rs:28:40: 37:2 {fn(impl std::fmt::Debug) -> std::option::Option<impl std::fmt::Debug> {std::option::Option::<impl std::fmt::Debug>::Some}, fn() -> impl std::fmt::Debug {non_send}, impl std::fmt::Debug, std::option::Option<impl std::fmt::Debug>, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, ()}]>`
2019-10-09T21:25:01.5064109Z +    = note: required because it appears within the type `{fn(impl std::fmt::Debug) -> std::option::Option<impl std::fmt::Debug> {std::option::Option::<impl std::fmt::Debug>::Some}, fn() -> impl std::fmt::Debug {non_send}, impl std::fmt::Debug, std::option::Option<impl std::fmt::Debug>, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}`
2019-10-09T21:25:01.5064988Z +    = note: required because it appears within the type `[static generator@$DIR/async-fn-nonsend.rs:28:40: 37:2 {fn(impl std::fmt::Debug) -> std::option::Option<impl std::fmt::Debug> {std::option::Option::<impl std::fmt::Debug>::Some}, fn() -> impl std::fmt::Debug {non_send}, impl std::fmt::Debug, std::option::Option<impl std::fmt::Debug>, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]`
2019-10-09T21:25:01.5065963Z +    = note: required because it appears within the type `std::future::GenFuture<[static generator@$DIR/async-fn-nonsend.rs:28:40: 37:2 {fn(impl std::fmt::Debug) -> std::option::Option<impl std::fmt::Debug> {std::option::Option::<impl std::fmt::Debug>::Some}, fn() -> impl std::fmt::Debug {non_send}, impl std::fmt::Debug, std::option::Option<impl std::fmt::Debug>, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]>`
2019-10-09T21:25:01.5066232Z 32    = note: required because it appears within the type `impl std::future::Future`
2019-10-09T21:25:01.5066369Z 33    = note: required because it appears within the type `impl std::future::Future`
2019-10-09T21:25:01.5066614Z 
2019-10-09T21:25:01.5066742Z 45    = note: required because of the requirements on the impl of `std::marker::Send` for `&mut dyn std::fmt::Write`
2019-10-09T21:25:01.5067116Z 46    = note: required because it appears within the type `std::fmt::Formatter<'_>`
2019-10-09T21:25:01.5067556Z 47    = note: required because of the requirements on the impl of `std::marker::Send` for `&mut std::fmt::Formatter<'_>`
2019-10-09T21:25:01.5067556Z 47    = note: required because of the requirements on the impl of `std::marker::Send` for `&mut std::fmt::Formatter<'_>`
2019-10-09T21:25:01.5068475Z -    = note: required because it appears within the type `for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, ()}`
2019-10-09T21:25:01.5069072Z -    = note: required because it appears within the type `[static generator@$DIR/async-fn-nonsend.rs:39:38: 45:2 for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, ()}]`
2019-10-09T21:25:01.5069679Z -    = note: required because it appears within the type `std::future::GenFuture<[static generator@$DIR/async-fn-nonsend.rs:39:38: 45:2 for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, ()}]>`
2019-10-09T21:25:01.5070887Z +    = note: required because it appears within the type `for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}`
2019-10-09T21:25:01.5074386Z +    = note: required because it appears within the type `[static generator@$DIR/async-fn-nonsend.rs:39:38: 45:2 for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]`
2019-10-09T21:25:01.5075504Z +    = note: required because it appears within the type `std::future::GenFuture<[static generator@$DIR/async-fn-nonsend.rs:39:38: 45:2 for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]>`
2019-10-09T21:25:01.5075609Z 51    = note: required because it appears within the type `impl std::future::Future`
2019-10-09T21:25:01.5075657Z 52    = note: required because it appears within the type `impl std::future::Future`
2019-10-09T21:25:01.5075855Z 
2019-10-09T21:25:01.5075855Z 
2019-10-09T21:25:01.5076293Z 68    = note: required because of the requirements on the impl of `std::marker::Send` for `std::slice::Iter<'_, std::fmt::ArgumentV1<'_>>`
2019-10-09T21:25:01.5076521Z 69    = note: required because it appears within the type `std::fmt::Formatter<'_>`
2019-10-09T21:25:01.5076793Z 70    = note: required because of the requirements on the impl of `std::marker::Send` for `&mut std::fmt::Formatter<'_>`
2019-10-09T21:25:01.5077242Z -    = note: required because it appears within the type `for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, ()}`
2019-10-09T21:25:01.5077689Z -    = note: required because it appears within the type `[static generator@$DIR/async-fn-nonsend.rs:39:38: 45:2 for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, ()}]`
2019-10-09T21:25:01.5078086Z -    = note: required because it appears within the type `std::future::GenFuture<[static generator@$DIR/async-fn-nonsend.rs:39:38: 45:2 for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, ()}]>`
2019-10-09T21:25:01.5078592Z +    = note: required because it appears within the type `for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}`
2019-10-09T21:25:01.5079146Z +    = note: required because it appears within the type `[static generator@$DIR/async-fn-nonsend.rs:39:38: 45:2 for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]`
2019-10-09T21:25:01.5080280Z +    = note: required because it appears within the type `std::future::GenFuture<[static generator@$DIR/async-fn-nonsend.rs:39:38: 45:2 for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]>`
2019-10-09T21:25:01.5080524Z 74    = note: required because it appears within the type `impl std::future::Future`
2019-10-09T21:25:01.5080579Z 75    = note: required because it appears within the type `impl std::future::Future`
2019-10-09T21:25:01.5080673Z 
2019-10-09T21:25:01.5080699Z 
2019-10-09T21:25:01.5080744Z The actual stderr differed from the expected stderr.
2019-10-09T21:25:01.5081142Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-nonsend/async-fn-nonsend.stderr
2019-10-09T21:25:01.5081142Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-nonsend/async-fn-nonsend.stderr
2019-10-09T21:25:01.5081407Z To update references, rerun the tests and pass the `--bless` flag
2019-10-09T21:25:01.5081701Z To only update this specific test, also pass `--test-args async-await/async-fn-nonsend.rs`
2019-10-09T21:25:01.5081793Z error: 1 errors occurred comparing output.
2019-10-09T21:25:01.5081839Z status: exit code: 1
2019-10-09T21:25:01.5081839Z status: exit code: 1
2019-10-09T21:25:01.5082644Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-fn-nonsend.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-nonsend" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--crate-type" "lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-nonsend/auxiliary" "-A" "unused"
2019-10-09T21:25:01.5082984Z ------------------------------------------
2019-10-09T21:25:01.5083018Z 
2019-10-09T21:25:01.5083238Z ------------------------------------------
2019-10-09T21:25:01.5083310Z stderr:
2019-10-09T21:25:01.5083310Z stderr:
2019-10-09T21:25:01.5083652Z ------------------------------------------
2019-10-09T21:25:01.5083778Z error[E0277]: `std::rc::Rc<()>` cannot be sent between threads safely
2019-10-09T21:25:01.5084090Z    |
2019-10-09T21:25:01.5084090Z    |
2019-10-09T21:25:01.5084126Z LL | fn assert_send(_: impl Send) {}
2019-10-09T21:25:01.5084387Z ...
2019-10-09T21:25:01.5084387Z ...
2019-10-09T21:25:01.5084424Z LL |     assert_send(local_dropped_before_await());
2019-10-09T21:25:01.5084466Z    |     ^^^^^^^^^^^ `std::rc::Rc<()>` cannot be sent between threads safely
2019-10-09T21:25:01.5084521Z    |
2019-10-09T21:25:01.5084565Z    = help: within `impl std::future::Future`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<()>`
2019-10-09T21:25:01.5084629Z    = note: required because it appears within the type `impl std::fmt::Debug`
2019-10-09T21:25:01.5085077Z    = note: required because it appears within the type `{impl std::fmt::Debug, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}`
2019-10-09T21:25:01.5085615Z    = note: required because it appears within the type `[static generator@/checkout/src/test/ui/async-await/async-fn-nonsend.rs:21:39: 26:2 {impl std::fmt::Debug, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]`
2019-10-09T21:25:01.5086162Z    = note: required because it appears within the type `std::future::GenFuture<[static generator@/checkout/src/test/ui/async-await/async-fn-nonsend.rs:21:39: 26:2 {impl std::fmt::Debug, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]>`
2019-10-09T21:25:01.5086512Z    = note: required because it appears within the type `impl std::future::Future`
2019-10-09T21:25:01.5086558Z    = note: required because it appears within the type `impl std::future::Future`
2019-10-09T21:25:01.5086605Z 
2019-10-09T21:25:01.5086645Z error[E0277]: `std::rc::Rc<()>` cannot be sent between threads safely
2019-10-09T21:25:01.5086942Z    |
2019-10-09T21:25:01.5086942Z    |
2019-10-09T21:25:01.5091809Z LL | fn assert_send(_: impl Send) {}
2019-10-09T21:25:01.5092327Z ...
2019-10-09T21:25:01.5092327Z ...
2019-10-09T21:25:01.5092375Z LL |     assert_send(non_send_temporary_in_match());
2019-10-09T21:25:01.5092427Z    |     ^^^^^^^^^^^ `std::rc::Rc<()>` cannot be sent between threads safely
2019-10-09T21:25:01.5092482Z    |
2019-10-09T21:25:01.5092555Z    = help: within `impl std::future::Future`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<()>`
2019-10-09T21:25:01.5092619Z    = note: required because it appears within the type `impl std::fmt::Debug`
2019-10-09T21:25:01.5093381Z    = note: required because it appears within the type `{fn(impl std::fmt::Debug) -> std::option::Option<impl std::fmt::Debug> {std::option::Option::<impl std::fmt::Debug>::Some}, fn() -> impl std::fmt::Debug {non_send}, impl std::fmt::Debug, std::option::Option<impl std::fmt::Debug>, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}`
2019-10-09T21:25:01.5094690Z    = note: required because it appears within the type `[static generator@/checkout/src/test/ui/async-await/async-fn-nonsend.rs:28:40: 37:2 {fn(impl std::fmt::Debug) -> std::option::Option<impl std::fmt::Debug> {std::option::Option::<impl std::fmt::Debug>::Some}, fn() -> impl std::fmt::Debug {non_send}, impl std::fmt::Debug, std::option::Option<impl std::fmt::Debug>, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]`
2019-10-09T21:25:01.5095778Z    = note: required because it appears within the type `std::future::GenFuture<[static generator@/checkout/src/test/ui/async-await/async-fn-nonsend.rs:28:40: 37:2 {fn(impl std::fmt::Debug) -> std::option::Option<impl std::fmt::Debug> {std::option::Option::<impl std::fmt::Debug>::Some}, fn() -> impl std::fmt::Debug {non_send}, impl std::fmt::Debug, std::option::Option<impl std::fmt::Debug>, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]>`
2019-10-09T21:25:01.5095899Z    = note: required because it appears within the type `impl std::future::Future`
2019-10-09T21:25:01.5095963Z    = note: required because it appears within the type `impl std::future::Future`
2019-10-09T21:25:01.5095997Z 
2019-10-09T21:25:01.5096036Z error[E0277]: `dyn std::fmt::Write` cannot be sent between threads safely
2019-10-09T21:25:01.5096321Z    |
2019-10-09T21:25:01.5096321Z    |
2019-10-09T21:25:01.5096359Z LL | fn assert_send(_: impl Send) {}
2019-10-09T21:25:01.5096781Z ...
2019-10-09T21:25:01.5096781Z ...
2019-10-09T21:25:01.5096819Z LL |     assert_send(non_sync_with_method_call());
2019-10-09T21:25:01.5096874Z    |     ^^^^^^^^^^^ `dyn std::fmt::Write` cannot be sent between threads safely
2019-10-09T21:25:01.5096974Z    = help: the trait `std::marker::Send` is not implemented for `dyn std::fmt::Write`
2019-10-09T21:25:01.5097026Z    = note: required because of the requirements on the impl of `std::marker::Send` for `&mut dyn std::fmt::Write`
2019-10-09T21:25:01.5097441Z    = note: required because it appears within the type `std::fmt::Formatter<'_>`
2019-10-09T21:25:01.5097868Z    = note: required because of the requirements on the impl of `std::marker::Send` for `&mut std::fmt::Formatter<'_>`
2019-10-09T21:25:01.5097868Z    = note: required because of the requirements on the impl of `std::marker::Send` for `&mut std::fmt::Formatter<'_>`
2019-10-09T21:25:01.5098510Z    = note: required because it appears within the type `for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}`
2019-10-09T21:25:01.5100856Z    = note: required because it appears within the type `[static generator@/checkout/src/test/ui/async-await/async-fn-nonsend.rs:39:38: 45:2 for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]`
2019-10-09T21:25:01.5101938Z    = note: required because it appears within the type `std::future::GenFuture<[static generator@/checkout/src/test/ui/async-await/async-fn-nonsend.rs:39:38: 45:2 for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]>`
2019-10-09T21:25:01.5102086Z    = note: required because it appears within the type `impl std::future::Future`
2019-10-09T21:25:01.5102141Z    = note: required because it appears within the type `impl std::future::Future`
2019-10-09T21:25:01.5102181Z 
2019-10-09T21:25:01.5102523Z error[E0277]: `*mut (dyn std::ops::Fn() + 'static)` cannot be shared between threads safely
2019-10-09T21:25:01.5102831Z    |
2019-10-09T21:25:01.5102831Z    |
2019-10-09T21:25:01.5102894Z LL | fn assert_send(_: impl Send) {}
2019-10-09T21:25:01.5103190Z ...
2019-10-09T21:25:01.5103190Z ...
2019-10-09T21:25:01.5103263Z LL |     assert_send(non_sync_with_method_call());
2019-10-09T21:25:01.5103701Z    |     ^^^^^^^^^^^ `*mut (dyn std::ops::Fn() + 'static)` cannot be shared between threads safely
2019-10-09T21:25:01.5103744Z    |
2019-10-09T21:25:01.5104033Z    = help: within `std::fmt::ArgumentV1<'_>`, the trait `std::marker::Sync` is not implemented for `*mut (dyn std::ops::Fn() + 'static)`
2019-10-09T21:25:01.5104293Z    = note: required because it appears within the type `std::marker::PhantomData<*mut (dyn std::ops::Fn() + 'static)>`
2019-10-09T21:25:01.5104345Z    = note: required because it appears within the type `core::fmt::Void`
2019-10-09T21:25:01.5104408Z    = note: required because it appears within the type `&core::fmt::Void`
2019-10-09T21:25:01.5104786Z    = note: required because it appears within the type `std::fmt::ArgumentV1<'_>`
2019-10-09T21:25:01.5105041Z    = note: required because of the requirements on the impl of `std::marker::Send` for `std::slice::Iter<'_, std::fmt::ArgumentV1<'_>>`
2019-10-09T21:25:01.5105409Z    = note: required because it appears within the type `std::fmt::Formatter<'_>`
2019-10-09T21:25:01.5105663Z    = note: required because of the requirements on the impl of `std::marker::Send` for `&mut std::fmt::Formatter<'_>`
2019-10-09T21:25:01.5106140Z    = note: required because it appears within the type `for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}`
2019-10-09T21:25:01.5106690Z    = note: required because it appears within the type `[static generator@/checkout/src/test/ui/async-await/async-fn-nonsend.rs:39:38: 45:2 for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]`
2019-10-09T21:25:01.5107298Z    = note: required because it appears within the type `std::future::GenFuture<[static generator@/checkout/src/test/ui/async-await/async-fn-nonsend.rs:39:38: 45:2 for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]>`
2019-10-09T21:25:01.5107393Z    = note: required because it appears within the type `impl std::future::Future`
2019-10-09T21:25:01.5107438Z    = note: required because it appears within the type `impl std::future::Future`
2019-10-09T21:25:01.5107520Z error: aborting due to 4 previous errors
2019-10-09T21:25:01.5107550Z 
2019-10-09T21:25:01.5107757Z For more information about this error, try `rustc --explain E0277`.
2019-10-09T21:25:01.5107786Z 
---
2019-10-09T21:25:01.5110862Z ---- [ui] ui/async-await/await-into-future.rs stdout ----
2019-10-09T21:25:01.5110895Z 
2019-10-09T21:25:01.5111126Z error: test compilation failed although it shouldn't!
2019-10-09T21:25:01.5111176Z status: exit code: 1
2019-10-09T21:25:01.5111982Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/await-into-future.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/await-into-future" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/await-into-future/auxiliary" "-A" "unused"
2019-10-09T21:25:01.5112475Z ------------------------------------------
2019-10-09T21:25:01.5112509Z 
2019-10-09T21:25:01.5112730Z ------------------------------------------
2019-10-09T21:25:01.5112793Z stderr:
2019-10-09T21:25:01.5112793Z stderr:
2019-10-09T21:25:01.5113009Z ------------------------------------------
2019-10-09T21:25:01.5113250Z error[E0658]: use of unstable library feature 'into_future'
2019-10-09T21:25:01.5113523Z   --> /checkout/src/test/ui/async-await/await-into-future.rs:5:28
2019-10-09T21:25:01.5113573Z    |
2019-10-09T21:25:01.5113777Z LL | use std::{future::{Future, IntoFuture}, pin::Pin};
2019-10-09T21:25:01.5113880Z    |
2019-10-09T21:25:01.5113880Z    |
2019-10-09T21:25:01.5113922Z    = help: add `#![feature(into_future)]` to the crate attributes to enable
2019-10-09T21:25:01.5114187Z error[E0658]: use of unstable library feature 'into_future'
2019-10-09T21:25:01.5114564Z   --> /checkout/src/test/ui/async-await/await-into-future.rs:10:5
2019-10-09T21:25:01.5114605Z    |
2019-10-09T21:25:01.5114605Z    |
2019-10-09T21:25:01.5114662Z LL |     type Future = Pin<Box<dyn Future<Output = i32>>>;
2019-10-09T21:25:01.5114738Z    |
2019-10-09T21:25:01.5114738Z    |
2019-10-09T21:25:01.5114795Z    = help: add `#![feature(into_future)]` to the crate attributes to enable
2019-10-09T21:25:01.5115025Z error[E0658]: use of unstable library feature 'into_future'
2019-10-09T21:25:01.5115408Z   --> /checkout/src/test/ui/async-await/await-into-future.rs:12:5
2019-10-09T21:25:01.5115465Z    |
2019-10-09T21:25:01.5115808Z LL | /     fn into_future(self) -> Self::Future {
2019-10-09T21:25:01.5115808Z LL | /     fn into_future(self) -> Self::Future {
2019-10-09T21:25:01.5115854Z LL | |         Box::pin(me())
2019-10-09T21:25:01.5115939Z    | |_____^
2019-10-09T21:25:01.5115970Z    |
2019-10-09T21:25:01.5115970Z    |
2019-10-09T21:25:01.5116104Z    = help: add `#![feature(into_future)]` to the crate attributes to enable
2019-10-09T21:25:01.5116356Z error[E0658]: use of unstable library feature 'into_future'
2019-10-09T21:25:01.5116552Z   --> /checkout/src/test/ui/async-await/await-into-future.rs:9:6
2019-10-09T21:25:01.5116607Z    |
2019-10-09T21:25:01.5116607Z    |
2019-10-09T21:25:01.5116641Z LL | impl IntoFuture for AwaitMe {
2019-10-09T21:25:01.5116724Z    |
2019-10-09T21:25:01.5116724Z    |
2019-10-09T21:25:01.5116762Z    = help: add `#![feature(into_future)]` to the crate attributes to enable
2019-10-09T21:25:01.5116975Z error[E0658]: use of unstable library feature 'into_future'
2019-10-09T21:25:01.5117187Z   --> /checkout/src/test/ui/async-await/await-into-future.rs:12:29
2019-10-09T21:25:01.5117232Z    |
2019-10-09T21:25:01.5117409Z LL |     fn into_future(self) -> Self::Future {
2019-10-09T21:25:01.5117409Z LL |     fn into_future(self) -> Self::Future {
2019-10-09T21:25:01.5117466Z    |                             ^^^^^^^^^^^^
2019-10-09T21:25:01.5117501Z    |
2019-10-09T21:25:01.5117546Z    = help: add `#![feature(into_future)]` to the crate attributes to enable
2019-10-09T21:25:01.5117621Z error: aborting due to 5 previous errors
2019-10-09T21:25:01.5117645Z 
2019-10-09T21:25:01.5117839Z For more information about this error, try `rustc --explain E0658`.
2019-10-09T21:25:01.5117866Z 
2019-10-09T21:25:01.5117866Z 
2019-10-09T21:25:01.5118057Z ------------------------------------------
2019-10-09T21:25:01.5118082Z 
2019-10-09T21:25:01.5118102Z 
2019-10-09T21:25:01.5118280Z ---- [ui] ui/async-await/async-fn-size.rs stdout ----
2019-10-09T21:25:01.5118323Z 
2019-10-09T21:25:01.5118500Z error: test compilation failed although it shouldn't!
2019-10-09T21:25:01.5118537Z status: exit code: 1
2019-10-09T21:25:01.5119134Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-fn-size.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-size/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-size/auxiliary"
2019-10-09T21:25:01.5119509Z ------------------------------------------
2019-10-09T21:25:01.5119537Z 
2019-10-09T21:25:01.5119711Z ------------------------------------------
2019-10-09T21:25:01.5119747Z stderr:
2019-10-09T21:25:01.5119747Z stderr:
2019-10-09T21:25:01.5119934Z ------------------------------------------
2019-10-09T21:25:01.5120849Z error: reached the type-length limit while instantiating `<std::boxed::Box<std::future::Ge...>, ()}]>, ()}]>, ()}]>>>>>::into`
2019-10-09T21:25:01.5121189Z    |
2019-10-09T21:25:01.5121434Z LL | /     fn into(self) -> U {
2019-10-09T21:25:01.5121485Z LL | |         U::from(self)
2019-10-09T21:25:01.5121559Z LL | |     }
2019-10-09T21:25:01.5121559Z LL | |     }
2019-10-09T21:25:01.5121602Z    | |_____^
2019-10-09T21:25:01.5121645Z    |
2019-10-09T21:25:01.5121715Z    = note: consider adding a `#![type_length_limit="1266781"]` attribute to your crate
2019-10-09T21:25:01.5121798Z error: aborting due to previous error
2019-10-09T21:25:01.5121890Z 
2019-10-09T21:25:01.5121919Z 
2019-10-09T21:25:01.5122186Z ------------------------------------------
2019-10-09T21:25:01.5122186Z ------------------------------------------
2019-10-09T21:25:01.5122220Z 
2019-10-09T21:25:01.5122247Z 
2019-10-09T21:25:01.5122504Z ---- [ui] ui/async-await/issues/issue-62009-1.rs stdout ----
2019-10-09T21:25:01.5122575Z diff of stderr:
2019-10-09T21:25:01.5122604Z 
2019-10-09T21:25:01.5122645Z 32    |
2019-10-09T21:25:01.5122706Z 33 LL |     (|_| 2333).await;
2019-10-09T21:25:01.5123048Z 34    |     ^^^^^^^^^^^^^^^^ the trait `std::future::Future` is not implemented for `[closure@$DIR/issue-62009-1.rs:14:5: 14:15]`
2019-10-09T21:25:01.5123258Z -    | 
2019-10-09T21:25:01.5123604Z -   ::: $SRC_DIR/libstd/future.rs:LL:COL
2019-10-09T21:25:01.5124058Z - LL |     F: Future
2019-10-09T21:25:01.5124058Z - LL |     F: Future
2019-10-09T21:25:01.5124285Z -    |        ------ required by this bound in `std::future::poll_with_tls_context`
2019-10-09T21:25:01.5124355Z +    = note: required by `std::future::IntoFuture::into_future`
2019-10-09T21:25:01.5124434Z 41 error: aborting due to 4 previous errors
2019-10-09T21:25:01.5124487Z 42 
2019-10-09T21:25:01.5124511Z 
2019-10-09T21:25:01.5124533Z 
2019-10-09T21:25:01.5124533Z 
2019-10-09T21:25:01.5124572Z The actual stderr differed from the expected stderr.
2019-10-09T21:25:01.5124864Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62009-1/issue-62009-1.stderr
2019-10-09T21:25:01.5125243Z To update references, rerun the tests and pass the `--bless` flag
2019-10-09T21:25:01.5125501Z To only update this specific test, also pass `--test-args async-await/issues/issue-62009-1.rs`
2019-10-09T21:25:01.5125597Z error: 1 errors occurred comparing output.
2019-10-09T21:25:01.5125637Z status: exit code: 1
2019-10-09T21:25:01.5125637Z status: exit code: 1
2019-10-09T21:25:01.5126624Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-62009-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62009-1" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62009-1/auxiliary" "-A" "unused"
2019-10-09T21:25:01.5127006Z ------------------------------------------
2019-10-09T21:25:01.5127035Z 
2019-10-09T21:25:01.5127211Z ------------------------------------------
2019-10-09T21:25:01.5127247Z stderr:
2019-10-09T21:25:01.5127247Z stderr:
2019-10-09T21:25:01.5127489Z ------------------------------------------
2019-10-09T21:25:01.5127535Z error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-10-09T21:25:01.5127744Z   --> /checkout/src/test/ui/async-await/issues/issue-62009-1.rs:8:5
2019-10-09T21:25:01.5127836Z LL | fn main() {
2019-10-09T21:25:01.5128004Z    |    ---- this is not `async`
2019-10-09T21:25:01.5128004Z    |    ---- this is not `async`
2019-10-09T21:25:01.5128061Z LL |     async { let (); }.await;
2019-10-09T21:25:01.5128101Z    |     ^^^^^^^^^^^^^^^^^^^^^^^ only allowed inside `async` functions and blocks
2019-10-09T21:25:01.5128127Z 
2019-10-09T21:25:01.5128163Z error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-10-09T21:25:01.5128381Z   --> /checkout/src/test/ui/async-await/issues/issue-62009-1.rs:10:5
2019-10-09T21:25:01.5128460Z LL |   fn main() {
2019-10-09T21:25:01.5128644Z    |      ---- this is not `async`
2019-10-09T21:25:01.5128680Z ...
2019-10-09T21:25:01.5128712Z LL | /     async {
2019-10-09T21:25:01.5128712Z LL | /     async {
2019-10-09T21:25:01.5128775Z LL | |     //~^ ERROR `await` is only allowed inside `async` functions and blocks
2019-10-09T21:25:01.5128817Z LL | |         let task1 = print_dur().await;
2019-10-09T21:25:01.5128852Z LL | |     }.await;
2019-10-09T21:25:01.5128891Z    | |___________^ only allowed inside `async` functions and blocks
2019-10-09T21:25:01.5128936Z 
2019-10-09T21:25:01.5128973Z error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-10-09T21:25:01.5129177Z   --> /checkout/src/test/ui/async-await/issues/issue-62009-1.rs:14:5
2019-10-09T21:25:01.5129265Z LL | fn main() {
2019-10-09T21:25:01.5129430Z    |    ---- this is not `async`
2019-10-09T21:25:01.5129483Z ...
2019-10-09T21:25:01.5129483Z ...
2019-10-09T21:25:01.5129517Z LL |     (|_| 2333).await;
2019-10-09T21:25:01.5129562Z    |     ^^^^^^^^^^^^^^^^ only allowed inside `async` functions and blocks
2019-10-09T21:25:01.5129587Z 
2019-10-09T21:25:01.5129955Z error[E0277]: the trait bound `[closure@/checkout/src/test/ui/async-await/issues/issue-62009-1.rs:14:5: 14:15]: std::future::Future` is not satisfied
2019-10-09T21:25:01.5130725Z   --> /checkout/src/test/ui/async-await/issues/issue-62009-1.rs:14:5
2019-10-09T21:25:01.5130784Z    |
2019-10-09T21:25:01.5130849Z LL |     (|_| 2333).await;
2019-10-09T21:25:01.5131193Z    |     ^^^^^^^^^^^^^^^^ the trait `std::future::Future` is not implemented for `[closure@/checkout/src/test/ui/async-await/issues/issue-62009-1.rs:14:5: 14:15]`
2019-10-09T21:25:01.5131249Z    |
2019-10-09T21:25:01.5131312Z    = note: required by `std::future::IntoFuture::into_future`
2019-10-09T21:25:01.5131387Z error: aborting due to 4 previous errors
2019-10-09T21:25:01.5131415Z 
2019-10-09T21:25:01.5131675Z For more information about this error, try `rustc --explain E0277`.
2019-10-09T21:25:01.5131722Z 
---
2019-10-09T21:25:02.2982888Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-09T21:25:02.2982971Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-09T21:25:02.2983465Z 
2019-10-09T21:25:02.2983492Z 
2019-10-09T21:25:02.2986010Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-09T21:25:02.2986361Z 
2019-10-09T21:25:02.2986394Z 
2019-10-09T21:25:02.2986442Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-09T21:25:02.2986514Z Build completed unsuccessfully in 1:05:03
2019-10-09T21:25:02.2986514Z Build completed unsuccessfully in 1:05:03
2019-10-09T21:25:02.2986561Z == clock drift check ==
2019-10-09T21:25:02.2986608Z   local time: Wed Oct  9 21:25:01 UTC 2019
2019-10-09T21:25:02.4144582Z   network time: Wed, 09 Oct 2019 21:25:02 GMT
2019-10-09T21:25:02.4145822Z == end clock drift check ==
2019-10-09T21:25:02.9338156Z ##[error]Bash exited with code '1'.
2019-10-09T21:25:02.9386970Z ##[section]Starting: Checkout
2019-10-09T21:25:02.9388613Z ==============================================================================
2019-10-09T21:25:02.9388658Z Task         : Get sources
2019-10-09T21:25:02.9388712Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
