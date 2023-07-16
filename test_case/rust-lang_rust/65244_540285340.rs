plain
2019-10-10T00:22:59.2619929Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-10T00:22:59.2634126Z ##[command]git config gc.auto 0
2019-10-10T00:22:59.2639204Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-10T00:22:59.2644089Z ##[command]git config --get-all http.proxy
2019-10-10T00:22:59.2648013Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65244/merge:refs/remotes/pull/65244/merge
---
2019-10-10T01:25:28.0618552Z .................................................................................................... 1600/9143
2019-10-10T01:25:35.1291987Z .................................................................................................... 1700/9143
2019-10-10T01:25:47.3193787Z ..................i...............i................................................................. 1800/9143
2019-10-10T01:25:54.6559553Z .................................................................................................... 1900/9143
2019-10-10T01:26:10.1436274Z .........iiiii...................................................................................... 2000/9143
2019-10-10T01:26:20.3332554Z .................................................................................................... 2200/9143
2019-10-10T01:26:22.9361324Z .................................................................................................... 2300/9143
2019-10-10T01:26:28.8331409Z .................................................................................................... 2400/9143
2019-10-10T01:26:35.2038676Z .................................................................................................... 2500/9143
---
2019-10-10T01:29:33.2294487Z .................................................................................................... 4700/9143
2019-10-10T01:29:40.4493073Z ..i...............i................................................................................. 4800/9143
2019-10-10T01:29:52.1400855Z .................................................................................................... 4900/9143
2019-10-10T01:29:57.8339048Z .................................................................................................... 5000/9143
2019-10-10T01:30:09.4171651Z ................................................................................................ii.i 5100/9143
2019-10-10T01:30:14.7691634Z i................................................................................................... 5200/9143
2019-10-10T01:30:30.3035003Z .................................................................................................... 5400/9143
2019-10-10T01:30:37.1336818Z ..............................................................i..................................... 5500/9143
2019-10-10T01:30:44.7522359Z .................................................................................................... 5600/9143
2019-10-10T01:30:52.3116159Z .................................................................................................... 5700/9143
2019-10-10T01:30:52.3116159Z .................................................................................................... 5700/9143
2019-10-10T01:31:02.9809092Z ...........................................................ii...i..ii...........i................... 5800/9143
2019-10-10T01:31:29.4605340Z .................................................................................................... 6000/9143
2019-10-10T01:31:38.9191787Z .................................................................................................... 6100/9143
2019-10-10T01:31:38.9191787Z .................................................................................................... 6100/9143
2019-10-10T01:31:45.8390978Z .................................................................i..ii.............................. 6200/9143
2019-10-10T01:32:15.8913954Z .................................................................................................... 6400/9143
2019-10-10T01:32:17.9933957Z .........................i.......................................................................... 6500/9143
2019-10-10T01:32:20.2326238Z ..................................................................................................i. 6600/9143
2019-10-10T01:32:22.9709458Z .................................................................................................... 6700/9143
---
2019-10-10T01:36:41.7756860Z 
2019-10-10T01:36:41.7757695Z ---- [ui] ui/async-await/async-fn-nonsend.rs stdout ----
2019-10-10T01:36:41.7757950Z diff of stderr:
2019-10-10T01:36:41.7758438Z 
2019-10-10T01:36:41.7758623Z 1 error[E0277]: `std::rc::Rc<()>` cannot be sent between threads safely
2019-10-10T01:36:41.7759092Z +   --> $DIR/async-fn-nonsend.rs:50:5
2019-10-10T01:36:41.7759808Z -    |
2019-10-10T01:36:41.7759808Z -    |
2019-10-10T01:36:41.7760017Z 4 LL | fn assert_send(_: impl Send) {}
2019-10-10T01:36:41.7760649Z 5 ...
2019-10-10T01:36:41.7761038Z - ...
2019-10-10T01:36:41.7761038Z - ...
2019-10-10T01:36:41.7761245Z 7 LL |     assert_send(local_dropped_before_await());
2019-10-10T01:36:41.7761443Z 8    |     ^^^^^^^^^^^ `std::rc::Rc<()>` cannot be sent between threads safely
2019-10-10T01:36:41.7761737Z 
2019-10-10T01:36:41.7761737Z 
2019-10-10T01:36:41.7761927Z 10    = help: within `impl std::future::Future`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<()>`
2019-10-10T01:36:41.7762136Z 11    = note: required because it appears within the type `impl std::fmt::Debug`
2019-10-10T01:36:41.7762930Z 12    = note: required because it appears within the type `{impl std::fmt::Debug, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}`
2019-10-10T01:36:41.7763808Z -    = note: required because it appears within the type `[static generator@/checkout/src/test/ui/async-await/async-fn-nonsend.rs:21:39: 26:2 {impl std::fmt::Debug, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]`
2019-10-10T01:36:41.7765105Z -    = note: required because it appears within the type `std::future::GenFuture<[static generator@/checkout/src/test/ui/async-await/async-fn-nonsend.rs:21:39: 26:2 {impl std::fmt::Debug, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]>`
2019-10-10T01:36:41.7766115Z +    = note: required because it appears within the type `[static generator@$DIR/async-fn-nonsend.rs:21:39: 26:2 {impl std::fmt::Debug, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]`
2019-10-10T01:36:41.7767014Z +    = note: required because it appears within the type `std::future::GenFuture<[static generator@$DIR/async-fn-nonsend.rs:21:39: 26:2 {impl std::fmt::Debug, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]>`
2019-10-10T01:36:41.7767274Z 15    = note: required because it appears within the type `impl std::future::Future`
2019-10-10T01:36:41.7767476Z 16    = note: required because it appears within the type `impl std::future::Future`
2019-10-10T01:36:41.7767774Z 
2019-10-10T01:36:41.7767774Z 
2019-10-10T01:36:41.7767956Z 18 error[E0277]: `std::rc::Rc<()>` cannot be sent between threads safely
2019-10-10T01:36:41.7768369Z +   --> $DIR/async-fn-nonsend.rs:52:5
2019-10-10T01:36:41.7769101Z -    |
2019-10-10T01:36:41.7769101Z -    |
2019-10-10T01:36:41.7769298Z 21 LL | fn assert_send(_: impl Send) {}
2019-10-10T01:36:41.7769922Z 22 ...
2019-10-10T01:36:41.7770496Z - ...
2019-10-10T01:36:41.7770496Z - ...
2019-10-10T01:36:41.7770713Z 24 LL |     assert_send(non_send_temporary_in_match());
2019-10-10T01:36:41.7770876Z 25    |     ^^^^^^^^^^^ `std::rc::Rc<()>` cannot be sent between threads safely
2019-10-10T01:36:41.7771185Z 
2019-10-10T01:36:41.7771185Z 
2019-10-10T01:36:41.7771367Z 27    = help: within `impl std::future::Future`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<()>`
2019-10-10T01:36:41.7771559Z 28    = note: required because it appears within the type `impl std::fmt::Debug`
2019-10-10T01:36:41.7772563Z 29    = note: required because it appears within the type `{fn(impl std::fmt::Debug) -> std::option::Option<impl std::fmt::Debug> {std::option::Option::<impl std::fmt::Debug>::Some}, fn() -> impl std::fmt::Debug {non_send}, impl std::fmt::Debug, std::option::Option<impl std::fmt::Debug>, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}`
2019-10-10T01:36:41.7773858Z -    = note: required because it appears within the type `[static generator@/checkout/src/test/ui/async-await/async-fn-nonsend.rs:28:40: 37:2 {fn(impl std::fmt::Debug) -> std::option::Option<impl std::fmt::Debug> {std::option::Option::<impl std::fmt::Debug>::Some}, fn() -> impl std::fmt::Debug {non_send}, impl std::fmt::Debug, std::option::Option<impl std::fmt::Debug>, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]`
2019-10-10T01:36:41.7775807Z -    = note: required because it appears within the type `std::future::GenFuture<[static generator@/checkout/src/test/ui/async-await/async-fn-nonsend.rs:28:40: 37:2 {fn(impl std::fmt::Debug) -> std::option::Option<impl std::fmt::Debug> {std::option::Option::<impl std::fmt::Debug>::Some}, fn() -> impl std::fmt::Debug {non_send}, impl std::fmt::Debug, std::option::Option<impl std::fmt::Debug>, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]>`
2019-10-10T01:36:41.7777033Z +    = note: required because it appears within the type `[static generator@$DIR/async-fn-nonsend.rs:28:40: 37:2 {fn(impl std::fmt::Debug) -> std::option::Option<impl std::fmt::Debug> {std::option::Option::<impl std::fmt::Debug>::Some}, fn() -> impl std::fmt::Debug {non_send}, impl std::fmt::Debug, std::option::Option<impl std::fmt::Debug>, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]`
2019-10-10T01:36:41.7778140Z +    = note: required because it appears within the type `std::future::GenFuture<[static generator@$DIR/async-fn-nonsend.rs:28:40: 37:2 {fn(impl std::fmt::Debug) -> std::option::Option<impl std::fmt::Debug> {std::option::Option::<impl std::fmt::Debug>::Some}, fn() -> impl std::fmt::Debug {non_send}, impl std::fmt::Debug, std::option::Option<impl std::fmt::Debug>, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]>`
2019-10-10T01:36:41.7778427Z 32    = note: required because it appears within the type `impl std::future::Future`
2019-10-10T01:36:41.7778641Z 33    = note: required because it appears within the type `impl std::future::Future`
2019-10-10T01:36:41.7779120Z 
2019-10-10T01:36:41.7779120Z 
2019-10-10T01:36:41.7779292Z 35 error[E0277]: `dyn std::fmt::Write` cannot be sent between threads safely
2019-10-10T01:36:41.7779739Z +   --> $DIR/async-fn-nonsend.rs:54:5
2019-10-10T01:36:41.7782202Z -    |
2019-10-10T01:36:41.7782202Z -    |
2019-10-10T01:36:41.7782427Z 38 LL | fn assert_send(_: impl Send) {}
2019-10-10T01:36:41.7782775Z 39 ...
2019-10-10T01:36:41.7782973Z - ...
2019-10-10T01:36:41.7782973Z - ...
2019-10-10T01:36:41.7783042Z 41 LL |     assert_send(non_sync_with_method_call());
2019-10-10T01:36:41.7783098Z 42    |     ^^^^^^^^^^^ `dyn std::fmt::Write` cannot be sent between threads safely
2019-10-10T01:36:41.7783214Z 43    = help: the trait `std::marker::Send` is not implemented for `dyn std::fmt::Write`
2019-10-10T01:36:41.7783280Z 44    = note: required because of the requirements on the impl of `std::marker::Send` for `&mut dyn std::fmt::Write`
2019-10-10T01:36:41.7783618Z 45    = note: required because it appears within the type `std::fmt::Formatter<'_>`
2019-10-10T01:36:41.7783664Z 
2019-10-10T01:36:41.7783664Z 
2019-10-10T01:36:41.7784018Z 46    = note: required because of the requirements on the impl of `std::marker::Send` for `&mut std::fmt::Formatter<'_>`
2019-10-10T01:36:41.7784358Z -    = note: required because of the requirements on the impl of `std::marker::Send` for `&mut std::fmt::Formatter<'_>`
2019-10-10T01:36:41.7785200Z 48    = note: required because it appears within the type `for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}`
2019-10-10T01:36:41.7786094Z -    = note: required because it appears within the type `[static generator@/checkout/src/test/ui/async-await/async-fn-nonsend.rs:39:38: 45:2 for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]`
2019-10-10T01:36:41.7786893Z -    = note: required because it appears within the type `std::future::GenFuture<[static generator@/checkout/src/test/ui/async-await/async-fn-nonsend.rs:39:38: 45:2 for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]>`
2019-10-10T01:36:41.7787602Z +    = note: required because it appears within the type `[static generator@$DIR/async-fn-nonsend.rs:39:38: 45:2 for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]`
2019-10-10T01:36:41.7788475Z +    = note: required because it appears within the type `std::future::GenFuture<[static generator@$DIR/async-fn-nonsend.rs:39:38: 45:2 for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]>`
2019-10-10T01:36:41.7788581Z 51    = note: required because it appears within the type `impl std::future::Future`
2019-10-10T01:36:41.7788759Z 52    = note: required because it appears within the type `impl std::future::Future`
2019-10-10T01:36:41.7788858Z 
2019-10-10T01:36:41.7788858Z 
2019-10-10T01:36:41.7789173Z 54 error[E0277]: `*mut (dyn std::ops::Fn() + 'static)` cannot be shared between threads safely
2019-10-10T01:36:41.7789412Z +   --> $DIR/async-fn-nonsend.rs:54:5
2019-10-10T01:36:41.7789671Z -    |
2019-10-10T01:36:41.7789671Z -    |
2019-10-10T01:36:41.7789718Z 57 LL | fn assert_send(_: impl Send) {}
2019-10-10T01:36:41.7790034Z 58 ...
2019-10-10T01:36:41.7790223Z - ...
2019-10-10T01:36:41.7790223Z - ...
2019-10-10T01:36:41.7790272Z 60 LL |     assert_send(non_sync_with_method_call());
2019-10-10T01:36:41.7790571Z 61    |     ^^^^^^^^^^^ `*mut (dyn std::ops::Fn() + 'static)` cannot be shared between threads safely
2019-10-10T01:36:41.7790651Z 
2019-10-10T01:36:41.7790924Z 69    = note: required because it appears within the type `std::fmt::Formatter<'_>`
2019-10-10T01:36:41.7791254Z 70    = note: required because of the requirements on the impl of `std::marker::Send` for `&mut std::fmt::Formatter<'_>`
2019-10-10T01:36:41.7791254Z 70    = note: required because of the requirements on the impl of `std::marker::Send` for `&mut std::fmt::Formatter<'_>`
2019-10-10T01:36:41.7792245Z 71    = note: required because it appears within the type `for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}`
2019-10-10T01:36:41.7793032Z -    = note: required because it appears within the type `[static generator@/checkout/src/test/ui/async-await/async-fn-nonsend.rs:39:38: 45:2 for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]`
2019-10-10T01:36:41.7794135Z -    = note: required because it appears within the type `std::future::GenFuture<[static generator@/checkout/src/test/ui/async-await/async-fn-nonsend.rs:39:38: 45:2 for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]>`
2019-10-10T01:36:41.7794812Z +    = note: required because it appears within the type `[static generator@$DIR/async-fn-nonsend.rs:39:38: 45:2 for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]`
2019-10-10T01:36:41.7795899Z +    = note: required because it appears within the type `std::future::GenFuture<[static generator@$DIR/async-fn-nonsend.rs:39:38: 45:2 for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]>`
2019-10-10T01:36:41.7796012Z 74    = note: required because it appears within the type `impl std::future::Future`
2019-10-10T01:36:41.7796072Z 75    = note: required because it appears within the type `impl std::future::Future`
2019-10-10T01:36:41.7796189Z 76 error: aborting due to 4 previous errors
2019-10-10T01:36:41.7796358Z 77 
2019-10-10T01:36:41.7796692Z 78 For more information about this error, try `rustc --explain E0277`.
2019-10-10T01:36:41.7796732Z 
2019-10-10T01:36:41.7796732Z 
2019-10-10T01:36:41.7796760Z 
2019-10-10T01:36:41.7796808Z The actual stderr differed from the expected stderr.
2019-10-10T01:36:41.7797444Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-nonsend/async-fn-nonsend.stderr
2019-10-10T01:36:41.7797798Z To update references, rerun the tests and pass the `--bless` flag
2019-10-10T01:36:41.7798095Z To only update this specific test, also pass `--test-args async-await/async-fn-nonsend.rs`
2019-10-10T01:36:41.7798198Z error: 1 errors occurred comparing output.
2019-10-10T01:36:41.7798246Z status: exit code: 1
2019-10-10T01:36:41.7798246Z status: exit code: 1
2019-10-10T01:36:41.7799070Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-fn-nonsend.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-nonsend" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--crate-type" "lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-nonsend/auxiliary" "-A" "unused"
2019-10-10T01:36:41.7799438Z ------------------------------------------
2019-10-10T01:36:41.7799475Z 
2019-10-10T01:36:41.7799709Z ------------------------------------------
2019-10-10T01:36:41.7799772Z stderr:
2019-10-10T01:36:41.7799772Z stderr:
2019-10-10T01:36:41.7800005Z ------------------------------------------
2019-10-10T01:36:41.7800060Z error[E0277]: `std::rc::Rc<()>` cannot be sent between threads safely
2019-10-10T01:36:41.7800396Z    |
2019-10-10T01:36:41.7800396Z    |
2019-10-10T01:36:41.7800440Z LL | fn assert_send(_: impl Send) {}
2019-10-10T01:36:41.7800902Z ...
2019-10-10T01:36:41.7800902Z ...
2019-10-10T01:36:41.7800964Z LL |     assert_send(local_dropped_before_await());
2019-10-10T01:36:41.7801017Z    |     ^^^^^^^^^^^ `std::rc::Rc<()>` cannot be sent between threads safely
2019-10-10T01:36:41.7801083Z    |
2019-10-10T01:36:41.7801139Z    = help: within `impl std::future::Future`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<()>`
2019-10-10T01:36:41.7801203Z    = note: required because it appears within the type `impl std::fmt::Debug`
2019-10-10T01:36:41.7801816Z    = note: required because it appears within the type `{impl std::fmt::Debug, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}`
2019-10-10T01:36:41.7802489Z    = note: required because it appears within the type `[static generator@/checkout/src/test/ui/async-await/async-fn-nonsend.rs:21:39: 26:2 {impl std::fmt::Debug, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]`
2019-10-10T01:36:41.7803190Z    = note: required because it appears within the type `std::future::GenFuture<[static generator@/checkout/src/test/ui/async-await/async-fn-nonsend.rs:21:39: 26:2 {impl std::fmt::Debug, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]>`
2019-10-10T01:36:41.7803300Z    = note: required because it appears within the type `impl std::future::Future`
2019-10-10T01:36:41.7803467Z    = note: required because it appears within the type `impl std::future::Future`
2019-10-10T01:36:41.7803521Z 
2019-10-10T01:36:41.7803570Z error[E0277]: `std::rc::Rc<()>` cannot be sent between threads safely
2019-10-10T01:36:41.7803917Z    |
2019-10-10T01:36:41.7803917Z    |
2019-10-10T01:36:41.7803983Z LL | fn assert_send(_: impl Send) {}
2019-10-10T01:36:41.7804297Z ...
2019-10-10T01:36:41.7804297Z ...
2019-10-10T01:36:41.7804361Z LL |     assert_send(non_send_temporary_in_match());
2019-10-10T01:36:41.7804413Z    |     ^^^^^^^^^^^ `std::rc::Rc<()>` cannot be sent between threads safely
2019-10-10T01:36:41.7804459Z    |
2019-10-10T01:36:41.7804531Z    = help: within `impl std::future::Future`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<()>`
2019-10-10T01:36:41.7804593Z    = note: required because it appears within the type `impl std::fmt::Debug`
2019-10-10T01:36:41.7805822Z    = note: required because it appears within the type `{fn(impl std::fmt::Debug) -> std::option::Option<impl std::fmt::Debug> {std::option::Option::<impl std::fmt::Debug>::Some}, fn() -> impl std::fmt::Debug {non_send}, impl std::fmt::Debug, std::option::Option<impl std::fmt::Debug>, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}`
2019-10-10T01:36:41.7806842Z    = note: required because it appears within the type `[static generator@/checkout/src/test/ui/async-await/async-fn-nonsend.rs:28:40: 37:2 {fn(impl std::fmt::Debug) -> std::option::Option<impl std::fmt::Debug> {std::option::Option::<impl std::fmt::Debug>::Some}, fn() -> impl std::fmt::Debug {non_send}, impl std::fmt::Debug, std::option::Option<impl std::fmt::Debug>, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]`
2019-10-10T01:36:41.7807819Z    = note: required because it appears within the type `std::future::GenFuture<[static generator@/checkout/src/test/ui/async-await/async-fn-nonsend.rs:28:40: 37:2 {fn(impl std::fmt::Debug) -> std::option::Option<impl std::fmt::Debug> {std::option::Option::<impl std::fmt::Debug>::Some}, fn() -> impl std::fmt::Debug {non_send}, impl std::fmt::Debug, std::option::Option<impl std::fmt::Debug>, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]>`
2019-10-10T01:36:41.7807964Z    = note: required because it appears within the type `impl std::future::Future`
2019-10-10T01:36:41.7808038Z    = note: required because it appears within the type `impl std::future::Future`
2019-10-10T01:36:41.7808077Z 
2019-10-10T01:36:41.7808126Z error[E0277]: `dyn std::fmt::Write` cannot be sent between threads safely
2019-10-10T01:36:41.7808625Z    |
2019-10-10T01:36:41.7808625Z    |
2019-10-10T01:36:41.7808668Z LL | fn assert_send(_: impl Send) {}
2019-10-10T01:36:41.7808987Z ...
2019-10-10T01:36:41.7808987Z ...
2019-10-10T01:36:41.7809032Z LL |     assert_send(non_sync_with_method_call());
2019-10-10T01:36:41.7809082Z    |     ^^^^^^^^^^^ `dyn std::fmt::Write` cannot be sent between threads safely
2019-10-10T01:36:41.7809195Z    = help: the trait `std::marker::Send` is not implemented for `dyn std::fmt::Write`
2019-10-10T01:36:41.7809364Z    = note: required because of the requirements on the impl of `std::marker::Send` for `&mut dyn std::fmt::Write`
2019-10-10T01:36:41.7809683Z    = note: required because it appears within the type `std::fmt::Formatter<'_>`
2019-10-10T01:36:41.7809986Z    = note: required because of the requirements on the impl of `std::marker::Send` for `&mut std::fmt::Formatter<'_>`
2019-10-10T01:36:41.7809986Z    = note: required because of the requirements on the impl of `std::marker::Send` for `&mut std::fmt::Formatter<'_>`
2019-10-10T01:36:41.7810561Z    = note: required because it appears within the type `for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}`
2019-10-10T01:36:41.7811232Z    = note: required because it appears within the type `[static generator@/checkout/src/test/ui/async-await/async-fn-nonsend.rs:39:38: 45:2 for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]`
2019-10-10T01:36:41.7811950Z    = note: required because it appears within the type `std::future::GenFuture<[static generator@/checkout/src/test/ui/async-await/async-fn-nonsend.rs:39:38: 45:2 for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]>`
2019-10-10T01:36:41.7812059Z    = note: required because it appears within the type `impl std::future::Future`
2019-10-10T01:36:41.7812114Z    = note: required because it appears within the type `impl std::future::Future`
2019-10-10T01:36:41.7812243Z 
2019-10-10T01:36:41.7812571Z error[E0277]: `*mut (dyn std::ops::Fn() + 'static)` cannot be shared between threads safely
2019-10-10T01:36:41.7812882Z    |
2019-10-10T01:36:41.7812882Z    |
2019-10-10T01:36:41.7812942Z LL | fn assert_send(_: impl Send) {}
2019-10-10T01:36:41.7813242Z ...
2019-10-10T01:36:41.7813242Z ...
2019-10-10T01:36:41.7813307Z LL |     assert_send(non_sync_with_method_call());
2019-10-10T01:36:41.7813584Z    |     ^^^^^^^^^^^ `*mut (dyn std::ops::Fn() + 'static)` cannot be shared between threads safely
2019-10-10T01:36:41.7813634Z    |
2019-10-10T01:36:41.7813967Z    = help: within `std::fmt::ArgumentV1<'_>`, the trait `std::marker::Sync` is not implemented for `*mut (dyn std::ops::Fn() + 'static)`
2019-10-10T01:36:41.7814280Z    = note: required because it appears within the type `std::marker::PhantomData<*mut (dyn std::ops::Fn() + 'static)>`
2019-10-10T01:36:41.7814361Z    = note: required because it appears within the type `core::fmt::Void`
2019-10-10T01:36:41.7814431Z    = note: required because it appears within the type `&core::fmt::Void`
2019-10-10T01:36:41.7814701Z    = note: required because it appears within the type `std::fmt::ArgumentV1<'_>`
2019-10-10T01:36:41.7815398Z    = note: required because of the requirements on the impl of `std::marker::Send` for `std::slice::Iter<'_, std::fmt::ArgumentV1<'_>>`
2019-10-10T01:36:41.7815706Z    = note: required because it appears within the type `std::fmt::Formatter<'_>`
2019-10-10T01:36:41.7816014Z    = note: required because of the requirements on the impl of `std::marker::Send` for `&mut std::fmt::Formatter<'_>`
2019-10-10T01:36:41.7816618Z    = note: required because it appears within the type `for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}`
2019-10-10T01:36:41.7817453Z    = note: required because it appears within the type `[static generator@/checkout/src/test/ui/async-await/async-fn-nonsend.rs:39:38: 45:2 for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]`
2019-10-10T01:36:41.7818202Z    = note: required because it appears within the type `std::future::GenFuture<[static generator@/checkout/src/test/ui/async-await/async-fn-nonsend.rs:39:38: 45:2 for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]>`
2019-10-10T01:36:41.7818322Z    = note: required because it appears within the type `impl std::future::Future`
2019-10-10T01:36:41.7818379Z    = note: required because it appears within the type `impl std::future::Future`
2019-10-10T01:36:41.7818645Z error: aborting due to 4 previous errors
2019-10-10T01:36:41.7818675Z 
2019-10-10T01:36:41.7818932Z For more information about this error, try `rustc --explain E0277`.
2019-10-10T01:36:41.7818967Z 
---
2019-10-10T01:36:41.7827847Z ---- [ui] ui/async-await/async-fn-size.rs stdout ----
2019-10-10T01:36:41.7827884Z 
2019-10-10T01:36:41.7828125Z error: test compilation failed although it shouldn't!
2019-10-10T01:36:41.7828177Z status: exit code: 1
2019-10-10T01:36:41.7829089Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-fn-size.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-size/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-size/auxiliary"
2019-10-10T01:36:41.7829429Z ------------------------------------------
2019-10-10T01:36:41.7829464Z 
2019-10-10T01:36:41.7829924Z ------------------------------------------
2019-10-10T01:36:41.7830001Z stderr:
2019-10-10T01:36:41.7830001Z stderr:
2019-10-10T01:36:41.7830232Z ------------------------------------------
2019-10-10T01:36:41.7830536Z error: reached the type-length limit while instantiating `<std::boxed::Box<std::future::Ge...>, ()}]>, ()}]>, ()}]>>>>>::into`
2019-10-10T01:36:41.7830851Z    |
2019-10-10T01:36:41.7831068Z LL | /     fn into(self) -> U {
2019-10-10T01:36:41.7831132Z LL | |         U::from(self)
2019-10-10T01:36:41.7831177Z LL | |     }
2019-10-10T01:36:41.7831177Z LL | |     }
2019-10-10T01:36:41.7831218Z    | |_____^
2019-10-10T01:36:41.7831259Z    |
2019-10-10T01:36:41.7831327Z    = note: consider adding a `#![type_length_limit="1266781"]` attribute to your crate
2019-10-10T01:36:41.7831405Z error: aborting due to previous error
2019-10-10T01:36:41.7831434Z 
2019-10-10T01:36:41.7831477Z 
2019-10-10T01:36:41.7831704Z ------------------------------------------
2019-10-10T01:36:41.7831704Z ------------------------------------------
2019-10-10T01:36:41.7831745Z 
2019-10-10T01:36:41.7831771Z 
2019-10-10T01:36:41.7832041Z ---- [ui] ui/async-await/issues/issue-62009-1.rs stdout ----
2019-10-10T01:36:41.7832090Z diff of stderr:
2019-10-10T01:36:41.7832118Z 
2019-10-10T01:36:41.7832164Z 1 error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-10-10T01:36:41.7832445Z -   --> /checkout/src/test/ui/async-await/issues/issue-62009-1.rs:8:5
2019-10-10T01:36:41.7832671Z +   --> $DIR/issue-62009-1.rs:8:5
2019-10-10T01:36:41.7832779Z 4 LL | fn main() {
2019-10-10T01:36:41.7832998Z 5    |    ---- this is not `async`
2019-10-10T01:36:41.7833083Z 
2019-10-10T01:36:41.7833083Z 
2019-10-10T01:36:41.7833134Z 7    |     ^^^^^^^^^^^^^^^^^^^^^^^ only allowed inside `async` functions and blocks
2019-10-10T01:36:41.7833403Z 8 
2019-10-10T01:36:41.7833453Z 9 error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-10-10T01:36:41.7833731Z -   --> /checkout/src/test/ui/async-await/issues/issue-62009-1.rs:10:5
2019-10-10T01:36:41.7833996Z +   --> $DIR/issue-62009-1.rs:10:5
2019-10-10T01:36:41.7834362Z 12 LL |   fn main() {
2019-10-10T01:36:41.7834643Z 13    |      ---- this is not `async`
2019-10-10T01:36:41.7834677Z 
2019-10-10T01:36:41.7834718Z 14 ...
2019-10-10T01:36:41.7834718Z 14 ...
2019-10-10T01:36:41.7834758Z 15 LL | /     async {
2019-10-10T01:36:41.7835518Z - LL | |     //~^ ERROR `await` is only allowed inside `async` functions and blocks
2019-10-10T01:36:41.7835577Z + LL | |
2019-10-10T01:36:41.7835623Z 17 LL | |         let task1 = print_dur().await;
2019-10-10T01:36:41.7835686Z 18 LL | |     }.await;
2019-10-10T01:36:41.7835736Z 19    | |___________^ only allowed inside `async` functions and blocks
2019-10-10T01:36:41.7835809Z 20 
2019-10-10T01:36:41.7835809Z 20 
2019-10-10T01:36:41.7835877Z 21 error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-10-10T01:36:41.7836155Z -   --> /checkout/src/test/ui/async-await/issues/issue-62009-1.rs:14:5
2019-10-10T01:36:41.7836390Z +   --> $DIR/issue-62009-1.rs:14:5
2019-10-10T01:36:41.7836516Z 24 LL | fn main() {
2019-10-10T01:36:41.7836751Z 25    |    ---- this is not `async`
2019-10-10T01:36:41.7836786Z 
2019-10-10T01:36:41.7836786Z 
2019-10-10T01:36:41.7836848Z 27 LL |     (|_| 2333).await;
2019-10-10T01:36:41.7836900Z 28    |     ^^^^^^^^^^^^^^^^ only allowed inside `async` functions and blocks
2019-10-10T01:36:41.7836946Z 29 
2019-10-10T01:36:41.7837317Z - error[E0277]: the trait bound `[closure@/checkout/src/test/ui/async-await/issues/issue-62009-1.rs:14:5: 14:15]: std::future::Future` is not satisfied
2019-10-10T01:36:41.7837601Z -   --> /checkout/src/test/ui/async-await/issues/issue-62009-1.rs:14:5
2019-10-10T01:36:41.7837922Z + error[E0277]: the trait bound `[closure@$DIR/issue-62009-1.rs:14:5: 14:15]: std::future::Future` is not satisfied
2019-10-10T01:36:41.7838181Z +   --> $DIR/issue-62009-1.rs:14:5
2019-10-10T01:36:41.7838228Z 32    |
2019-10-10T01:36:41.7838271Z 33 LL |     (|_| 2333).await;
2019-10-10T01:36:41.7838655Z -    |     ^^^^^^^^^^^^^^^^ the trait `std::future::Future` is not implemented for `[closure@/checkout/src/test/ui/async-await/issues/issue-62009-1.rs:14:5: 14:15]`
2019-10-10T01:36:41.7839151Z +    |     ^^^^^^^^^^^^^^^^ the trait `std::future::Future` is not implemented for `[closure@$DIR/issue-62009-1.rs:14:5: 14:15]`
2019-10-10T01:36:41.7839208Z 35    |
2019-10-10T01:36:41.7839275Z 36    = note: required by `std::future::IntoFuture::into_future`
2019-10-10T01:36:41.7839348Z 
2019-10-10T01:36:41.7839375Z 
2019-10-10T01:36:41.7839440Z The actual stderr differed from the expected stderr.
2019-10-10T01:36:41.7839774Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62009-1/issue-62009-1.stderr
2019-10-10T01:36:41.7839774Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62009-1/issue-62009-1.stderr
2019-10-10T01:36:41.7840039Z To update references, rerun the tests and pass the `--bless` flag
2019-10-10T01:36:41.7840377Z To only update this specific test, also pass `--test-args async-await/issues/issue-62009-1.rs`
2019-10-10T01:36:41.7840477Z error: 1 errors occurred comparing output.
2019-10-10T01:36:41.7840551Z status: exit code: 1
2019-10-10T01:36:41.7840551Z status: exit code: 1
2019-10-10T01:36:41.7841367Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-62009-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62009-1" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62009-1/auxiliary" "-A" "unused"
2019-10-10T01:36:41.7841743Z ------------------------------------------
2019-10-10T01:36:41.7841782Z 
2019-10-10T01:36:41.7842058Z ------------------------------------------
2019-10-10T01:36:41.7842109Z stderr:
2019-10-10T01:36:41.7842109Z stderr:
2019-10-10T01:36:41.7842371Z ------------------------------------------
2019-10-10T01:36:41.7842528Z error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-10-10T01:36:41.7842893Z   --> /checkout/src/test/ui/async-await/issues/issue-62009-1.rs:8:5
2019-10-10T01:36:41.7842998Z LL | fn main() {
2019-10-10T01:36:41.7843261Z    |    ---- this is not `async`
2019-10-10T01:36:41.7843261Z    |    ---- this is not `async`
2019-10-10T01:36:41.7843315Z LL |     async { let (); }.await;
2019-10-10T01:36:41.7843371Z    |     ^^^^^^^^^^^^^^^^^^^^^^^ only allowed inside `async` functions and blocks
2019-10-10T01:36:41.7843423Z 
2019-10-10T01:36:41.7843474Z error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-10-10T01:36:41.7843765Z   --> /checkout/src/test/ui/async-await/issues/issue-62009-1.rs:10:5
2019-10-10T01:36:41.7843883Z LL |   fn main() {
2019-10-10T01:36:41.7844126Z    |      ---- this is not `async`
2019-10-10T01:36:41.7844176Z ...
2019-10-10T01:36:41.7844240Z LL | /     async {
2019-10-10T01:36:41.7844240Z LL | /     async {
2019-10-10T01:36:41.7844311Z LL | |     //~^ ERROR `await` is only allowed inside `async` functions and blocks
2019-10-10T01:36:41.7844366Z LL | |         let task1 = print_dur().await;
2019-10-10T01:36:41.7844433Z LL | |     }.await;
2019-10-10T01:36:41.7844486Z    | |___________^ only allowed inside `async` functions and blocks
2019-10-10T01:36:41.7844521Z 
2019-10-10T01:36:41.7844572Z error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-10-10T01:36:41.7845108Z   --> /checkout/src/test/ui/async-await/issues/issue-62009-1.rs:14:5
2019-10-10T01:36:41.7845296Z LL | fn main() {
2019-10-10T01:36:41.7845576Z    |    ---- this is not `async`
2019-10-10T01:36:41.7845623Z ...
2019-10-10T01:36:41.7845623Z ...
2019-10-10T01:36:41.7845666Z LL |     (|_| 2333).await;
2019-10-10T01:36:41.7845734Z    |     ^^^^^^^^^^^^^^^^ only allowed inside `async` functions and blocks
2019-10-10T01:36:41.7845767Z 
2019-10-10T01:36:41.7846118Z error[E0277]: the trait bound `[closure@/checkout/src/test/ui/async-await/issues/issue-62009-1.rs:14:5: 14:15]: std::future::Future` is not satisfied
2019-10-10T01:36:41.7846568Z   --> /checkout/src/test/ui/async-await/issues/issue-62009-1.rs:14:5
2019-10-10T01:36:41.7846644Z    |
2019-10-10T01:36:41.7846687Z LL |     (|_| 2333).await;
2019-10-10T01:36:41.7847047Z    |     ^^^^^^^^^^^^^^^^ the trait `std::future::Future` is not implemented for `[closure@/checkout/src/test/ui/async-await/issues/issue-62009-1.rs:14:5: 14:15]`
2019-10-10T01:36:41.7847124Z    |
2019-10-10T01:36:41.7847172Z    = note: required by `std::future::IntoFuture::into_future`
2019-10-10T01:36:41.7847268Z error: aborting due to 4 previous errors
2019-10-10T01:36:41.7847299Z 
2019-10-10T01:36:41.7847558Z For more information about this error, try `rustc --explain E0277`.
2019-10-10T01:36:41.7847594Z 
---
2019-10-10T01:36:41.7849427Z test result: FAILED. 9099 passed; 4 failed; 40 ignored; 0 measured; 0 filtered out
2019-10-10T01:36:41.7849464Z 
2019-10-10T01:36:41.7850986Z 
2019-10-10T01:36:41.7851034Z 
2019-10-10T01:36:41.7852668Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-10T01:36:41.7852946Z 
2019-10-10T01:36:41.7852976Z 
2019-10-10T01:36:41.7853028Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-10T01:36:41.7853105Z Build completed unsuccessfully in 1:07:38
2019-10-10T01:36:41.7853105Z Build completed unsuccessfully in 1:07:38
2019-10-10T01:36:41.7853425Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-10T01:36:41.7853488Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-10T01:36:41.7863899Z == clock drift check ==
2019-10-10T01:36:41.7887126Z   local time: Thu Oct 10 01:36:41 UTC 2019
2019-10-10T01:36:41.8617619Z   network time: Thu, 10 Oct 2019 01:36:41 GMT
2019-10-10T01:36:41.8617785Z == end clock drift check ==
2019-10-10T01:36:42.3263172Z ##[error]Bash exited with code '1'.
2019-10-10T01:36:42.3309029Z ##[section]Starting: Checkout
2019-10-10T01:36:42.3310869Z ==============================================================================
2019-10-10T01:36:42.3310923Z Task         : Get sources
2019-10-10T01:36:42.3310972Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
