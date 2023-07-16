plain
2019-10-13T23:38:35.6760634Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-13T23:38:35.6870093Z ##[command]git config gc.auto 0
2019-10-13T23:38:35.6932935Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-13T23:38:35.6992855Z ##[command]git config --get-all http.proxy
2019-10-13T23:38:35.7162683Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65294/merge:refs/remotes/pull/65294/merge
---
2019-10-14T00:40:07.8030682Z .................................................................................................... 1600/9175
2019-10-14T00:40:14.2316313Z .................................................................................................... 1700/9175
2019-10-14T00:40:26.6778057Z ......................i...............i............................................................. 1800/9175
2019-10-14T00:40:34.0136925Z .................................................................................................... 1900/9175
2019-10-14T00:40:48.8198990Z .............iiiii.................................................................................. 2000/9175
2019-10-14T00:40:59.3390521Z .................................................................................................... 2200/9175
2019-10-14T00:41:01.9633229Z .................................................................................................... 2300/9175
2019-10-14T00:41:07.5900858Z .................................................................................................... 2400/9175
2019-10-14T00:41:29.9198359Z .................................................................................................... 2500/9175
---
2019-10-14T00:44:29.8274627Z ....................i...............i............................................................... 4800/9175
2019-10-14T00:44:41.7384587Z .................................................................................................... 4900/9175
2019-10-14T00:44:48.2396741Z .................................................................................................... 5000/9175
2019-10-14T00:44:59.1503684Z .................................................................................................... 5100/9175
2019-10-14T00:45:05.5319276Z ....................ii.ii........................................................................... 5200/9175
2019-10-14T00:45:16.2358033Z .................................................................................................... 5400/9175
2019-10-14T00:45:26.4613172Z ........................................................................................i........... 5500/9175
2019-10-14T00:45:34.7260214Z .................................................................................................... 5600/9175
2019-10-14T00:45:39.9390237Z .................................................................................................... 5700/9175
2019-10-14T00:45:39.9390237Z .................................................................................................... 5700/9175
2019-10-14T00:45:50.7948554Z .....................................................................................ii...i..ii..... 5800/9175
2019-10-14T00:46:16.0253550Z .................................................................................................... 6000/9175
2019-10-14T00:46:26.2531525Z .................................................................................................... 6100/9175
2019-10-14T00:46:26.2531525Z .................................................................................................... 6100/9175
2019-10-14T00:46:35.3989152Z ............................................................................................i..ii... 6200/9175
2019-10-14T00:47:05.2018432Z .................................................................................................... 6400/9175
2019-10-14T00:47:10.4920845Z .....................................................i.............................................. 6500/9175
2019-10-14T00:47:12.7341639Z .................................................................................................... 6600/9175
2019-10-14T00:47:15.2710733Z ..........................i......................................................................... 6700/9175
---
2019-10-14T00:48:21.8197319Z ...................................................................................FFF.............. 7400/9175
2019-10-14T00:48:28.6921620Z .................................................................................................... 7500/9175
2019-10-14T00:48:39.1873626Z .................................................................................................... 7600/9175
2019-10-14T00:48:48.8141059Z .................................................................................................... 7700/9175
2019-10-14T00:48:55.7796659Z ..ii......i......................................................................................... 7800/9175
2019-10-14T00:49:18.3120744Z .................................................................................................... 8000/9175
2019-10-14T00:49:27.6418504Z .................................................................................................... 8100/9175
2019-10-14T00:49:35.9199895Z .................................................................................................... 8200/9175
2019-10-14T00:50:16.5690708Z .................................................................................................... 8300/9175
---
2019-10-14T00:51:31.5772980Z +   --> $DIR/error-with-trait-decl.rs:4:5
2019-10-14T00:51:31.5773178Z +    |
2019-10-14T00:51:31.5773343Z + LL |     #[track_caller]
2019-10-14T00:51:31.5773484Z +    |     ^^^^^^^^^^^^^^^
2019-10-14T00:51:31.5773623Z + LL |     fn unwrap(&self);
2019-10-14T00:51:31.5774029Z +    |     ----------------- not a function
2019-10-14T00:51:31.5774371Z 9 error[E0738]: `#[track_caller]` is not supported in trait declarations.
2019-10-14T00:51:31.5774772Z 10   --> $DIR/error-with-trait-decl.rs:4:5
2019-10-14T00:51:31.5774967Z 11    |
2019-10-14T00:51:31.5775087Z 
---
2019-10-14T00:51:31.5777155Z 18 
2019-10-14T00:51:31.5777294Z 
2019-10-14T00:51:31.5777419Z 
2019-10-14T00:51:31.5777563Z The actual stderr differed from the expected stderr.
2019-10-14T00:51:31.5778050Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/error-with-trait-decl/error-with-trait-decl.stderr
2019-10-14T00:51:31.5778514Z To update references, rerun the tests and pass the `--bless` flag
2019-10-14T00:51:31.5778993Z To only update this specific test, also pass `--test-args rfc-2091-track-caller/error-with-trait-decl.rs`
2019-10-14T00:51:31.5779333Z error: 1 errors occurred comparing output.
2019-10-14T00:51:31.5779475Z status: exit code: 1
2019-10-14T00:51:31.5779475Z status: exit code: 1
2019-10-14T00:51:31.5780572Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2091-track-caller/error-with-trait-decl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/error-with-trait-decl" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/error-with-trait-decl/auxiliary" "-A" "unused"
2019-10-14T00:51:31.5781674Z ------------------------------------------
2019-10-14T00:51:31.5781930Z 
2019-10-14T00:51:31.5782390Z ------------------------------------------
2019-10-14T00:51:31.5782592Z stderr:
2019-10-14T00:51:31.5782592Z stderr:
2019-10-14T00:51:31.5782961Z ------------------------------------------
2019-10-14T00:51:31.5783177Z warning: the feature `track_caller` is incomplete and may cause the compiler to crash
2019-10-14T00:51:31.5783613Z   --> /checkout/src/test/ui/rfc-2091-track-caller/error-with-trait-decl.rs:1:12
2019-10-14T00:51:31.5783830Z    |
2019-10-14T00:51:31.5783998Z LL | #![feature(track_caller)] //~ WARN the feature `track_caller` is incomplete
2019-10-14T00:51:31.5784294Z    |
2019-10-14T00:51:31.5784438Z    = note: `#[warn(incomplete_features)]` on by default
2019-10-14T00:51:31.5784560Z 
2019-10-14T00:51:31.5784711Z error[E0739]: attribute should be applied to function
2019-10-14T00:51:31.5784711Z error[E0739]: attribute should be applied to function
2019-10-14T00:51:31.5785140Z   --> /checkout/src/test/ui/rfc-2091-track-caller/error-with-trait-decl.rs:4:5
2019-10-14T00:51:31.5785329Z    |
2019-10-14T00:51:31.5785484Z LL |     #[track_caller]
2019-10-14T00:51:31.5785626Z    |     ^^^^^^^^^^^^^^^
2019-10-14T00:51:31.5785766Z LL |     fn unwrap(&self);
2019-10-14T00:51:31.5786143Z    |     ----------------- not a function
2019-10-14T00:51:31.5786458Z error[E0738]: `#[track_caller]` is not supported in trait declarations.
2019-10-14T00:51:31.5786874Z   --> /checkout/src/test/ui/rfc-2091-track-caller/error-with-trait-decl.rs:4:5
2019-10-14T00:51:31.5787075Z    |
2019-10-14T00:51:31.5787214Z LL |     #[track_caller]
---
2019-10-14T00:51:31.5791933Z +   --> $DIR/error-with-trait-default-impl.rs:4:5
2019-10-14T00:51:31.5792210Z +    |
2019-10-14T00:51:31.5792352Z + LL |     #[track_caller]
2019-10-14T00:51:31.5792506Z +    |     ^^^^^^^^^^^^^^^
2019-10-14T00:51:31.5792646Z + LL |     fn unwrap(&self) {}
2019-10-14T00:51:31.5793028Z +    |     ------------------- not a function
2019-10-14T00:51:31.5793399Z 9 error[E0738]: `#[track_caller]` is not supported in trait declarations.
2019-10-14T00:51:31.5793781Z 10   --> $DIR/error-with-trait-default-impl.rs:4:5
2019-10-14T00:51:31.5793998Z 11    |
2019-10-14T00:51:31.5794119Z 
---
2019-10-14T00:51:31.5795888Z 18 
2019-10-14T00:51:31.5796009Z 
2019-10-14T00:51:31.5796127Z 
2019-10-14T00:51:31.5796268Z The actual stderr differed from the expected stderr.
2019-10-14T00:51:31.5796772Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/error-with-trait-default-impl/error-with-trait-default-impl.stderr
2019-10-14T00:51:31.5797359Z To update references, rerun the tests and pass the `--bless` flag
2019-10-14T00:51:31.5798067Z To only update this specific test, also pass `--test-args rfc-2091-track-caller/error-with-trait-default-impl.rs`
2019-10-14T00:51:31.5799422Z error: 1 errors occurred comparing output.
2019-10-14T00:51:31.5799467Z status: exit code: 1
2019-10-14T00:51:31.5799467Z status: exit code: 1
2019-10-14T00:51:31.5800371Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2091-track-caller/error-with-trait-default-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/error-with-trait-default-impl" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/error-with-trait-default-impl/auxiliary" "-A" "unused"
2019-10-14T00:51:31.5800739Z ------------------------------------------
2019-10-14T00:51:31.5800790Z 
2019-10-14T00:51:31.5801014Z ------------------------------------------
2019-10-14T00:51:31.5801059Z stderr:
2019-10-14T00:51:31.5801059Z stderr:
2019-10-14T00:51:31.5801744Z ------------------------------------------
2019-10-14T00:51:31.5801824Z warning: the feature `track_caller` is incomplete and may cause the compiler to crash
2019-10-14T00:51:31.5802194Z   --> /checkout/src/test/ui/rfc-2091-track-caller/error-with-trait-default-impl.rs:1:12
2019-10-14T00:51:31.5802276Z    |
2019-10-14T00:51:31.5802325Z LL | #![feature(track_caller)] //~ WARN the feature `track_caller` is incomplete
2019-10-14T00:51:31.5802431Z    |
2019-10-14T00:51:31.5802476Z    = note: `#[warn(incomplete_features)]` on by default
2019-10-14T00:51:31.5802507Z 
2019-10-14T00:51:31.5802552Z error[E0739]: attribute should be applied to function
2019-10-14T00:51:31.5802552Z error[E0739]: attribute should be applied to function
2019-10-14T00:51:31.5802854Z   --> /checkout/src/test/ui/rfc-2091-track-caller/error-with-trait-default-impl.rs:4:5
2019-10-14T00:51:31.5802903Z    |
2019-10-14T00:51:31.5802944Z LL |     #[track_caller]
2019-10-14T00:51:31.5803004Z    |     ^^^^^^^^^^^^^^^
2019-10-14T00:51:31.5803048Z LL |     fn unwrap(&self) {}
2019-10-14T00:51:31.5803274Z    |     ------------------- not a function
2019-10-14T00:51:31.5803368Z error[E0738]: `#[track_caller]` is not supported in trait declarations.
2019-10-14T00:51:31.5803635Z   --> /checkout/src/test/ui/rfc-2091-track-caller/error-with-trait-default-impl.rs:4:5
2019-10-14T00:51:31.5803852Z    |
2019-10-14T00:51:31.5803911Z LL |     #[track_caller]
---
2019-10-14T00:51:31.5805457Z +   --> $DIR/error-with-trait-fn-impl.rs:8:5
2019-10-14T00:51:31.5805512Z +    |
2019-10-14T00:51:31.5805568Z + LL |     #[track_caller]
2019-10-14T00:51:31.5805610Z +    |     ^^^^^^^^^^^^^^^
2019-10-14T00:51:31.5805653Z + LL |     fn unwrap(&self) {}
2019-10-14T00:51:31.5805883Z +    |     ------------------- not a function
2019-10-14T00:51:31.5805988Z 9 error[E0738]: `#[track_caller]` is not supported in traits yet.
2019-10-14T00:51:31.5806312Z 10   --> $DIR/error-with-trait-fn-impl.rs:8:5
2019-10-14T00:51:31.5806387Z 11    |
2019-10-14T00:51:31.5806413Z 
---
2019-10-14T00:51:31.5807208Z 18 
2019-10-14T00:51:31.5807243Z 
2019-10-14T00:51:31.5807285Z 
2019-10-14T00:51:31.5807329Z The actual stderr differed from the expected stderr.
2019-10-14T00:51:31.5807666Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/error-with-trait-fn-impl/error-with-trait-fn-impl.stderr
2019-10-14T00:51:31.5807940Z To update references, rerun the tests and pass the `--bless` flag
2019-10-14T00:51:31.5808271Z To only update this specific test, also pass `--test-args rfc-2091-track-caller/error-with-trait-fn-impl.rs`
2019-10-14T00:51:31.5808376Z error: 1 errors occurred comparing output.
2019-10-14T00:51:31.5808421Z status: exit code: 1
2019-10-14T00:51:31.5808421Z status: exit code: 1
2019-10-14T00:51:31.5809234Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2091-track-caller/error-with-trait-fn-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/error-with-trait-fn-impl" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/error-with-trait-fn-impl/auxiliary" "-A" "unused"
2019-10-14T00:51:31.5809613Z ------------------------------------------
2019-10-14T00:51:31.5809664Z 
2019-10-14T00:51:31.5809909Z ------------------------------------------
2019-10-14T00:51:31.5809957Z stderr:
2019-10-14T00:51:31.5809957Z stderr:
2019-10-14T00:51:31.5810192Z ------------------------------------------
2019-10-14T00:51:31.5810263Z warning: the feature `track_caller` is incomplete and may cause the compiler to crash
2019-10-14T00:51:31.5810554Z   --> /checkout/src/test/ui/rfc-2091-track-caller/error-with-trait-fn-impl.rs:1:12
2019-10-14T00:51:31.5810622Z    |
2019-10-14T00:51:31.5810674Z LL | #![feature(track_caller)] //~ WARN the feature `track_caller` is incomplete
2019-10-14T00:51:31.5810876Z    |
2019-10-14T00:51:31.5810941Z    = note: `#[warn(incomplete_features)]` on by default
2019-10-14T00:51:31.5810973Z 
2019-10-14T00:51:31.5811019Z error[E0739]: attribute should be applied to function
2019-10-14T00:51:31.5811019Z error[E0739]: attribute should be applied to function
2019-10-14T00:51:31.5811741Z   --> /checkout/src/test/ui/rfc-2091-track-caller/error-with-trait-fn-impl.rs:8:5
2019-10-14T00:51:31.5811819Z    |
2019-10-14T00:51:31.5811862Z LL |     #[track_caller]
2019-10-14T00:51:31.5811904Z    |     ^^^^^^^^^^^^^^^
2019-10-14T00:51:31.5811962Z LL |     fn unwrap(&self) {}
2019-10-14T00:51:31.5812260Z    |     ------------------- not a function
2019-10-14T00:51:31.5812357Z error[E0738]: `#[track_caller]` is not supported in traits yet.
2019-10-14T00:51:31.5812627Z   --> /checkout/src/test/ui/rfc-2091-track-caller/error-with-trait-fn-impl.rs:8:5
2019-10-14T00:51:31.5812675Z    |
2019-10-14T00:51:31.5812716Z LL |     #[track_caller]
---
2019-10-14T00:51:31.5833604Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-14T00:51:31.5833706Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-14T00:51:31.5835500Z 
2019-10-14T00:51:31.5835574Z 
2019-10-14T00:51:31.5837513Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-14T00:51:31.5837814Z 
2019-10-14T00:51:31.5837843Z 
2019-10-14T00:51:31.5845178Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-14T00:51:31.5845279Z Build completed unsuccessfully in 1:06:29
2019-10-14T00:51:31.5845279Z Build completed unsuccessfully in 1:06:29
2019-10-14T00:51:31.5899347Z == clock drift check ==
2019-10-14T00:51:31.5920785Z   local time: Mon Oct 14 00:51:31 UTC 2019
2019-10-14T00:51:31.8700156Z   network time: Mon, 14 Oct 2019 00:51:31 GMT
2019-10-14T00:51:31.8703886Z == end clock drift check ==
2019-10-14T00:51:32.3063043Z ##[error]Bash exited with code '1'.
2019-10-14T00:51:32.3111386Z ##[section]Starting: Checkout
2019-10-14T00:51:32.3130585Z ==============================================================================
2019-10-14T00:51:32.3130644Z Task         : Get sources
2019-10-14T00:51:32.3130708Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
