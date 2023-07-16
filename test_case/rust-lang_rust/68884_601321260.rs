plain
2020-03-19T16:30:19.5577931Z ========================== Starting Command Output ===========================
2020-03-19T16:30:19.5583127Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c355445c-19ea-4dac-b6b5-99b3ded60f75.sh
2020-03-19T16:30:19.5583433Z 
2020-03-19T16:30:19.5588543Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-19T16:30:19.5609076Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68884/merge to s
2020-03-19T16:30:19.5612612Z Task         : Get sources
2020-03-19T16:30:19.5612939Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-19T16:30:19.5613255Z Version      : 1.0.0
2020-03-19T16:30:19.5613492Z Author       : Microsoft
---
2020-03-19T16:30:20.5483113Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-19T16:30:20.5489712Z ##[command]git config gc.auto 0
2020-03-19T16:30:20.5494062Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-19T16:30:20.5497979Z ##[command]git config --get-all http.proxy
2020-03-19T16:30:20.5504877Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68884/merge:refs/remotes/pull/68884/merge
---
2020-03-19T17:31:51.3419318Z .................................................................................................... 1700/9804
2020-03-19T17:31:56.0377872Z .................................................................................................... 1800/9804
2020-03-19T17:32:08.3587604Z ...........................................................................i........................ 1900/9804
2020-03-19T17:32:15.3585616Z .................................................................................................... 2000/9804
2020-03-19T17:32:23.7722237Z .................................................................iiiii.............................. 2100/9804
2020-03-19T17:32:42.6507392Z .................................................................................................... 2300/9804
2020-03-19T17:32:45.0072031Z .................................................................................................... 2400/9804
2020-03-19T17:32:48.0284865Z .................................................................................................... 2500/9804
2020-03-19T17:33:09.2663585Z .................................................................................................... 2600/9804
---
2020-03-19T17:35:54.3353676Z ......................................i...............i............................................. 5000/9804
2020-03-19T17:36:03.6895183Z .................................................................................................... 5100/9804
2020-03-19T17:36:10.4413401Z .................................................................................i.................. 5200/9804
2020-03-19T17:36:16.1874467Z .................................................................................................... 5300/9804
2020-03-19T17:36:26.7583767Z ..............................................................ii.ii........i...i.................... 5400/9804
2020-03-19T17:36:35.0743703Z .i.................................................................................................. 5600/9804
2020-03-19T17:36:44.8156398Z ......i............................................................................................. 5700/9804
2020-03-19T17:36:51.2047395Z .........................................................i.......................................... 5800/9804
2020-03-19T17:36:58.0041112Z .................................................................................................... 5900/9804
2020-03-19T17:36:58.0041112Z .................................................................................................... 5900/9804
2020-03-19T17:37:06.0515707Z .................................................................................................... 6000/9804
2020-03-19T17:37:14.2260363Z ...................................................ii...i..ii...........i........................... 6100/9804
2020-03-19T17:37:34.6818045Z .................................................................................................... 6300/9804
2020-03-19T17:37:41.8232537Z .................................................................................................... 6400/9804
2020-03-19T17:37:41.8232537Z .................................................................................................... 6400/9804
2020-03-19T17:37:49.5647512Z .................................................................................i..ii.............. 6500/9804
2020-03-19T17:38:18.3798810Z .................................................................................................... 6700/9804
2020-03-19T17:38:28.2858154Z ................................................................................i................... 6800/9804
2020-03-19T17:38:30.4263889Z .................................................................................................... 6900/9804
2020-03-19T17:38:32.5493283Z .................................................................................................... 7000/9804
---
2020-03-19T17:40:21.7040686Z .................................................................................................... 7800/9804
2020-03-19T17:40:27.5218754Z .................................................................................................... 7900/9804
2020-03-19T17:40:33.8303794Z ...................................................................i................................ 8000/9804
2020-03-19T17:40:44.7660213Z .................................................................................................... 8100/9804
2020-03-19T17:40:50.6718559Z ................iiiiiiiiii.i........................................................................ 8200/9804
2020-03-19T17:41:05.3202211Z .................................................................................................... 8400/9804
2020-03-19T17:41:11.7745948Z .................................................................................................... 8500/9804
2020-03-19T17:41:27.1368049Z .................................................................................................... 8600/9804
2020-03-19T17:41:34.2069468Z .................................................................................................... 8700/9804
---
2020-03-19T17:43:33.0897843Z 
2020-03-19T17:43:33.0898012Z 34 LL | | }
2020-03-19T17:43:33.0898186Z 35    | |_^
2020-03-19T17:43:33.0898349Z 36    |
2020-03-19T17:43:33.0898888Z -    = note: hidden type `(&u8, &u8)` captures lifetime '_#8r
2020-03-19T17:43:33.0899528Z +    = note: hidden type `(&u8, &u8)` captures lifetime '_#4r
2020-03-19T17:43:33.0900128Z 39 error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2020-03-19T17:43:33.0900814Z 40   --> $DIR/ret-impl-trait-no-fg.rs:9:1
2020-03-19T17:43:33.0901038Z 
2020-03-19T17:43:33.0901191Z 48 LL | | }
2020-03-19T17:43:33.0901191Z 48 LL | | }
2020-03-19T17:43:33.0901380Z 49    | |_^
2020-03-19T17:43:33.0901543Z 50    |
2020-03-19T17:43:33.0907101Z -    = note: hidden type `(&u8, &u8)` captures lifetime '_#9r
2020-03-19T17:43:33.0907907Z +    = note: hidden type `(&u8, &u8)` captures lifetime '_#5r
2020-03-19T17:43:33.0908432Z 53 error: aborting due to 5 previous errors
2020-03-19T17:43:33.0908654Z 54 
2020-03-19T17:43:33.0908789Z 
2020-03-19T17:43:33.0908899Z 
2020-03-19T17:43:33.0908899Z 
2020-03-19T17:43:33.0909130Z The actual stderr differed from the expected stderr.
2020-03-19T17:43:33.0910000Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/ret-impl-trait-no-fg/ret-impl-trait-no-fg.stderr
2020-03-19T17:43:33.0910823Z To update references, rerun the tests and pass the `--bless` flag
2020-03-19T17:43:33.0911844Z To only update this specific test, also pass `--test-args async-await/multiple-lifetimes/ret-impl-trait-no-fg.rs`
2020-03-19T17:43:33.0918478Z error: 1 errors occurred comparing output.
2020-03-19T17:43:33.0918745Z status: exit code: 1
2020-03-19T17:43:33.0918745Z status: exit code: 1
2020-03-19T17:43:33.0921415Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/multiple-lifetimes/ret-impl-trait-no-fg.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/ret-impl-trait-no-fg" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/ret-impl-trait-no-fg/auxiliary"
2020-03-19T17:43:33.0923725Z ------------------------------------------
2020-03-19T17:43:33.0923935Z 
2020-03-19T17:43:33.0924362Z ------------------------------------------
2020-03-19T17:43:33.0924608Z stderr:
2020-03-19T17:43:33.0924608Z stderr:
2020-03-19T17:43:33.0925032Z ------------------------------------------
2020-03-19T17:43:33.0925341Z error: ambiguous lifetime bound in `impl Trait`
2020-03-19T17:43:33.0926024Z   --> /checkout/src/test/ui/async-await/multiple-lifetimes/ret-impl-trait-no-fg.rs:9:64
2020-03-19T17:43:33.0926353Z    |
2020-03-19T17:43:33.0926954Z LL | async fn async_ret_impl_trait<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a, 'b> {
2020-03-19T17:43:33.0927844Z    |                                                                ^^^^^^^^^^^^^^^^^^ neither `'a` nor `'b` outlives the other
2020-03-19T17:43:33.0929235Z    |
2020-03-19T17:43:33.0929553Z    = help: add #![feature(member_constraints)] to the crate attributes to enable
2020-03-19T17:43:33.0930089Z error: ambiguous lifetime bound in `impl Trait`
2020-03-19T17:43:33.0931034Z   --> /checkout/src/test/ui/async-await/multiple-lifetimes/ret-impl-trait-no-fg.rs:9:64
2020-03-19T17:43:33.0931399Z    |
2020-03-19T17:43:33.0931399Z    |
2020-03-19T17:43:33.0932036Z LL | async fn async_ret_impl_trait<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a, 'b> {
2020-03-19T17:43:33.0932905Z    |                                                                ^^^^^^^^^^^^^^^^^^ neither `'a` nor `'b` outlives the other
2020-03-19T17:43:33.0933306Z    |
2020-03-19T17:43:33.0933611Z    = help: add #![feature(member_constraints)] to the crate attributes to enable
2020-03-19T17:43:33.0934107Z error: ambiguous lifetime bound in `impl Trait`
2020-03-19T17:43:33.0934788Z   --> /checkout/src/test/ui/async-await/multiple-lifetimes/ret-impl-trait-no-fg.rs:9:64
2020-03-19T17:43:33.0935112Z    |
2020-03-19T17:43:33.0935112Z    |
2020-03-19T17:43:33.0935869Z LL | async fn async_ret_impl_trait<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a, 'b> {
2020-03-19T17:43:33.0936520Z    |                                                                ^^^^^^^^^^^^^^^^^^ the elided lifetimes here do not outlive one another
2020-03-19T17:43:33.0936926Z    |
2020-03-19T17:43:33.0937245Z    = help: add #![feature(member_constraints)] to the crate attributes to enable
2020-03-19T17:43:33.0937832Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2020-03-19T17:43:33.0938614Z   --> /checkout/src/test/ui/async-await/multiple-lifetimes/ret-impl-trait-no-fg.rs:9:1
2020-03-19T17:43:33.0938956Z    |
2020-03-19T17:43:33.0938956Z    |
2020-03-19T17:43:33.0939552Z LL | / async fn async_ret_impl_trait<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a, 'b> {
2020-03-19T17:43:33.0939981Z LL | |     //~^ ERROR ambiguous lifetime bound
2020-03-19T17:43:33.0940310Z LL | |     //~| ERROR ambiguous lifetime bound
2020-03-19T17:43:33.0940619Z LL | |     //~| ERROR ambiguous lifetime bound
2020-03-19T17:43:33.0941039Z LL | |     (a, b)
2020-03-19T17:43:33.0941229Z LL | | }
2020-03-19T17:43:33.0941387Z    | |_^
2020-03-19T17:43:33.0941553Z    |
2020-03-19T17:43:33.0941553Z    |
2020-03-19T17:43:33.0942054Z    = note: hidden type `(&u8, &u8)` captures lifetime '_#4r
2020-03-19T17:43:33.0942611Z error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
2020-03-19T17:43:33.0943380Z   --> /checkout/src/test/ui/async-await/multiple-lifetimes/ret-impl-trait-no-fg.rs:9:1
2020-03-19T17:43:33.0943700Z    |
2020-03-19T17:43:33.0943700Z    |
2020-03-19T17:43:33.0944291Z LL | / async fn async_ret_impl_trait<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a, 'b> {
2020-03-19T17:43:33.0944740Z LL | |     //~^ ERROR ambiguous lifetime bound
2020-03-19T17:43:33.0945050Z LL | |     //~| ERROR ambiguous lifetime bound
2020-03-19T17:43:33.0945356Z LL | |     //~| ERROR ambiguous lifetime bound
2020-03-19T17:43:33.0945775Z LL | |     (a, b)
2020-03-19T17:43:33.0945952Z LL | | }
2020-03-19T17:43:33.0946127Z    | |_^
2020-03-19T17:43:33.0946405Z    |
2020-03-19T17:43:33.0946405Z    |
2020-03-19T17:43:33.0946924Z    = note: hidden type `(&u8, &u8)` captures lifetime '_#5r
2020-03-19T17:43:33.0947401Z error: aborting due to 5 previous errors
2020-03-19T17:43:33.0947593Z 
2020-03-19T17:43:33.0948089Z For more information about this error, try `rustc --explain E0700`.
2020-03-19T17:43:33.0948334Z 
---
2020-03-19T17:43:33.0951614Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-19T17:43:33.0952084Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-19T17:43:33.0952375Z 
2020-03-19T17:43:33.0952482Z 
2020-03-19T17:43:33.0956831Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-19T17:43:33.0959936Z 
2020-03-19T17:43:33.0960048Z 
2020-03-19T17:43:33.0960657Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-19T17:43:33.0961095Z Build completed unsuccessfully in 1:08:52
2020-03-19T17:43:33.0961095Z Build completed unsuccessfully in 1:08:52
2020-03-19T17:43:33.0991796Z == clock drift check ==
2020-03-19T17:43:33.1010378Z   local time: Thu Mar 19 17:43:33 UTC 2020
2020-03-19T17:43:33.2716118Z   network time: Thu, 19 Mar 2020 17:43:33 GMT
2020-03-19T17:43:33.2721741Z == end clock drift check ==
2020-03-19T17:43:33.7136798Z 
2020-03-19T17:43:33.7202310Z ##[error]Bash exited with code '1'.
2020-03-19T17:43:33.7217933Z ##[section]Finishing: Run build
2020-03-19T17:43:33.7276331Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68884/merge to s
2020-03-19T17:43:33.7281874Z Task         : Get sources
2020-03-19T17:43:33.7282285Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-19T17:43:33.7282648Z Version      : 1.0.0
2020-03-19T17:43:33.7283343Z Author       : Microsoft
2020-03-19T17:43:33.7283343Z Author       : Microsoft
2020-03-19T17:43:33.7283765Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-19T17:43:33.7284243Z ==============================================================================
2020-03-19T17:43:34.1080935Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-19T17:43:34.1128569Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68884/merge to s
2020-03-19T17:43:34.1224032Z Cleaning up task key
2020-03-19T17:43:34.1226144Z Start cleaning up orphan processes.
2020-03-19T17:43:34.1442807Z Terminate orphan process: pid (3270) (python)
2020-03-19T17:43:34.1618637Z ##[section]Finishing: Finalize Job
