plain
2019-10-09T21:59:54.8448981Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-09T21:59:54.8559286Z ##[command]git config gc.auto 0
2019-10-09T21:59:54.8646786Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-09T21:59:54.8710417Z ##[command]git config --get-all http.proxy
2019-10-09T21:59:54.8857052Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65244/merge:refs/remotes/pull/65244/merge
---
2019-10-09T23:03:05.9578817Z .................................................................................................... 1600/9143
2019-10-09T23:03:12.6299563Z .................................................................................................... 1700/9143
2019-10-09T23:03:24.0007346Z ..................i...............i................................................................. 1800/9143
2019-10-09T23:03:31.0754027Z .................................................................................................... 1900/9143
2019-10-09T23:03:48.6427648Z .........iiiii...................................................................................... 2000/9143
2019-10-09T23:03:59.4127776Z .................................................................................................... 2200/9143
2019-10-09T23:04:01.9206341Z .................................................................................................... 2300/9143
2019-10-09T23:04:08.6044574Z .................................................................................................... 2400/9143
2019-10-09T23:04:15.8426224Z .................................................................................................... 2500/9143
---
2019-10-09T23:07:18.9882913Z .................................................................................................... 4700/9143
2019-10-09T23:07:26.3408520Z ..i...............i................................................................................. 4800/9143
2019-10-09T23:07:38.2229793Z .................................................................................................... 4900/9143
2019-10-09T23:07:45.2218365Z .................................................................................................... 5000/9143
2019-10-09T23:07:57.4953780Z ................................................................................................ii.i 5100/9143
2019-10-09T23:08:02.8051537Z i................................................................................................... 5200/9143
2019-10-09T23:08:17.9405240Z .................................................................................................... 5400/9143
2019-10-09T23:08:25.0786509Z ..............................................................i..................................... 5500/9143
2019-10-09T23:08:33.9766819Z .................................................................................................... 5600/9143
2019-10-09T23:08:41.5503517Z .................................................................................................... 5700/9143
2019-10-09T23:08:41.5503517Z .................................................................................................... 5700/9143
2019-10-09T23:08:54.2087059Z ...........................................................ii...i..ii...........i................... 5800/9143
2019-10-09T23:09:19.9420229Z .................................................................................................... 6000/9143
2019-10-09T23:09:29.1142424Z .................................................................................................... 6100/9143
2019-10-09T23:09:29.1142424Z .................................................................................................... 6100/9143
2019-10-09T23:09:35.4888296Z .................................................................i..ii.............................. 6200/9143
2019-10-09T23:10:10.2536098Z .................................................................................................... 6400/9143
2019-10-09T23:10:12.8713347Z .........................i.......................................................................... 6500/9143
2019-10-09T23:10:15.5372892Z ..................................................................................................i. 6600/9143
2019-10-09T23:10:18.5692964Z .................................................................................................... 6700/9143
---
2019-10-09T23:14:38.5592809Z ---- [ui] ui/async-await/async-fn-nonsend.rs stdout ----
2019-10-09T23:14:38.5592874Z diff of stderr:
2019-10-09T23:14:38.5592910Z 
2019-10-09T23:14:38.5592972Z 9    |
2019-10-09T23:14:38.5593029Z 10    = help: within `impl std::future::Future`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<()>`
2019-10-09T23:14:38.5593113Z 11    = note: required because it appears within the type `impl std::fmt::Debug`
2019-10-09T23:14:38.5593526Z -    = note: required because it appears within the type `{impl std::fmt::Debug, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, ()}`
2019-10-09T23:14:38.5593973Z -    = note: required because it appears within the type `[static generator@$DIR/async-fn-nonsend.rs:21:39: 26:2 {impl std::fmt::Debug, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, ()}]`
2019-10-09T23:14:38.5594452Z -    = note: required because it appears within the type `std::future::GenFuture<[static generator@$DIR/async-fn-nonsend.rs:21:39: 26:2 {impl std::fmt::Debug, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, ()}]>`
2019-10-09T23:14:38.5595016Z +    = note: required because it appears within the type `{impl std::fmt::Debug, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}`
2019-10-09T23:14:38.5595683Z +    = note: required because it appears within the type `[static generator@$DIR/async-fn-nonsend.rs:21:39: 26:2 {impl std::fmt::Debug, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]`
2019-10-09T23:14:38.5596433Z +    = note: required because it appears within the type `std::future::GenFuture<[static generator@$DIR/async-fn-nonsend.rs:21:39: 26:2 {impl std::fmt::Debug, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]>`
2019-10-09T23:14:38.5596710Z 15    = note: required because it appears within the type `impl std::future::Future`
2019-10-09T23:14:38.5596777Z 16    = note: required because it appears within the type `impl std::future::Future`
2019-10-09T23:14:38.5596843Z 
2019-10-09T23:14:38.5597002Z 26    |
2019-10-09T23:14:38.5597002Z 26    |
2019-10-09T23:14:38.5597057Z 27    = help: within `impl std::future::Future`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<()>`
2019-10-09T23:14:38.5597106Z 28    = note: required because it appears within the type `impl std::fmt::Debug`
2019-10-09T23:14:38.5597636Z -    = note: required because it appears within the type `{fn(impl std::fmt::Debug) -> std::option::Option<impl std::fmt::Debug> {std::option::Option::<impl std::fmt::Debug>::Some}, fn() -> impl std::fmt::Debug {non_send}, impl std::fmt::Debug, std::option::Option<impl std::fmt::Debug>, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, ()}`
2019-10-09T23:14:38.5598175Z -    = note: required because it appears within the type `[static generator@$DIR/async-fn-nonsend.rs:28:40: 37:2 {fn(impl std::fmt::Debug) -> std::option::Option<impl std::fmt::Debug> {std::option::Option::<impl std::fmt::Debug>::Some}, fn() -> impl std::fmt::Debug {non_send}, impl std::fmt::Debug, std::option::Option<impl std::fmt::Debug>, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, ()}]`
2019-10-09T23:14:38.5598733Z -    = note: required because it appears within the type `std::future::GenFuture<[static generator@$DIR/async-fn-nonsend.rs:28:40: 37:2 {fn(impl std::fmt::Debug) -> std::option::Option<impl std::fmt::Debug> {std::option::Option::<impl std::fmt::Debug>::Some}, fn() -> impl std::fmt::Debug {non_send}, impl std::fmt::Debug, std::option::Option<impl std::fmt::Debug>, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, ()}]>`
2019-10-09T23:14:38.5599401Z +    = note: required because it appears within the type `{fn(impl std::fmt::Debug) -> std::option::Option<impl std::fmt::Debug> {std::option::Option::<impl std::fmt::Debug>::Some}, fn() -> impl std::fmt::Debug {non_send}, impl std::fmt::Debug, std::option::Option<impl std::fmt::Debug>, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}`
2019-10-09T23:14:38.5600096Z +    = note: required because it appears within the type `[static generator@$DIR/async-fn-nonsend.rs:28:40: 37:2 {fn(impl std::fmt::Debug) -> std::option::Option<impl std::fmt::Debug> {std::option::Option::<impl std::fmt::Debug>::Some}, fn() -> impl std::fmt::Debug {non_send}, impl std::fmt::Debug, std::option::Option<impl std::fmt::Debug>, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]`
2019-10-09T23:14:38.5600819Z +    = note: required because it appears within the type `std::future::GenFuture<[static generator@$DIR/async-fn-nonsend.rs:28:40: 37:2 {fn(impl std::fmt::Debug) -> std::option::Option<impl std::fmt::Debug> {std::option::Option::<impl std::fmt::Debug>::Some}, fn() -> impl std::fmt::Debug {non_send}, impl std::fmt::Debug, std::option::Option<impl std::fmt::Debug>, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]>`
2019-10-09T23:14:38.5601007Z 32    = note: required because it appears within the type `impl std::future::Future`
2019-10-09T23:14:38.5601053Z 33    = note: required because it appears within the type `impl std::future::Future`
2019-10-09T23:14:38.5601119Z 
2019-10-09T23:14:38.5601181Z 45    = note: required because of the requirements on the impl of `std::marker::Send` for `&mut dyn std::fmt::Write`
2019-10-09T23:14:38.5601514Z 46    = note: required because it appears within the type `std::fmt::Formatter<'_>`
2019-10-09T23:14:38.5601856Z 47    = note: required because of the requirements on the impl of `std::marker::Send` for `&mut std::fmt::Formatter<'_>`
2019-10-09T23:14:38.5601856Z 47    = note: required because of the requirements on the impl of `std::marker::Send` for `&mut std::fmt::Formatter<'_>`
2019-10-09T23:14:38.5602197Z -    = note: required because it appears within the type `for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, ()}`
2019-10-09T23:14:38.5602586Z -    = note: required because it appears within the type `[static generator@$DIR/async-fn-nonsend.rs:39:38: 45:2 for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, ()}]`
2019-10-09T23:14:38.5603040Z -    = note: required because it appears within the type `std::future::GenFuture<[static generator@$DIR/async-fn-nonsend.rs:39:38: 45:2 for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, ()}]>`
2019-10-09T23:14:38.5603552Z +    = note: required because it appears within the type `for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}`
2019-10-09T23:14:38.5604197Z +    = note: required because it appears within the type `[static generator@$DIR/async-fn-nonsend.rs:39:38: 45:2 for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]`
2019-10-09T23:14:38.5604755Z +    = note: required because it appears within the type `std::future::GenFuture<[static generator@$DIR/async-fn-nonsend.rs:39:38: 45:2 for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]>`
2019-10-09T23:14:38.5604843Z 51    = note: required because it appears within the type `impl std::future::Future`
2019-10-09T23:14:38.5604894Z 52    = note: required because it appears within the type `impl std::future::Future`
2019-10-09T23:14:38.5604973Z 
2019-10-09T23:14:38.5604973Z 
2019-10-09T23:14:38.5605234Z 68    = note: required because of the requirements on the impl of `std::marker::Send` for `std::slice::Iter<'_, std::fmt::ArgumentV1<'_>>`
2019-10-09T23:14:38.5605464Z 69    = note: required because it appears within the type `std::fmt::Formatter<'_>`
2019-10-09T23:14:38.5605719Z 70    = note: required because of the requirements on the impl of `std::marker::Send` for `&mut std::fmt::Formatter<'_>`
2019-10-09T23:14:38.5606028Z -    = note: required because it appears within the type `for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, ()}`
2019-10-09T23:14:38.5606411Z -    = note: required because it appears within the type `[static generator@$DIR/async-fn-nonsend.rs:39:38: 45:2 for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, ()}]`
2019-10-09T23:14:38.5606907Z -    = note: required because it appears within the type `std::future::GenFuture<[static generator@$DIR/async-fn-nonsend.rs:39:38: 45:2 for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, ()}]>`
2019-10-09T23:14:38.5607754Z +    = note: required because it appears within the type `for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}`
2019-10-09T23:14:38.5608490Z +    = note: required because it appears within the type `[static generator@$DIR/async-fn-nonsend.rs:39:38: 45:2 for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]`
2019-10-09T23:14:38.5609211Z +    = note: required because it appears within the type `std::future::GenFuture<[static generator@$DIR/async-fn-nonsend.rs:39:38: 45:2 for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]>`
2019-10-09T23:14:38.5609302Z 74    = note: required because it appears within the type `impl std::future::Future`
2019-10-09T23:14:38.5609385Z 75    = note: required because it appears within the type `impl std::future::Future`
2019-10-09T23:14:38.5609462Z 
2019-10-09T23:14:38.5609487Z 
2019-10-09T23:14:38.5609551Z The actual stderr differed from the expected stderr.
2019-10-09T23:14:38.5609878Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-nonsend/async-fn-nonsend.stderr
2019-10-09T23:14:38.5609878Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-nonsend/async-fn-nonsend.stderr
2019-10-09T23:14:38.5610140Z To update references, rerun the tests and pass the `--bless` flag
2019-10-09T23:14:38.5610454Z To only update this specific test, also pass `--test-args async-await/async-fn-nonsend.rs`
2019-10-09T23:14:38.5610540Z error: 1 errors occurred comparing output.
2019-10-09T23:14:38.5610608Z status: exit code: 1
2019-10-09T23:14:38.5610608Z status: exit code: 1
2019-10-09T23:14:38.5611545Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-fn-nonsend.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-nonsend" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--crate-type" "lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-nonsend/auxiliary" "-A" "unused"
2019-10-09T23:14:38.5611842Z ------------------------------------------
2019-10-09T23:14:38.5611870Z 
2019-10-09T23:14:38.5612063Z ------------------------------------------
2019-10-09T23:14:38.5612101Z stderr:
2019-10-09T23:14:38.5612101Z stderr:
2019-10-09T23:14:38.5612272Z ------------------------------------------
2019-10-09T23:14:38.5612314Z error[E0277]: `std::rc::Rc<()>` cannot be sent between threads safely
2019-10-09T23:14:38.5612570Z    |
2019-10-09T23:14:38.5612570Z    |
2019-10-09T23:14:38.5612787Z LL | fn assert_send(_: impl Send) {}
2019-10-09T23:14:38.5613332Z ...
2019-10-09T23:14:38.5613332Z ...
2019-10-09T23:14:38.5613379Z LL |     assert_send(local_dropped_before_await());
2019-10-09T23:14:38.5613451Z    |     ^^^^^^^^^^^ `std::rc::Rc<()>` cannot be sent between threads safely
2019-10-09T23:14:38.5613497Z    |
2019-10-09T23:14:38.5613633Z    = help: within `impl std::future::Future`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<()>`
2019-10-09T23:14:38.5613725Z    = note: required because it appears within the type `impl std::fmt::Debug`
2019-10-09T23:14:38.5614292Z    = note: required because it appears within the type `{impl std::fmt::Debug, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}`
2019-10-09T23:14:38.5614960Z    = note: required because it appears within the type `[static generator@/checkout/src/test/ui/async-await/async-fn-nonsend.rs:21:39: 26:2 {impl std::fmt::Debug, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]`
2019-10-09T23:14:38.5615653Z    = note: required because it appears within the type `std::future::GenFuture<[static generator@/checkout/src/test/ui/async-await/async-fn-nonsend.rs:21:39: 26:2 {impl std::fmt::Debug, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]>`
2019-10-09T23:14:38.5615752Z    = note: required because it appears within the type `impl std::future::Future`
2019-10-09T23:14:38.5615829Z    = note: required because it appears within the type `impl std::future::Future`
2019-10-09T23:14:38.5615865Z 
2019-10-09T23:14:38.5615912Z error[E0277]: `std::rc::Rc<()>` cannot be sent between threads safely
2019-10-09T23:14:38.5616538Z    |
2019-10-09T23:14:38.5616538Z    |
2019-10-09T23:14:38.5616574Z LL | fn assert_send(_: impl Send) {}
2019-10-09T23:14:38.5616831Z ...
2019-10-09T23:14:38.5616831Z ...
2019-10-09T23:14:38.5616867Z LL |     assert_send(non_send_temporary_in_match());
2019-10-09T23:14:38.5616909Z    |     ^^^^^^^^^^^ `std::rc::Rc<()>` cannot be sent between threads safely
2019-10-09T23:14:38.5616965Z    |
2019-10-09T23:14:38.5617006Z    = help: within `impl std::future::Future`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<()>`
2019-10-09T23:14:38.5617059Z    = note: required because it appears within the type `impl std::fmt::Debug`
2019-10-09T23:14:38.5617661Z    = note: required because it appears within the type `{fn(impl std::fmt::Debug) -> std::option::Option<impl std::fmt::Debug> {std::option::Option::<impl std::fmt::Debug>::Some}, fn() -> impl std::fmt::Debug {non_send}, impl std::fmt::Debug, std::option::Option<impl std::fmt::Debug>, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}`
2019-10-09T23:14:38.5618348Z    = note: required because it appears within the type `[static generator@/checkout/src/test/ui/async-await/async-fn-nonsend.rs:28:40: 37:2 {fn(impl std::fmt::Debug) -> std::option::Option<impl std::fmt::Debug> {std::option::Option::<impl std::fmt::Debug>::Some}, fn() -> impl std::fmt::Debug {non_send}, impl std::fmt::Debug, std::option::Option<impl std::fmt::Debug>, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]`
2019-10-09T23:14:38.5619232Z    = note: required because it appears within the type `std::future::GenFuture<[static generator@/checkout/src/test/ui/async-await/async-fn-nonsend.rs:28:40: 37:2 {fn(impl std::fmt::Debug) -> std::option::Option<impl std::fmt::Debug> {std::option::Option::<impl std::fmt::Debug>::Some}, fn() -> impl std::fmt::Debug {non_send}, impl std::fmt::Debug, std::option::Option<impl std::fmt::Debug>, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]>`
2019-10-09T23:14:38.5619332Z    = note: required because it appears within the type `impl std::future::Future`
2019-10-09T23:14:38.5619393Z    = note: required because it appears within the type `impl std::future::Future`
2019-10-09T23:14:38.5619421Z 
2019-10-09T23:14:38.5619458Z error[E0277]: `dyn std::fmt::Write` cannot be sent between threads safely
2019-10-09T23:14:38.5619747Z    |
2019-10-09T23:14:38.5619747Z    |
2019-10-09T23:14:38.5619781Z LL | fn assert_send(_: impl Send) {}
2019-10-09T23:14:38.5620032Z ...
2019-10-09T23:14:38.5620032Z ...
2019-10-09T23:14:38.5620068Z LL |     assert_send(non_sync_with_method_call());
2019-10-09T23:14:38.5620128Z    |     ^^^^^^^^^^^ `dyn std::fmt::Write` cannot be sent between threads safely
2019-10-09T23:14:38.5620203Z    = help: the trait `std::marker::Send` is not implemented for `dyn std::fmt::Write`
2019-10-09T23:14:38.5620270Z    = note: required because of the requirements on the impl of `std::marker::Send` for `&mut dyn std::fmt::Write`
2019-10-09T23:14:38.5620506Z    = note: required because it appears within the type `std::fmt::Formatter<'_>`
2019-10-09T23:14:38.5620740Z    = note: required because of the requirements on the impl of `std::marker::Send` for `&mut std::fmt::Formatter<'_>`
2019-10-09T23:14:38.5620740Z    = note: required because of the requirements on the impl of `std::marker::Send` for `&mut std::fmt::Formatter<'_>`
2019-10-09T23:14:38.5621210Z    = note: required because it appears within the type `for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}`
2019-10-09T23:14:38.5622338Z    = note: required because it appears within the type `[static generator@/checkout/src/test/ui/async-await/async-fn-nonsend.rs:39:38: 45:2 for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]`
2019-10-09T23:14:38.5623256Z    = note: required because it appears within the type `std::future::GenFuture<[static generator@/checkout/src/test/ui/async-await/async-fn-nonsend.rs:39:38: 45:2 for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]>`
2019-10-09T23:14:38.5623365Z    = note: required because it appears within the type `impl std::future::Future`
2019-10-09T23:14:38.5623515Z    = note: required because it appears within the type `impl std::future::Future`
2019-10-09T23:14:38.5623551Z 
2019-10-09T23:14:38.5623869Z error[E0277]: `*mut (dyn std::ops::Fn() + 'static)` cannot be shared between threads safely
2019-10-09T23:14:38.5624165Z    |
2019-10-09T23:14:38.5624165Z    |
2019-10-09T23:14:38.5624228Z LL | fn assert_send(_: impl Send) {}
2019-10-09T23:14:38.5624610Z ...
2019-10-09T23:14:38.5624610Z ...
2019-10-09T23:14:38.5624675Z LL |     assert_send(non_sync_with_method_call());
2019-10-09T23:14:38.5624979Z    |     ^^^^^^^^^^^ `*mut (dyn std::ops::Fn() + 'static)` cannot be shared between threads safely
2019-10-09T23:14:38.5625028Z    |
2019-10-09T23:14:38.5625352Z    = help: within `std::fmt::ArgumentV1<'_>`, the trait `std::marker::Sync` is not implemented for `*mut (dyn std::ops::Fn() + 'static)`
2019-10-09T23:14:38.5625662Z    = note: required because it appears within the type `std::marker::PhantomData<*mut (dyn std::ops::Fn() + 'static)>`
2019-10-09T23:14:38.5625723Z    = note: required because it appears within the type `core::fmt::Void`
2019-10-09T23:14:38.5625793Z    = note: required because it appears within the type `&core::fmt::Void`
2019-10-09T23:14:38.5626366Z    = note: required because it appears within the type `std::fmt::ArgumentV1<'_>`
2019-10-09T23:14:38.5626676Z    = note: required because of the requirements on the impl of `std::marker::Send` for `std::slice::Iter<'_, std::fmt::ArgumentV1<'_>>`
2019-10-09T23:14:38.5626923Z    = note: required because it appears within the type `std::fmt::Formatter<'_>`
2019-10-09T23:14:38.5627192Z    = note: required because of the requirements on the impl of `std::marker::Send` for `&mut std::fmt::Formatter<'_>`
2019-10-09T23:14:38.5627875Z    = note: required because it appears within the type `for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}`
2019-10-09T23:14:38.5680857Z    = note: required because it appears within the type `[static generator@/checkout/src/test/ui/async-await/async-fn-nonsend.rs:39:38: 45:2 for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]`
2019-10-09T23:14:38.5681711Z    = note: required because it appears within the type `std::future::GenFuture<[static generator@/checkout/src/test/ui/async-await/async-fn-nonsend.rs:39:38: 45:2 for<'r, 's> {&'r mut std::fmt::Formatter<'s>, bool, bool, fn(impl std::future::Future) -> <impl std::future::Future as std::future::IntoFuture>::Future {<impl std::future::Future as std::future::IntoFuture>::into_future}, fn() -> impl std::future::Future {fut}, impl std::future::Future, impl std::future::Future, impl std::future::Future, ()}]>`
2019-10-09T23:14:38.5681802Z    = note: required because it appears within the type `impl std::future::Future`
2019-10-09T23:14:38.5681866Z    = note: required because it appears within the type `impl std::future::Future`
2019-10-09T23:14:38.5682307Z error: aborting due to 4 previous errors
2019-10-09T23:14:38.5682346Z 
2019-10-09T23:14:38.5682689Z For more information about this error, try `rustc --explain E0277`.
2019-10-09T23:14:38.5682725Z 
---
2019-10-09T23:14:38.5686226Z ---- [ui] ui/async-await/async-fn-size.rs stdout ----
2019-10-09T23:14:38.5686253Z 
2019-10-09T23:14:38.5686445Z error: test compilation failed although it shouldn't!
2019-10-09T23:14:38.5686483Z status: exit code: 1
2019-10-09T23:14:38.5687078Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-fn-size.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-size/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-size/auxiliary"
2019-10-09T23:14:38.5687344Z ------------------------------------------
2019-10-09T23:14:38.5687394Z 
2019-10-09T23:14:38.5687571Z ------------------------------------------
2019-10-09T23:14:38.5687607Z stderr:
2019-10-09T23:14:38.5687607Z stderr:
2019-10-09T23:14:38.5687774Z ------------------------------------------
2019-10-09T23:14:38.5688020Z error: reached the type-length limit while instantiating `<std::boxed::Box<std::future::Ge...>, ()}]>, ()}]>, ()}]>>>>>::into`
2019-10-09T23:14:38.5688258Z    |
2019-10-09T23:14:38.5688461Z LL | /     fn into(self) -> U {
2019-10-09T23:14:38.5688502Z LL | |         U::from(self)
2019-10-09T23:14:38.5688538Z LL | |     }
2019-10-09T23:14:38.5688538Z LL | |     }
2019-10-09T23:14:38.5688590Z    | |_____^
2019-10-09T23:14:38.5688624Z    |
2019-10-09T23:14:38.5688666Z    = note: consider adding a `#![type_length_limit="1266781"]` attribute to your crate
2019-10-09T23:14:38.5688747Z error: aborting due to previous error
2019-10-09T23:14:38.5688773Z 
2019-10-09T23:14:38.5688794Z 
2019-10-09T23:14:38.5689000Z ------------------------------------------
2019-10-09T23:14:38.5689000Z ------------------------------------------
2019-10-09T23:14:38.5689044Z 
2019-10-09T23:14:38.5689064Z 
2019-10-09T23:14:38.5689264Z ---- [ui] ui/async-await/issues/issue-62009-1.rs stdout ----
2019-10-09T23:14:38.5689304Z diff of stderr:
2019-10-09T23:14:38.5689343Z 
2019-10-09T23:14:38.5689376Z 32    |
2019-10-09T23:14:38.5689411Z 33 LL |     (|_| 2333).await;
2019-10-09T23:14:38.5689692Z 34    |     ^^^^^^^^^^^^^^^^ the trait `std::future::Future` is not implemented for `[closure@$DIR/issue-62009-1.rs:14:5: 14:15]`
2019-10-09T23:14:38.5689874Z -    | 
2019-10-09T23:14:38.5690063Z -   ::: $SRC_DIR/libstd/future.rs:LL:COL
2019-10-09T23:14:38.5690283Z - LL |     F: Future
2019-10-09T23:14:38.5690283Z - LL |     F: Future
2019-10-09T23:14:38.5690501Z -    |        ------ required by this bound in `std::future::poll_with_tls_context`
2019-10-09T23:14:38.5690548Z +    = note: required by `std::future::IntoFuture::into_future`
2019-10-09T23:14:38.5690640Z 41 error: aborting due to 4 previous errors
2019-10-09T23:14:38.5690771Z 42 
2019-10-09T23:14:38.5690794Z 
2019-10-09T23:14:38.5690833Z 
2019-10-09T23:14:38.5690833Z 
2019-10-09T23:14:38.5690871Z The actual stderr differed from the expected stderr.
2019-10-09T23:14:38.5691177Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62009-1/issue-62009-1.stderr
2019-10-09T23:14:38.5697050Z To update references, rerun the tests and pass the `--bless` flag
2019-10-09T23:14:38.5697567Z To only update this specific test, also pass `--test-args async-await/issues/issue-62009-1.rs`
2019-10-09T23:14:38.5697657Z error: 1 errors occurred comparing output.
2019-10-09T23:14:38.5697714Z status: exit code: 1
2019-10-09T23:14:38.5697714Z status: exit code: 1
2019-10-09T23:14:38.5698396Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-62009-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62009-1" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62009-1/auxiliary" "-A" "unused"
2019-10-09T23:14:38.5698694Z ------------------------------------------
2019-10-09T23:14:38.5698722Z 
2019-10-09T23:14:38.5698917Z ------------------------------------------
2019-10-09T23:14:38.5698962Z stderr:
2019-10-09T23:14:38.5698962Z stderr:
2019-10-09T23:14:38.5699138Z ------------------------------------------
2019-10-09T23:14:38.5699297Z error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-10-09T23:14:38.5699682Z   --> /checkout/src/test/ui/async-await/issues/issue-62009-1.rs:8:5
2019-10-09T23:14:38.5699779Z LL | fn main() {
2019-10-09T23:14:38.5699955Z    |    ---- this is not `async`
2019-10-09T23:14:38.5699955Z    |    ---- this is not `async`
2019-10-09T23:14:38.5700199Z LL |     async { let (); }.await;
2019-10-09T23:14:38.5700261Z    |     ^^^^^^^^^^^^^^^^^^^^^^^ only allowed inside `async` functions and blocks
2019-10-09T23:14:38.5700290Z 
2019-10-09T23:14:38.5700328Z error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-10-09T23:14:38.5700547Z   --> /checkout/src/test/ui/async-await/issues/issue-62009-1.rs:10:5
2019-10-09T23:14:38.5700639Z LL |   fn main() {
2019-10-09T23:14:38.5700817Z    |      ---- this is not `async`
2019-10-09T23:14:38.5700878Z ...
2019-10-09T23:14:38.5700914Z LL | /     async {
2019-10-09T23:14:38.5700914Z LL | /     async {
2019-10-09T23:14:38.5700956Z LL | |     //~^ ERROR `await` is only allowed inside `async` functions and blocks
2019-10-09T23:14:38.5701014Z LL | |         let task1 = print_dur().await;
2019-10-09T23:14:38.5701053Z LL | |     }.await;
2019-10-09T23:14:38.5701093Z    | |___________^ only allowed inside `async` functions and blocks
2019-10-09T23:14:38.5701120Z 
2019-10-09T23:14:38.5701176Z error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-10-09T23:14:38.5701400Z   --> /checkout/src/test/ui/async-await/issues/issue-62009-1.rs:14:5
2019-10-09T23:14:38.5701493Z LL | fn main() {
2019-10-09T23:14:38.5701670Z    |    ---- this is not `async`
2019-10-09T23:14:38.5701710Z ...
2019-10-09T23:14:38.5701710Z ...
2019-10-09T23:14:38.5701923Z LL |     (|_| 2333).await;
2019-10-09T23:14:38.5701981Z    |     ^^^^^^^^^^^^^^^^ only allowed inside `async` functions and blocks
2019-10-09T23:14:38.5702008Z 
2019-10-09T23:14:38.5702730Z error[E0277]: the trait bound `[closure@/checkout/src/test/ui/async-await/issues/issue-62009-1.rs:14:5: 14:15]: std::future::Future` is not satisfied
2019-10-09T23:14:38.5703721Z   --> /checkout/src/test/ui/async-await/issues/issue-62009-1.rs:14:5
2019-10-09T23:14:38.5703990Z    |
2019-10-09T23:14:38.5704039Z LL |     (|_| 2333).await;
2019-10-09T23:14:38.5704510Z    |     ^^^^^^^^^^^^^^^^ the trait `std::future::Future` is not implemented for `[closure@/checkout/src/test/ui/async-await/issues/issue-62009-1.rs:14:5: 14:15]`
2019-10-09T23:14:38.5704750Z    |
2019-10-09T23:14:38.5704796Z    = note: required by `std::future::IntoFuture::into_future`
2019-10-09T23:14:38.5704893Z error: aborting due to 4 previous errors
2019-10-09T23:14:38.5704922Z 
2019-10-09T23:14:38.5705222Z For more information about this error, try `rustc --explain E0277`.
2019-10-09T23:14:38.5705273Z 
---
2019-10-09T23:14:38.5706933Z test result: FAILED. 9099 passed; 4 failed; 40 ignored; 0 measured; 0 filtered out
2019-10-09T23:14:38.5706988Z 
2019-10-09T23:14:38.5737488Z 
2019-10-09T23:14:38.5737565Z 
2019-10-09T23:14:38.5739589Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-09T23:14:38.5739826Z 
2019-10-09T23:14:38.5739852Z 
2019-10-09T23:14:38.5740272Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-09T23:14:38.5741118Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-09T23:14:38.5741118Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-09T23:14:38.5741209Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-09T23:14:38.5741256Z Build completed unsuccessfully in 1:08:06
2019-10-09T23:14:38.5741297Z == clock drift check ==
2019-10-09T23:14:38.5741354Z   local time: Wed Oct  9 23:14:38 UTC 2019
2019-10-09T23:14:38.6074182Z   network time: Wed, 09 Oct 2019 23:14:38 GMT
2019-10-09T23:14:38.6074289Z == end clock drift check ==
2019-10-09T23:14:39.2703569Z ##[error]Bash exited with code '1'.
2019-10-09T23:14:39.2752420Z ##[section]Starting: Checkout
2019-10-09T23:14:39.2754804Z ==============================================================================
2019-10-09T23:14:39.2754861Z Task         : Get sources
2019-10-09T23:14:39.2754910Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
