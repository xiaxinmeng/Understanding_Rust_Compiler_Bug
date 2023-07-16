plain
2019-11-12T12:51:45.5156390Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-12T12:51:45.5339997Z ##[command]git config gc.auto 0
2019-11-12T12:51:45.5408615Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-12T12:51:45.5475721Z ##[command]git config --get-all http.proxy
2019-11-12T12:51:45.5619110Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66294/merge:refs/remotes/pull/66294/merge
---
2019-11-12T13:49:32.0490154Z .................................................................................................... 1400/9232
2019-11-12T13:49:38.1690294Z .................................................................................................... 1500/9232
2019-11-12T13:49:43.8917159Z .................................................................................................... 1600/9232
2019-11-12T13:49:52.5397960Z .................................................................................................... 1700/9232
2019-11-12T13:50:00.5293883Z ..i................................................................................................. 1800/9232
2019-11-12T13:50:06.9325763Z ......................................................................................iiii.i........ 1900/9232
2019-11-12T13:50:27.3757889Z .................................................................................................... 2100/9232
2019-11-12T13:50:29.6398254Z .................................................................................................... 2200/9232
2019-11-12T13:50:32.0716358Z .................................................................................................... 2300/9232
2019-11-12T13:50:41.5726347Z .................................................................................................... 2400/9232
2019-11-12T13:50:41.5726347Z .................................................................................................... 2400/9232
2019-11-12T13:50:58.3850604Z .................................................................................................... 2500/9232
2019-11-12T13:51:02.0556389Z .................................................................................................... 2600/9232
2019-11-12T13:51:12.4774358Z .....................................................i.............................................. 2700/9232
2019-11-12T13:51:19.4830703Z .................................................................................................... 2800/9232
2019-11-12T13:51:26.2101592Z ....................................................i............................................... 2900/9232
2019-11-12T13:51:33.8331026Z .................................................................................................... 3000/9232
2019-11-12T13:51:39.8679337Z .................................................................................................... 3100/9232
2019-11-12T13:51:46.0326480Z ..................................ii.................F...........................................F.. 3200/9232
2019-11-12T13:52:01.2144444Z .................................................................................................... 3400/9232
2019-11-12T13:52:08.8060842Z ..........................i..............................................i.......................... 3500/9232
2019-11-12T13:52:14.3087739Z .................................................................................................... 3600/9232
2019-11-12T13:52:18.8051181Z .................................................................................................... 3700/9232
---
2019-11-12T13:53:28.3129264Z ...................................................................................i...............i 4700/9232
2019-11-12T13:53:34.8299964Z .................................................................................................... 4800/9232
2019-11-12T13:53:43.4499893Z .................................................................................................... 4900/9232
2019-11-12T13:53:48.3489408Z .................................................................................................... 5000/9232
2019-11-12T13:53:59.8331674Z ......................................................................................ii.ii......... 5100/9232
2019-11-12T13:54:02.9317399Z ..i................................................................................................. 5200/9232
2019-11-12T13:54:15.4619078Z .................................................................................................... 5400/9232
2019-11-12T13:54:23.3653305Z ....................................................................i............................... 5500/9232
2019-11-12T13:54:30.3380379Z .................................................................................................... 5600/9232
2019-11-12T13:54:37.6258231Z .................................................................................................... 5700/9232
2019-11-12T13:54:37.6258231Z .................................................................................................... 5700/9232
2019-11-12T13:54:46.3879429Z .....................................................ii...i..ii...........i......................... 5800/9232
2019-11-12T13:55:08.5325407Z .................................................................................................... 6000/9232
2019-11-12T13:55:16.5568357Z .................................................................................................... 6100/9232
2019-11-12T13:55:16.5568357Z .................................................................................................... 6100/9232
2019-11-12T13:55:22.9260675Z ........................................................................i..ii....................... 6200/9232
2019-11-12T13:55:51.4338839Z .................................................................................................... 6400/9232
2019-11-12T13:55:53.4994501Z ........................................i........................................................... 6500/9232
2019-11-12T13:55:55.5526591Z .................................................................................................... 6600/9232
2019-11-12T13:55:57.6933921Z ........................i........................................................................... 6700/9232
---
2019-11-12T14:00:37.9252253Z - error[E0080]: evaluation of constant value failed
2019-11-12T14:00:37.9252610Z + error[E0391]: cycle detected when const-evaluating `a`
2019-11-12T14:00:37.9252945Z 2   --> $DIR/infinite-recursion-const-fn.rs:3:25
2019-11-12T14:00:37.9253115Z 3    |
2019-11-12T14:00:37.9253415Z 4 LL | const fn a() -> usize { b() }
2019-11-12T14:00:37.9253700Z 5    |                         ^^^
2019-11-12T14:00:37.9253991Z -    |                         |
2019-11-12T14:00:37.9254351Z -    |                         reached the configured maximum number of stack frames
2019-11-12T14:00:37.9254351Z -    |                         reached the configured maximum number of stack frames
2019-11-12T14:00:37.9254759Z -    |                         inside call to `b` at $DIR/infinite-recursion-const-fn.rs:3:25
2019-11-12T14:00:37.9254914Z +    |
2019-11-12T14:00:37.9255232Z + note: ...which requires const-evaluating `b`...
2019-11-12T14:00:37.9255937Z +   --> $DIR/infinite-recursion-const-fn.rs:4:25
2019-11-12T14:00:37.9256568Z +    |
2019-11-12T14:00:37.9257020Z 9 LL | const fn b() -> usize { a() }
2019-11-12T14:00:37.9257802Z -    |                         |
2019-11-12T14:00:37.9258276Z -    |                         inside call to `a` at $DIR/infinite-recursion-const-fn.rs:4:25
2019-11-12T14:00:37.9258724Z -    |                         inside call to `a` at $DIR/infinite-recursion-const-fn.rs:4:25
2019-11-12T14:00:37.9259216Z -    |                         inside call to `a` at $DIR/infinite-recursion-const-fn.rs:4:25
---
2019-11-12T14:00:37.9279048Z -    |                         inside call to `a` at $DIR/infinite-recursion-const-fn.rs:4:25
2019-11-12T14:00:37.9279395Z -    |                         inside call to `a` at $DIR/infinite-recursion-const-fn.rs:4:25
2019-11-12T14:00:37.9279689Z -    |                         inside call to `a` at $DIR/infinite-recursion-const-fn.rs:4:25
2019-11-12T14:00:37.9279743Z +    |                         ^^^
2019-11-12T14:00:37.9280033Z +    = note: ...which again requires const-evaluating `a`, completing the cycle
2019-11-12T14:00:37.9280287Z + note: cycle used when const-evaluating `ARR::{{constant}}#0`
2019-11-12T14:00:37.9280525Z +   --> $DIR/infinite-recursion-const-fn.rs:5:18
2019-11-12T14:00:37.9280602Z +    |
2019-11-12T14:00:37.9280646Z 61 LL | const ARR: [i32; a()] = [5; 6];
2019-11-12T14:00:37.9281090Z -    |                  --- inside call to `a` at $DIR/infinite-recursion-const-fn.rs:5:18
2019-11-12T14:00:37.9281201Z 63 
2019-11-12T14:00:37.9281406Z 64 error: aborting due to previous error
2019-11-12T14:00:37.9281463Z 65 
2019-11-12T14:00:37.9281654Z 
2019-11-12T14:00:37.9281654Z 
2019-11-12T14:00:37.9282196Z - For more information about this error, try `rustc --explain E0080`.
2019-11-12T14:00:37.9282413Z + For more information about this error, try `rustc --explain E0391`.
2019-11-12T14:00:37.9282471Z 67 
2019-11-12T14:00:37.9282493Z 
2019-11-12T14:00:37.9282513Z 
2019-11-12T14:00:37.9282549Z The actual stderr differed from the expected stderr.
2019-11-12T14:00:37.9282845Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-recursion-const-fn/infinite-recursion-const-fn.stderr
2019-11-12T14:00:37.9283059Z To update references, rerun the tests and pass the `--bless` flag
2019-11-12T14:00:37.9283470Z To only update this specific test, also pass `--test-args infinite/infinite-recursion-const-fn.rs`
2019-11-12T14:00:37.9283553Z error: 1 errors occurred comparing output.
2019-11-12T14:00:37.9283587Z status: exit code: 1
2019-11-12T14:00:37.9283587Z status: exit code: 1
2019-11-12T14:00:37.9284346Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-recursion-const-fn" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-recursion-const-fn/auxiliary" "-A" "unused"
2019-11-12T14:00:37.9284666Z ------------------------------------------
2019-11-12T14:00:37.9284694Z 
2019-11-12T14:00:37.9284871Z ------------------------------------------
2019-11-12T14:00:37.9284926Z stderr:
2019-11-12T14:00:37.9284926Z stderr:
2019-11-12T14:00:37.9285101Z ------------------------------------------
2019-11-12T14:00:37.9285289Z error[E0391]: cycle detected when const-evaluating `a`
2019-11-12T14:00:37.9285496Z   --> /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:3:25
2019-11-12T14:00:37.9285554Z    |
2019-11-12T14:00:37.9286206Z LL | const fn a() -> usize { b() } //~ ERROR evaluation of constant value failed
2019-11-12T14:00:37.9286333Z    |
2019-11-12T14:00:37.9286333Z    |
2019-11-12T14:00:37.9286573Z note: ...which requires const-evaluating `b`...
2019-11-12T14:00:37.9286909Z    |
2019-11-12T14:00:37.9286909Z    |
2019-11-12T14:00:37.9287127Z LL | const fn b() -> usize { a() }
2019-11-12T14:00:37.9287177Z    |                         ^^^
2019-11-12T14:00:37.9287603Z    = note: ...which again requires const-evaluating `a`, completing the cycle
2019-11-12T14:00:37.9287860Z note: cycle used when const-evaluating `ARR::{{constant}}#0`
2019-11-12T14:00:37.9288191Z    |
2019-11-12T14:00:37.9288191Z    |
2019-11-12T14:00:37.9288235Z LL | const ARR: [i32; a()] = [5; 6];
2019-11-12T14:00:37.9288311Z 
2019-11-12T14:00:37.9288381Z error: aborting due to previous error
2019-11-12T14:00:37.9288410Z 
2019-11-12T14:00:37.9288669Z For more information about this error, try `rustc --explain E0391`.
---
2019-11-12T14:00:37.9289462Z ---- [ui] ui/invalid_const_promotion.rs stdout ----
2019-11-12T14:00:37.9289489Z 
2019-11-12T14:00:37.9289694Z error: test compilation failed although it shouldn't!
2019-11-12T14:00:37.9289742Z status: exit code: 1
2019-11-12T14:00:37.9290538Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/invalid_const_promotion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/invalid_const_promotion/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "debug_assertions=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/invalid_const_promotion/auxiliary"
2019-11-12T14:00:37.9290810Z ------------------------------------------
2019-11-12T14:00:37.9290837Z 
2019-11-12T14:00:37.9291013Z ------------------------------------------
2019-11-12T14:00:37.9291049Z stderr:
2019-11-12T14:00:37.9291049Z stderr:
2019-11-12T14:00:37.9291244Z ------------------------------------------
2019-11-12T14:00:37.9291284Z error[E0080]: evaluation of constant value failed
2019-11-12T14:00:37.9291483Z   --> /checkout/src/test/ui/invalid_const_promotion.rs:22:27
2019-11-12T14:00:37.9291540Z    |
2019-11-12T14:00:37.9291716Z LL | const fn bar() -> usize { 0 - 1 }
2019-11-12T14:00:37.9291759Z    |                           ^^^^^ attempt to subtract with overflow
2019-11-12T14:00:37.9291836Z error[E0080]: evaluation of constant expression failed
2019-11-12T14:00:37.9292469Z   --> /checkout/src/test/ui/invalid_const_promotion.rs:25:25
2019-11-12T14:00:37.9292515Z    |
2019-11-12T14:00:37.9292730Z LL |     let _: &'static _ = &bar();
---
2019-11-12T14:00:37.9294150Z ---- [ui] ui/union/union-nodrop.rs stdout ----
2019-11-12T14:00:37.9294178Z 
2019-11-12T14:00:37.9294552Z error: test compilation failed although it shouldn't!
2019-11-12T14:00:37.9294612Z status: exit code: 101
2019-11-12T14:00:37.9295243Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/union/union-nodrop.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-nodrop/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-nodrop/auxiliary"
2019-11-12T14:00:37.9295626Z ------------------------------------------
2019-11-12T14:00:37.9295655Z 
2019-11-12T14:00:37.9296421Z ------------------------------------------
2019-11-12T14:00:37.9296494Z stderr:
2019-11-12T14:00:37.9296494Z stderr:
2019-11-12T14:00:37.9296725Z ------------------------------------------
2019-11-12T14:00:37.9297029Z thread 'rustc' panicked at 'assertion failed: body.arg_count == 0', src/librustc_mir/const_eval.rs:149:5
2019-11-12T14:00:37.9297162Z 
2019-11-12T14:00:37.9297208Z error: internal compiler error: unexpected panic
2019-11-12T14:00:37.9297237Z 
2019-11-12T14:00:37.9297301Z note: the compiler unexpectedly panicked. this is a bug.
2019-11-12T14:00:37.9297301Z note: the compiler unexpectedly panicked. this is a bug.
2019-11-12T14:00:37.9297333Z 
2019-11-12T14:00:37.9297764Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-11-12T14:00:37.9298086Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-11-12T14:00:37.9298130Z 
2019-11-12T14:00:37.9298130Z 
2019-11-12T14:00:37.9298448Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2019-11-12T14:00:37.9298512Z 
2019-11-12T14:00:37.9298737Z ------------------------------------------
2019-11-12T14:00:37.9298770Z 
2019-11-12T14:00:37.9298816Z 
---
2019-11-12T14:00:37.9313490Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-12T14:00:37.9313568Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-12T14:00:37.9313650Z 
2019-11-12T14:00:37.9313673Z 
2019-11-12T14:00:37.9315402Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-12T14:00:37.9316156Z 
2019-11-12T14:00:37.9316196Z 
2019-11-12T14:00:37.9316246Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-12T14:00:37.9316316Z Build completed unsuccessfully in 1:02:26
2019-11-12T14:00:37.9316316Z Build completed unsuccessfully in 1:02:26
2019-11-12T14:00:37.9368836Z == clock drift check ==
2019-11-12T14:00:37.9385543Z   local time: Tue Nov 12 14:00:37 UTC 2019
2019-11-12T14:00:38.2232430Z   network time: Tue, 12 Nov 2019 14:00:38 GMT
2019-11-12T14:00:38.2232817Z == end clock drift check ==
2019-11-12T14:00:38.9616564Z 
2019-11-12T14:00:38.9723048Z ##[error]Bash exited with code '1'.
2019-11-12T14:00:38.9765439Z ##[section]Starting: Checkout
2019-11-12T14:00:38.9768074Z ==============================================================================
2019-11-12T14:00:38.9768134Z Task         : Get sources
2019-11-12T14:00:38.9768201Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
