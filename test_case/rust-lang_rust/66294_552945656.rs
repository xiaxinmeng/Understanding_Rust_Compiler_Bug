plain
2019-11-12T14:21:54.7494220Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-12T14:21:54.7674183Z ##[command]git config gc.auto 0
2019-11-12T14:21:54.7758354Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-12T14:21:54.7817046Z ##[command]git config --get-all http.proxy
2019-11-12T14:21:54.7970427Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66294/merge:refs/remotes/pull/66294/merge
---
2019-11-12T15:22:11.1407710Z .................................................................................................... 1400/9232
2019-11-12T15:22:17.5004158Z .................................................................................................... 1500/9232
2019-11-12T15:22:24.2664200Z .................................................................................................... 1600/9232
2019-11-12T15:22:32.6637506Z .................................................................................................... 1700/9232
2019-11-12T15:22:41.1305725Z ..i................................................................................................. 1800/9232
2019-11-12T15:22:47.9905571Z ......................................................................................iiiii......... 1900/9232
2019-11-12T15:23:09.5679758Z .................................................................................................... 2100/9232
2019-11-12T15:23:11.9279036Z .................................................................................................... 2200/9232
2019-11-12T15:23:14.4723196Z .................................................................................................... 2300/9232
2019-11-12T15:23:24.0582305Z .................................................................................................... 2400/9232
2019-11-12T15:23:24.0582305Z .................................................................................................... 2400/9232
2019-11-12T15:23:41.1315533Z .................................................................................................... 2500/9232
2019-11-12T15:23:44.9468263Z .................................................................................................... 2600/9232
2019-11-12T15:23:55.5682219Z .....................................................i.............................................. 2700/9232
2019-11-12T15:24:02.9591097Z .................................................................................................... 2800/9232
2019-11-12T15:24:09.8174495Z ....................................................i............................................... 2900/9232
2019-11-12T15:24:17.6950751Z .................................................................................................... 3000/9232
2019-11-12T15:24:23.8170440Z .................................................................................................... 3100/9232
2019-11-12T15:24:30.4833676Z ..................................ii.................F...........................................F.. 3200/9232
2019-11-12T15:24:46.6853335Z .................................................................................................... 3400/9232
2019-11-12T15:24:54.2168285Z ..........................i..............................................i.......................... 3500/9232
2019-11-12T15:24:59.4033439Z .................................................................................................... 3600/9232
2019-11-12T15:25:05.1364883Z .................................................................................................... 3700/9232
---
2019-11-12T15:26:17.1314108Z ...................................................................................i...............i 4700/9232
2019-11-12T15:26:24.0858583Z .................................................................................................... 4800/9232
2019-11-12T15:26:33.2784345Z .................................................................................................... 4900/9232
2019-11-12T15:26:38.5147215Z .................................................................................................... 5000/9232
2019-11-12T15:26:49.9206924Z ......................................................................................ii.ii......... 5100/9232
2019-11-12T15:26:53.7423507Z ..i................................................................................................. 5200/9232
2019-11-12T15:27:07.0301940Z .................................................................................................... 5400/9232
2019-11-12T15:27:15.4144535Z ....................................................................i............................... 5500/9232
2019-11-12T15:27:22.9486254Z .................................................................................................... 5600/9232
2019-11-12T15:27:30.5737611Z .................................................................................................... 5700/9232
2019-11-12T15:27:30.5737611Z .................................................................................................... 5700/9232
2019-11-12T15:27:39.8072062Z .....................................................ii...i..ii...........i......................... 5800/9232
2019-11-12T15:28:02.7024967Z .................................................................................................... 6000/9232
2019-11-12T15:28:10.9598809Z .................................................................................................... 6100/9232
2019-11-12T15:28:10.9598809Z .................................................................................................... 6100/9232
2019-11-12T15:28:15.9748759Z ........................................................................i..ii....................... 6200/9232
2019-11-12T15:28:45.1470110Z .................................................................................................... 6400/9232
2019-11-12T15:28:47.3579780Z ........................................i........................................................... 6500/9232
2019-11-12T15:28:49.5856777Z .................................................................................................... 6600/9232
2019-11-12T15:28:51.9863665Z ........................i........................................................................... 6700/9232
---
2019-11-12T15:33:41.5986181Z - error[E0080]: evaluation of constant value failed
2019-11-12T15:33:41.5986723Z + error[E0391]: cycle detected when const-evaluating `a`
2019-11-12T15:33:41.5987236Z 2   --> $DIR/infinite-recursion-const-fn.rs:3:25
2019-11-12T15:33:41.5987541Z 3    |
2019-11-12T15:33:41.5987991Z 4 LL | const fn a() -> usize { b() }
2019-11-12T15:33:41.5988487Z 5    |                         ^^^
2019-11-12T15:33:41.5988927Z -    |                         |
2019-11-12T15:33:41.5989710Z -    |                         reached the configured maximum number of stack frames
2019-11-12T15:33:41.5989710Z -    |                         reached the configured maximum number of stack frames
2019-11-12T15:33:41.5990320Z -    |                         inside call to `b` at $DIR/infinite-recursion-const-fn.rs:3:25
2019-11-12T15:33:41.5990607Z +    |
2019-11-12T15:33:41.5991072Z + note: ...which requires const-evaluating `b`...
2019-11-12T15:33:41.5991602Z +   --> $DIR/infinite-recursion-const-fn.rs:4:25
2019-11-12T15:33:41.5991895Z +    |
2019-11-12T15:33:41.5992361Z 9 LL | const fn b() -> usize { a() }
2019-11-12T15:33:41.5993606Z -    |                         |
2019-11-12T15:33:41.5994301Z -    |                         inside call to `a` at $DIR/infinite-recursion-const-fn.rs:4:25
2019-11-12T15:33:41.5994871Z -    |                         inside call to `a` at $DIR/infinite-recursion-const-fn.rs:4:25
2019-11-12T15:33:41.5995438Z -    |                         inside call to `a` at $DIR/infinite-recursion-const-fn.rs:4:25
---
2019-11-12T15:33:41.6021143Z -    |                         inside call to `a` at $DIR/infinite-recursion-const-fn.rs:4:25
2019-11-12T15:33:41.6021623Z -    |                         inside call to `a` at $DIR/infinite-recursion-const-fn.rs:4:25
2019-11-12T15:33:41.6022070Z -    |                         inside call to `a` at $DIR/infinite-recursion-const-fn.rs:4:25
2019-11-12T15:33:41.6022273Z +    |                         ^^^
2019-11-12T15:33:41.6022659Z +    = note: ...which again requires const-evaluating `a`, completing the cycle
2019-11-12T15:33:41.6023060Z + note: cycle used when const-evaluating `ARR::{{constant}}#0`
2019-11-12T15:33:41.6024058Z +   --> $DIR/infinite-recursion-const-fn.rs:5:18
2019-11-12T15:33:41.6024303Z +    |
2019-11-12T15:33:41.6024441Z 61 LL | const ARR: [i32; a()] = [5; 6];
2019-11-12T15:33:41.6024888Z -    |                  --- inside call to `a` at $DIR/infinite-recursion-const-fn.rs:5:18
2019-11-12T15:33:41.6025197Z 63 
2019-11-12T15:33:41.6025348Z 64 error: aborting due to previous error
2019-11-12T15:33:41.6025476Z 65 
2019-11-12T15:33:41.6025590Z 
2019-11-12T15:33:41.6025590Z 
2019-11-12T15:33:41.6025979Z - For more information about this error, try `rustc --explain E0080`.
2019-11-12T15:33:41.6026526Z + For more information about this error, try `rustc --explain E0391`.
2019-11-12T15:33:41.6026751Z 67 
2019-11-12T15:33:41.6026868Z 
2019-11-12T15:33:41.6027003Z 
2019-11-12T15:33:41.6027156Z The actual stderr differed from the expected stderr.
2019-11-12T15:33:41.6027655Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-recursion-const-fn/infinite-recursion-const-fn.stderr
2019-11-12T15:33:41.6028095Z To update references, rerun the tests and pass the `--bless` flag
2019-11-12T15:33:41.6028555Z To only update this specific test, also pass `--test-args infinite/infinite-recursion-const-fn.rs`
2019-11-12T15:33:41.6029006Z error: 1 errors occurred comparing output.
2019-11-12T15:33:41.6029164Z status: exit code: 1
2019-11-12T15:33:41.6029164Z status: exit code: 1
2019-11-12T15:33:41.6030131Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-recursion-const-fn" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-recursion-const-fn/auxiliary" "-A" "unused"
2019-11-12T15:33:41.6030741Z ------------------------------------------
2019-11-12T15:33:41.6030915Z 
2019-11-12T15:33:41.6031276Z ------------------------------------------
2019-11-12T15:33:41.6031454Z stderr:
2019-11-12T15:33:41.6031454Z stderr:
2019-11-12T15:33:41.6031796Z ------------------------------------------
2019-11-12T15:33:41.6032197Z error[E0391]: cycle detected when const-evaluating `a`
2019-11-12T15:33:41.6032663Z   --> /checkout/src/test/ui/infinite/infinite-recursion-const-fn.rs:3:25
2019-11-12T15:33:41.6032841Z    |
2019-11-12T15:33:41.6033573Z LL | const fn a() -> usize { b() } //~ ERROR evaluation of constant value failed
2019-11-12T15:33:41.6037927Z    |
2019-11-12T15:33:41.6037927Z    |
2019-11-12T15:33:41.6038306Z note: ...which requires const-evaluating `b`...
2019-11-12T15:33:41.6038613Z    |
2019-11-12T15:33:41.6038613Z    |
2019-11-12T15:33:41.6038815Z LL | const fn b() -> usize { a() }
2019-11-12T15:33:41.6038880Z    |                         ^^^
2019-11-12T15:33:41.6039128Z    = note: ...which again requires const-evaluating `a`, completing the cycle
2019-11-12T15:33:41.6039384Z note: cycle used when const-evaluating `ARR::{{constant}}#0`
2019-11-12T15:33:41.6039695Z    |
2019-11-12T15:33:41.6039695Z    |
2019-11-12T15:33:41.6039737Z LL | const ARR: [i32; a()] = [5; 6];
2019-11-12T15:33:41.6039825Z 
2019-11-12T15:33:41.6039867Z error: aborting due to previous error
2019-11-12T15:33:41.6039904Z 
2019-11-12T15:33:41.6040166Z For more information about this error, try `rustc --explain E0391`.
---
2019-11-12T15:33:41.6040701Z ---- [ui] ui/invalid_const_promotion.rs stdout ----
2019-11-12T15:33:41.6040733Z 
2019-11-12T15:33:41.6040952Z error: test compilation failed although it shouldn't!
2019-11-12T15:33:41.6041015Z status: exit code: 1
2019-11-12T15:33:41.6041886Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/invalid_const_promotion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/invalid_const_promotion/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "debug_assertions=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/invalid_const_promotion/auxiliary"
2019-11-12T15:33:41.6042272Z ------------------------------------------
2019-11-12T15:33:41.6042306Z 
2019-11-12T15:33:41.6042537Z ------------------------------------------
2019-11-12T15:33:41.6042582Z stderr:
2019-11-12T15:33:41.6042582Z stderr:
2019-11-12T15:33:41.6042791Z ------------------------------------------
2019-11-12T15:33:41.6042839Z error[E0080]: evaluation of constant value failed
2019-11-12T15:33:41.6043088Z   --> /checkout/src/test/ui/invalid_const_promotion.rs:22:27
2019-11-12T15:33:41.6043228Z    |
2019-11-12T15:33:41.6048371Z LL | const fn bar() -> usize { 0 - 1 }
2019-11-12T15:33:41.6048462Z    |                           ^^^^^ attempt to subtract with overflow
2019-11-12T15:33:41.6048538Z error[E0080]: evaluation of constant expression failed
2019-11-12T15:33:41.6048799Z   --> /checkout/src/test/ui/invalid_const_promotion.rs:25:25
2019-11-12T15:33:41.6048846Z    |
2019-11-12T15:33:41.6049061Z LL |     let _: &'static _ = &bar();
---
2019-11-12T15:33:41.6056621Z ---- [ui] ui/union/union-nodrop.rs stdout ----
2019-11-12T15:33:41.6056652Z 
2019-11-12T15:33:41.6056888Z error: test compilation failed although it shouldn't!
2019-11-12T15:33:41.6056937Z status: exit code: 101
2019-11-12T15:33:41.6057650Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/union/union-nodrop.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-nodrop/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-nodrop/auxiliary"
2019-11-12T15:33:41.6057974Z ------------------------------------------
2019-11-12T15:33:41.6058031Z 
2019-11-12T15:33:41.6058249Z ------------------------------------------
2019-11-12T15:33:41.6058293Z stderr:
2019-11-12T15:33:41.6058293Z stderr:
2019-11-12T15:33:41.6058500Z ------------------------------------------
2019-11-12T15:33:41.6058795Z thread 'rustc' panicked at 'assertion failed: body.arg_count == 0', src/librustc_mir/const_eval.rs:149:5
2019-11-12T15:33:41.6058899Z 
2019-11-12T15:33:41.6058957Z error: internal compiler error: unexpected panic
2019-11-12T15:33:41.6058986Z 
2019-11-12T15:33:41.6059029Z note: the compiler unexpectedly panicked. this is a bug.
2019-11-12T15:33:41.6059029Z note: the compiler unexpectedly panicked. this is a bug.
2019-11-12T15:33:41.6059059Z 
2019-11-12T15:33:41.6059485Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-11-12T15:33:41.6059771Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-11-12T15:33:41.6059819Z 
2019-11-12T15:33:41.6059819Z 
2019-11-12T15:33:41.6060097Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2019-11-12T15:33:41.6060169Z 
2019-11-12T15:33:41.6060397Z ------------------------------------------
2019-11-12T15:33:41.6060428Z 
2019-11-12T15:33:41.6060453Z 
---
2019-11-12T15:33:41.6061810Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-12T15:33:41.6061883Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-12T15:33:41.6061915Z 
2019-11-12T15:33:41.6062029Z 
2019-11-12T15:33:41.6063861Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-12T15:33:41.6064101Z 
2019-11-12T15:33:41.6064130Z 
2019-11-12T15:33:41.6064234Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-12T15:33:41.6064284Z Build completed unsuccessfully in 1:05:06
2019-11-12T15:33:41.6064284Z Build completed unsuccessfully in 1:05:06
2019-11-12T15:33:41.6109126Z == clock drift check ==
2019-11-12T15:33:41.6127282Z   local time: Tue Nov 12 15:33:41 UTC 2019
2019-11-12T15:33:41.6971253Z   network time: Tue, 12 Nov 2019 15:33:41 GMT
2019-11-12T15:33:41.6981583Z == end clock drift check ==
2019-11-12T15:33:42.5056166Z 
2019-11-12T15:33:42.5184393Z ##[error]Bash exited with code '1'.
2019-11-12T15:33:42.5227110Z ##[section]Starting: Checkout
2019-11-12T15:33:42.5228717Z ==============================================================================
2019-11-12T15:33:42.5228765Z Task         : Get sources
2019-11-12T15:33:42.5228805Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
