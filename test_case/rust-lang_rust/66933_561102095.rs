plain
2019-12-03T09:14:05.1023910Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-03T09:14:05.1237491Z ##[command]git config gc.auto 0
2019-12-03T09:14:05.1306502Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-03T09:14:05.1364786Z ##[command]git config --get-all http.proxy
2019-12-03T09:14:05.1547037Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66933/merge:refs/remotes/pull/66933/merge
---
2019-12-03T10:14:33.2292649Z .................................................................................................... 1600/9320
2019-12-03T10:14:37.9993184Z .................................................................................................... 1700/9320
2019-12-03T10:14:50.9502170Z ........................................i........................................................... 1800/9320
2019-12-03T10:14:59.0827270Z .................................................................................................... 1900/9320
2019-12-03T10:15:13.0881405Z .........................iiiii...................................................................... 2000/9320
2019-12-03T10:15:23.5710034Z .................................................................................................... 2200/9320
2019-12-03T10:15:26.1850215Z .................................................................................................... 2300/9320
2019-12-03T10:15:30.8689095Z .................................................................................................... 2400/9320
2019-12-03T10:15:53.0880481Z .................................................................................................... 2500/9320
---
2019-12-03T10:18:37.2579773Z ..........................i...............i......................................................... 4800/9320
2019-12-03T10:18:48.8771231Z .................................................................................................... 4900/9320
2019-12-03T10:18:54.2618472Z .................................................................................................... 5000/9320
2019-12-03T10:19:02.5871796Z ...............................F.................................................................... 5100/9320
2019-12-03T10:19:10.4172563Z .................................ii.ii...........i.................................................. 5200/9320
2019-12-03T10:19:20.2274026Z .................................................................................................... 5400/9320
2019-12-03T10:19:30.3412049Z .................................................................................................... 5500/9320
2019-12-03T10:19:37.8273006Z ...............i.................................................................................... 5600/9320
2019-12-03T10:19:44.1014526Z .................................................................................................... 5700/9320
2019-12-03T10:19:44.1014526Z .................................................................................................... 5700/9320
2019-12-03T10:19:55.9386436Z .................................................................................................... 5800/9320
2019-12-03T10:20:08.4128039Z .ii...i..ii...........i............................................................................. 5900/9320
2019-12-03T10:20:26.9949068Z .................................................................................................... 6100/9320
2019-12-03T10:20:33.0105637Z .................................................................................................... 6200/9320
2019-12-03T10:20:33.0105637Z .................................................................................................... 6200/9320
2019-12-03T10:20:47.2301000Z ........................i..ii....................................................................... 6300/9320
2019-12-03T10:21:07.5335384Z ...............................................................................................i.... 6500/9320
2019-12-03T10:21:09.8819384Z .................................................................................................... 6600/9320
2019-12-03T10:21:12.2241948Z ......................................................................................i............. 6700/9320
2019-12-03T10:21:14.9709469Z .................................................................................................... 6800/9320
---
2019-12-03T10:26:10.3970105Z 
2019-12-03T10:26:10.3970974Z ---- [ui] ui/issues/issue-66923-show-error-for-correct-call.rs stdout ----
2019-12-03T10:26:10.3971462Z diff of stderr:
2019-12-03T10:26:10.3971690Z 
2019-12-03T10:26:10.3971909Z 1 error[E0277]: a collection of type `std::vec::Vec<f64>` cannot be built from an iterator over elements of type `&f64`
2019-12-03T10:26:10.3972421Z -   --> $DIR/issue-66923-show-error-for-correct-call.rs:6:39
2019-12-03T10:26:10.3973096Z +   --> $DIR/issue-66923-show-error-for-correct-call.rs:8:39
2019-12-03T10:26:10.3973555Z 3    |
2019-12-03T10:26:10.3973768Z 4 LL |     let x2: Vec<f64> = x1.into_iter().collect();
2019-12-03T10:26:10.3974400Z 5    |                                       ^^^^^^^ a collection of type `std::vec::Vec<f64>` cannot be built from `std::iter::Iterator<Item=&f64>`
2019-12-03T10:26:10.3974618Z 
2019-12-03T10:26:10.3974870Z 7    = help: the trait `std::iter::FromIterator<&f64>` is not implemented for `std::vec::Vec<f64>`
2019-12-03T10:26:10.3975092Z 8 
2019-12-03T10:26:10.3975321Z 9 error[E0277]: a collection of type `std::vec::Vec<f64>` cannot be built from an iterator over elements of type `&f64`
2019-12-03T10:26:10.3975830Z -   --> $DIR/issue-66923-show-error-for-correct-call.rs:10:29
2019-12-03T10:26:10.3976701Z +   --> $DIR/issue-66923-show-error-for-correct-call.rs:12:29
2019-12-03T10:26:10.3977332Z 11    |
2019-12-03T10:26:10.3977563Z 12 LL |     let x3 = x1.into_iter().collect::<Vec<f64>>();
2019-12-03T10:26:10.3978226Z 13    |                             ^^^^^^^ a collection of type `std::vec::Vec<f64>` cannot be built from `std::iter::Iterator<Item=&f64>`
2019-12-03T10:26:10.3978666Z 
2019-12-03T10:26:10.3978906Z The actual stderr differed from the expected stderr.
2019-12-03T10:26:10.3978906Z The actual stderr differed from the expected stderr.
2019-12-03T10:26:10.3979552Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-66923-show-error-for-correct-call/issue-66923-show-error-for-correct-call.stderr
2019-12-03T10:26:10.3980126Z To update references, rerun the tests and pass the `--bless` flag
2019-12-03T10:26:10.3980762Z To only update this specific test, also pass `--test-args issues/issue-66923-show-error-for-correct-call.rs`
2019-12-03T10:26:10.3981292Z error: 1 errors occurred comparing output.
2019-12-03T10:26:10.3981650Z status: exit code: 1
2019-12-03T10:26:10.3981650Z status: exit code: 1
2019-12-03T10:26:10.3982835Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-66923-show-error-for-correct-call.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-66923-show-error-for-correct-call" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-66923-show-error-for-correct-call/auxiliary" "-A" "unused"
2019-12-03T10:26:10.3983591Z ------------------------------------------
2019-12-03T10:26:10.3983829Z 
2019-12-03T10:26:10.3984280Z ------------------------------------------
2019-12-03T10:26:10.3986790Z stderr:
2019-12-03T10:26:10.3986790Z stderr:
2019-12-03T10:26:10.3987098Z ------------------------------------------
2019-12-03T10:26:10.3987156Z error[E0277]: a collection of type `std::vec::Vec<f64>` cannot be built from an iterator over elements of type `&f64`
2019-12-03T10:26:10.3987454Z   --> /checkout/src/test/ui/issues/issue-66923-show-error-for-correct-call.rs:8:39
2019-12-03T10:26:10.3987508Z    |
2019-12-03T10:26:10.3987971Z LL |     let x2: Vec<f64> = x1.into_iter().collect();
2019-12-03T10:26:10.3988228Z    |                                       ^^^^^^^ a collection of type `std::vec::Vec<f64>` cannot be built from `std::iter::Iterator<Item=&f64>`
2019-12-03T10:26:10.3988295Z    |
2019-12-03T10:26:10.3988368Z    = help: the trait `std::iter::FromIterator<&f64>` is not implemented for `std::vec::Vec<f64>`
2019-12-03T10:26:10.3988406Z 
2019-12-03T10:26:10.3988459Z error[E0277]: a collection of type `std::vec::Vec<f64>` cannot be built from an iterator over elements of type `&f64`
2019-12-03T10:26:10.3988800Z   --> /checkout/src/test/ui/issues/issue-66923-show-error-for-correct-call.rs:12:29
2019-12-03T10:26:10.3988871Z    |
2019-12-03T10:26:10.3988918Z LL |     let x3 = x1.into_iter().collect::<Vec<f64>>();
2019-12-03T10:26:10.3988988Z    |                             ^^^^^^^ a collection of type `std::vec::Vec<f64>` cannot be built from `std::iter::Iterator<Item=&f64>`
2019-12-03T10:26:10.3989056Z    |
2019-12-03T10:26:10.3989115Z    = help: the trait `std::iter::FromIterator<&f64>` is not implemented for `std::vec::Vec<f64>`
2019-12-03T10:26:10.3989214Z error: aborting due to 2 previous errors
2019-12-03T10:26:10.3989244Z 
2019-12-03T10:26:10.3989495Z For more information about this error, try `rustc --explain E0277`.
2019-12-03T10:26:10.3989533Z 
---
2019-12-03T10:26:10.4009897Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-03T10:26:10.4009987Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-03T10:26:10.4030371Z 
2019-12-03T10:26:10.4030496Z 
2019-12-03T10:26:10.4033030Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-03T10:26:10.4033294Z 
2019-12-03T10:26:10.4033323Z 
2019-12-03T10:26:10.4037329Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-03T10:26:10.4066258Z Build completed unsuccessfully in 1:05:51
2019-12-03T10:26:10.4066258Z Build completed unsuccessfully in 1:05:51
2019-12-03T10:26:10.4090538Z == clock drift check ==
2019-12-03T10:26:10.4106013Z   local time: Tue Dec  3 10:26:10 UTC 2019
2019-12-03T10:26:10.6881747Z   network time: Tue, 03 Dec 2019 10:26:10 GMT
2019-12-03T10:26:10.6888452Z == end clock drift check ==
2019-12-03T10:26:11.5280759Z 
2019-12-03T10:26:11.5346154Z ##[error]Bash exited with code '1'.
2019-12-03T10:26:11.5380231Z ##[section]Starting: Checkout
2019-12-03T10:26:11.5382068Z ==============================================================================
2019-12-03T10:26:11.5382118Z Task         : Get sources
2019-12-03T10:26:11.5382178Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
