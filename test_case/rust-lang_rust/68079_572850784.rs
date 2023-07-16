plain
2020-01-10T01:51:38.3296858Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-10T01:51:38.3388109Z ##[command]git config gc.auto 0
2020-01-10T01:51:38.3469247Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-10T01:51:38.3542880Z ##[command]git config --get-all http.proxy
2020-01-10T01:51:38.3676905Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68079/merge:refs/remotes/pull/68079/merge
---
2020-01-10T02:44:28.5613011Z .................................................................................................... 1600/9487
2020-01-10T02:44:33.7139154Z .................................................................................................... 1700/9487
2020-01-10T02:44:42.6569873Z .................................................................................................... 1800/9487
2020-01-10T02:44:51.3237761Z .....i.............................................................................................. 1900/9487
2020-01-10T02:44:58.1544971Z ...............................................................................................iiiii 2000/9487
2020-01-10T02:45:20.2964928Z .................................................................................................... 2200/9487
2020-01-10T02:45:22.7799692Z .................................................................................................... 2300/9487
2020-01-10T02:45:25.2945846Z .................................................................................................... 2400/9487
2020-01-10T02:45:31.2195369Z .................................................................................................... 2500/9487
---
2020-01-10T02:48:30.6036702Z ...........................i...............i........................................................ 4900/9487
2020-01-10T02:48:40.5538557Z .................................................................................................... 5000/9487
2020-01-10T02:48:46.7296669Z ........................................................................i........................... 5100/9487
2020-01-10T02:48:52.9652529Z .................................................................................................... 5200/9487
2020-01-10T02:49:02.3987484Z .......................................ii.ii...........i............................................ 5300/9487
2020-01-10T02:49:12.0776925Z .................................................................................................... 5500/9487
2020-01-10T02:49:21.8680152Z .................................................................................................... 5600/9487
2020-01-10T02:49:29.1160154Z .......................i............................................................................ 5700/9487
2020-01-10T02:49:35.4559720Z .................................................................................................... 5800/9487
2020-01-10T02:49:35.4559720Z .................................................................................................... 5800/9487
2020-01-10T02:49:46.9661827Z .................................................................................................... 5900/9487
2020-01-10T02:49:57.6056700Z ..............ii...i..ii...........i................................................................ 6000/9487
2020-01-10T02:50:15.2569079Z .................................................................................................... 6200/9487
2020-01-10T02:50:23.5618806Z .................................................................................................... 6300/9487
2020-01-10T02:50:23.5618806Z .................................................................................................... 6300/9487
2020-01-10T02:50:38.7957210Z .........................................i..ii...................................................... 6400/9487
2020-01-10T02:51:00.3492018Z .................................................................................................... 6600/9487
2020-01-10T02:51:02.5287720Z ................i................................................................................... 6700/9487
2020-01-10T02:51:04.9702177Z .................................................................................................... 6800/9487
2020-01-10T02:51:07.6104983Z ................i................................................................................... 6900/9487
---
2020-01-10T02:52:46.2092965Z .................................................................................................... 7500/9487
2020-01-10T02:52:50.1995282Z .................................................................................................... 7600/9487
2020-01-10T02:52:55.8164476Z .................................................................................................... 7700/9487
2020-01-10T02:53:05.9172854Z .................................................................................................... 7800/9487
2020-01-10T02:53:14.8116539Z .....................................................iiii........................................... 7900/9487
2020-01-10T02:53:29.9153655Z .................................................................................................... 8100/9487
2020-01-10T02:53:35.5224945Z .................................................................................................... 8200/9487
2020-01-10T02:53:51.5517654Z .................................................................................................... 8300/9487
2020-01-10T02:53:59.2205052Z .................................................................................................... 8400/9487
---
2020-01-10T02:55:57.3952678Z - error[E0013]: constants cannot refer to statics, use a constant instead
2020-01-10T02:55:57.3952747Z + error[E0013]: constants cannot refer to statics
2020-01-10T02:55:57.3952954Z 19   --> $DIR/issue-17718-const-bad-values.rs:5:46
2020-01-10T02:55:57.3952993Z 20    |
2020-01-10T02:55:57.3953222Z 21 LL | const C2: &'static mut usize = unsafe { &mut S };
2020-01-10T02:55:57.3953289Z 22    |                                              ^
2020-01-10T02:55:57.3953323Z +    |
2020-01-10T02:55:57.3953381Z +    = help: consider extracting the value of the static to a const, and referring to that
2020-01-10T02:55:57.3953420Z 23 
2020-01-10T02:55:57.3953420Z 23 
2020-01-10T02:55:57.3954319Z 24 error[E0658]: references in constants may only refer to immutable values
2020-01-10T02:55:57.3954658Z 25   --> $DIR/issue-17718-const-bad-values.rs:5:41
2020-01-10T02:55:57.3954695Z 
2020-01-10T02:55:57.3954721Z 
2020-01-10T02:55:57.3954768Z The actual stderr differed from the expected stderr.
2020-01-10T02:55:57.3955146Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17718-const-bad-values/issue-17718-const-bad-values.stderr
2020-01-10T02:55:57.3955421Z To update references, rerun the tests and pass the `--bless` flag
2020-01-10T02:55:57.3955925Z To only update this specific test, also pass `--test-args issues/issue-17718-const-bad-values.rs`
2020-01-10T02:55:57.3956050Z error: 1 errors occurred comparing output.
2020-01-10T02:55:57.3956095Z status: exit code: 1
2020-01-10T02:55:57.3956095Z status: exit code: 1
2020-01-10T02:55:57.3957099Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-17718-const-bad-values.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17718-const-bad-values" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17718-const-bad-values/auxiliary" "-A" "unused"
2020-01-10T02:55:57.3958115Z ------------------------------------------
2020-01-10T02:55:57.3958151Z 
2020-01-10T02:55:57.3958369Z ------------------------------------------
2020-01-10T02:55:57.3958434Z stderr:
2020-01-10T02:55:57.3958434Z stderr:
2020-01-10T02:55:57.3958646Z ------------------------------------------
2020-01-10T02:55:57.3958696Z error[E0658]: references in constants may only refer to immutable values
2020-01-10T02:55:57.3959312Z   --> /checkout/src/test/ui/issues/issue-17718-const-bad-values.rs:1:34
2020-01-10T02:55:57.3959362Z    |
2020-01-10T02:55:57.3959553Z LL | const C1: &'static mut [usize] = &mut [];
2020-01-10T02:55:57.3959616Z    |                                  ^^^^^^^ constants require immutable values
2020-01-10T02:55:57.3959653Z    |
2020-01-10T02:55:57.3959971Z    = note: for more information, see ***/issues/57349
2020-01-10T02:55:57.3960036Z    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
2020-01-10T02:55:57.3960097Z error[E0013]: constants cannot refer to statics
2020-01-10T02:55:57.3960327Z   --> /checkout/src/test/ui/issues/issue-17718-const-bad-values.rs:5:46
2020-01-10T02:55:57.3960388Z    |
2020-01-10T02:55:57.3960388Z    |
2020-01-10T02:55:57.3960958Z LL | const C2: &'static mut usize = unsafe { &mut S };
2020-01-10T02:55:57.3961053Z    |
2020-01-10T02:55:57.3961091Z    = help: consider extracting the value of the static to a const, and referring to that
2020-01-10T02:55:57.3961124Z 
2020-01-10T02:55:57.3961414Z error[E0013]: constants cannot refer to statics
2020-01-10T02:55:57.3961414Z error[E0013]: constants cannot refer to statics
2020-01-10T02:55:57.3961641Z   --> /checkout/src/test/ui/issues/issue-17718-const-bad-values.rs:5:46
2020-01-10T02:55:57.3961681Z    |
2020-01-10T02:55:57.3962265Z LL | const C2: &'static mut usize = unsafe { &mut S };
2020-01-10T02:55:57.3962365Z    |
2020-01-10T02:55:57.3962406Z    = help: consider extracting the value of the static to a const, and referring to that
2020-01-10T02:55:57.3962453Z 
2020-01-10T02:55:57.3962499Z error[E0658]: references in constants may only refer to immutable values
2020-01-10T02:55:57.3962499Z error[E0658]: references in constants may only refer to immutable values
2020-01-10T02:55:57.3962735Z   --> /checkout/src/test/ui/issues/issue-17718-const-bad-values.rs:5:41
2020-01-10T02:55:57.3962913Z    |
2020-01-10T02:55:57.3963123Z LL | const C2: &'static mut usize = unsafe { &mut S };
2020-01-10T02:55:57.3963171Z    |                                         ^^^^^^ constants require immutable values
2020-01-10T02:55:57.3963216Z    |
2020-01-10T02:55:57.3964151Z    = note: for more information, see ***/issues/57349
2020-01-10T02:55:57.3964222Z    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
2020-01-10T02:55:57.3964329Z error: aborting due to 4 previous errors
2020-01-10T02:55:57.3964360Z 
2020-01-10T02:55:57.3964407Z Some errors have detailed explanations: E0013, E0658.
2020-01-10T02:55:57.3964758Z For more information about an error, try `rustc --explain E0013`.
---
2020-01-10T02:55:57.3987980Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:384:22
2020-01-10T02:55:57.3988111Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-10T02:55:57.3999787Z 
2020-01-10T02:55:57.3999901Z 
2020-01-10T02:55:57.4001588Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-10T02:55:57.4002040Z 
2020-01-10T02:55:57.4002066Z 
2020-01-10T02:55:57.4017247Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-10T02:55:57.4017315Z Build completed unsuccessfully in 0:59:01
2020-01-10T02:55:57.4017315Z Build completed unsuccessfully in 0:59:01
2020-01-10T02:55:57.4077850Z == clock drift check ==
2020-01-10T02:55:57.4098225Z   local time: Fri Jan 10 02:55:57 UTC 2020
2020-01-10T02:55:57.6938455Z   network time: Fri, 10 Jan 2020 02:55:57 GMT
2020-01-10T02:55:57.6943475Z == end clock drift check ==
2020-01-10T02:55:58.0913402Z 
2020-01-10T02:55:58.1024874Z ##[error]Bash exited with code '1'.
2020-01-10T02:55:58.1068416Z ##[section]Starting: Checkout
2020-01-10T02:55:58.1069889Z ==============================================================================
2020-01-10T02:55:58.1069932Z Task         : Get sources
2020-01-10T02:55:58.1069988Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
