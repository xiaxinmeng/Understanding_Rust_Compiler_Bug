plain
2019-10-24T02:45:42.8372865Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-24T02:45:42.8758085Z ##[command]git config gc.auto 0
2019-10-24T02:45:42.8858142Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-24T02:45:42.8922099Z ##[command]git config --get-all http.proxy
2019-10-24T02:45:43.4462319Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65664/merge:refs/remotes/pull/65664/merge
---
2019-10-24T03:52:09.2364407Z .................................................................................................... 1600/9240
2019-10-24T03:52:14.9224768Z .................................................................................................... 1700/9240
2019-10-24T03:52:27.9464236Z ...................................................i...............i................................ 1800/9240
2019-10-24T03:52:36.6470364Z .................................................................................................... 1900/9240
2019-10-24T03:52:51.3247628Z .........................................iiiii...................................................... 2000/9240
2019-10-24T03:53:02.4379391Z .................................................................................................... 2200/9240
2019-10-24T03:53:05.1280200Z .................................................................................................... 2300/9240
2019-10-24T03:53:09.6510674Z .................................................................................................... 2400/9240
2019-10-24T03:53:33.8329192Z .................................................................................................... 2500/9240
---
2019-10-24T03:56:35.3603158Z .............................................i...............i...................................... 4800/9240
2019-10-24T03:56:44.9600420Z .................................................................................................... 4900/9240
2019-10-24T03:56:53.9567295Z .................................................................................................... 5000/9240
2019-10-24T03:57:01.2177873Z .................................................................................................... 5100/9240
2019-10-24T03:57:11.6271779Z .............................................ii.ii.................................................. 5200/9240
2019-10-24T03:57:22.1266629Z .................................................................................................... 5400/9240
2019-10-24T03:57:32.3303662Z .................................................................................................... 5500/9240
2019-10-24T03:57:40.0675445Z ............i....................................................................................... 5600/9240
2019-10-24T03:57:45.8208472Z .................................................................................................... 5700/9240
2019-10-24T03:57:45.8208472Z .................................................................................................... 5700/9240
2019-10-24T03:57:58.1994854Z .................................................................................................... 5800/9240
2019-10-24T03:58:10.5633985Z .........ii...i..ii...........i..................................................................... 5900/9240
2019-10-24T03:58:33.5836895Z .................................................................................................... 6100/9240
2019-10-24T03:58:42.1837621Z .................................................................................................... 6200/9240
2019-10-24T03:58:42.1837621Z .................................................................................................... 6200/9240
2019-10-24T03:58:56.7264907Z ...............................i..ii................................................................ 6300/9240
2019-10-24T03:59:18.5044246Z .................................................................................................i.. 6500/9240
2019-10-24T03:59:20.8917846Z .................................................................................................... 6600/9240
2019-10-24T03:59:23.3149024Z ........................................................................i........................... 6700/9240
2019-10-24T03:59:26.5124587Z .................................................................................................... 6800/9240
---
2019-10-24T04:03:46.6998805Z 
2019-10-24T04:03:46.6999819Z ---- [ui] ui/consts/const-eval/const_panic_libcore.rs stdout ----
2019-10-24T04:03:46.7000069Z diff of stderr:
2019-10-24T04:03:46.7000214Z 
2019-10-24T04:03:46.7000391Z 4 LL | const Z: () = panic!("cheese");
2019-10-24T04:03:46.7001030Z 6    |               |
2019-10-24T04:03:46.7001030Z 6    |               |
2019-10-24T04:03:46.7001488Z -    |               the evaluated program panicked at 'cheese', $DIR/const_panic_libcore.rs:5:15
2019-10-24T04:03:46.7001698Z +    |               "calling intrinsic `caller_location`" needs an rfc before being allowed inside constants
2019-10-24T04:03:46.7002345Z 9    = note: `#[deny(const_err)]` on by default
2019-10-24T04:03:46.7002897Z 10    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-10-24T04:03:46.7003112Z 
2019-10-24T04:03:46.7003112Z 
2019-10-24T04:03:46.7003273Z 15 LL | const Y: () = unreachable!();
2019-10-24T04:03:46.7003858Z 17    |               |
2019-10-24T04:03:46.7003858Z 17    |               |
2019-10-24T04:03:46.7004315Z -    |               the evaluated program panicked at 'internal error: entered unreachable code', $DIR/const_panic_libcore.rs:8:15
2019-10-24T04:03:46.7004746Z +    |               "calling intrinsic `caller_location`" needs an rfc before being allowed inside constants
2019-10-24T04:03:46.7005423Z 20    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-10-24T04:03:46.7006051Z 21 
2019-10-24T04:03:46.7006206Z 
2019-10-24T04:03:46.7006356Z 25 LL | const X: () = unimplemented!();
2019-10-24T04:03:46.7006356Z 25 LL | const X: () = unimplemented!();
2019-10-24T04:03:46.7006819Z 26    | --------------^^^^^^^^^^^^^^^^-
2019-10-24T04:03:46.7007020Z 27    |               |
2019-10-24T04:03:46.7007458Z -    |               the evaluated program panicked at 'not yet implemented', $DIR/const_panic_libcore.rs:11:15
2019-10-24T04:03:46.7007683Z +    |               "calling intrinsic `caller_location`" needs an rfc before being allowed inside constants
2019-10-24T04:03:46.7008308Z 30    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-10-24T04:03:46.7008553Z 31 
2019-10-24T04:03:46.7008694Z 
2019-10-24T04:03:46.7008827Z 
2019-10-24T04:03:46.7008827Z 
2019-10-24T04:03:46.7009004Z The actual stderr differed from the expected stderr.
2019-10-24T04:03:46.7009535Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore/const_panic_libcore.stderr
2019-10-24T04:03:46.7010006Z To update references, rerun the tests and pass the `--bless` flag
2019-10-24T04:03:46.7010535Z To only update this specific test, also pass `--test-args consts/const-eval/const_panic_libcore.rs`
2019-10-24T04:03:46.7010876Z error: 1 errors occurred comparing output.
2019-10-24T04:03:46.7011052Z status: exit code: 1
2019-10-24T04:03:46.7011052Z status: exit code: 1
2019-10-24T04:03:46.7012012Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_panic_libcore.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore/auxiliary" "-A" "unused"
2019-10-24T04:03:46.7014368Z ------------------------------------------
2019-10-24T04:03:46.7014659Z 
2019-10-24T04:03:46.7015143Z ------------------------------------------
2019-10-24T04:03:46.7015352Z stderr:
2019-10-24T04:03:46.7015352Z stderr:
2019-10-24T04:03:46.7016049Z ------------------------------------------
2019-10-24T04:03:46.7016331Z error: any use of this value will cause an error
2019-10-24T04:03:46.7016775Z   --> /checkout/src/test/ui/consts/const-eval/const_panic_libcore.rs:5:15
2019-10-24T04:03:46.7017011Z    |
2019-10-24T04:03:46.7017188Z LL | const Z: () = panic!("cheese");
2019-10-24T04:03:46.7017796Z    |               |
2019-10-24T04:03:46.7017796Z    |               |
2019-10-24T04:03:46.7017958Z    |               "calling intrinsic `caller_location`" needs an rfc before being allowed inside constants
2019-10-24T04:03:46.7018293Z    = note: `#[deny(const_err)]` on by default
2019-10-24T04:03:46.7018970Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-10-24T04:03:46.7019241Z 
2019-10-24T04:03:46.7019428Z error: any use of this value will cause an error
2019-10-24T04:03:46.7019428Z error: any use of this value will cause an error
2019-10-24T04:03:46.7019880Z   --> /checkout/src/test/ui/consts/const-eval/const_panic_libcore.rs:8:15
2019-10-24T04:03:46.7020089Z    |
2019-10-24T04:03:46.7020269Z LL | const Y: () = unreachable!();
2019-10-24T04:03:46.7021088Z    |               |
2019-10-24T04:03:46.7021088Z    |               |
2019-10-24T04:03:46.7021255Z    |               "calling intrinsic `caller_location`" needs an rfc before being allowed inside constants
2019-10-24T04:03:46.7022171Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-10-24T04:03:46.7022375Z 
2019-10-24T04:03:46.7022556Z error: any use of this value will cause an error
2019-10-24T04:03:46.7023110Z   --> /checkout/src/test/ui/consts/const-eval/const_panic_libcore.rs:11:15
2019-10-24T04:03:46.7023110Z   --> /checkout/src/test/ui/consts/const-eval/const_panic_libcore.rs:11:15
2019-10-24T04:03:46.7023302Z    |
2019-10-24T04:03:46.7023448Z LL | const X: () = unimplemented!();
2019-10-24T04:03:46.7023963Z    | --------------^^^^^^^^^^^^^^^^-
2019-10-24T04:03:46.7024173Z    |               |
2019-10-24T04:03:46.7024470Z    |               "calling intrinsic `caller_location`" needs an rfc before being allowed inside constants
2019-10-24T04:03:46.7030897Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-10-24T04:03:46.7031137Z 
2019-10-24T04:03:46.7031320Z error: aborting due to 3 previous errors
2019-10-24T04:03:46.7031456Z 
2019-10-24T04:03:46.7031456Z 
2019-10-24T04:03:46.7031579Z 
2019-10-24T04:03:46.7031967Z ------------------------------------------
2019-10-24T04:03:46.7032155Z 
2019-10-24T04:03:46.7032280Z 
2019-10-24T04:03:46.7032684Z ---- [ui] ui/consts/const-eval/const_panic_libcore_main.rs stdout ----
2019-10-24T04:03:46.7033008Z diff of stderr:
2019-10-24T04:03:46.7033256Z 
2019-10-24T04:03:46.7033404Z 4 LL | const Z: () = panic!("cheese");
2019-10-24T04:03:46.7038525Z 6    |               |
2019-10-24T04:03:46.7038525Z 6    |               |
2019-10-24T04:03:46.7039282Z -    |               the evaluated program panicked at 'cheese', $DIR/const_panic_libcore_main.rs:9:15
2019-10-24T04:03:46.7039632Z +    |               "calling intrinsic `caller_location`" needs an rfc before being allowed inside constants
2019-10-24T04:03:46.7040232Z 9    = note: `#[deny(const_err)]` on by default
2019-10-24T04:03:46.7040874Z 10    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-10-24T04:03:46.7041064Z 
2019-10-24T04:03:46.7041064Z 
2019-10-24T04:03:46.7041219Z 15 LL | const Y: () = unreachable!();
2019-10-24T04:03:46.7041817Z 17    |               |
2019-10-24T04:03:46.7041817Z 17    |               |
2019-10-24T04:03:46.7042278Z -    |               the evaluated program panicked at 'internal error: entered unreachable code', $DIR/const_panic_libcore_main.rs:12:15
2019-10-24T04:03:46.7042508Z +    |               "calling intrinsic `caller_location`" needs an rfc before being allowed inside constants
2019-10-24T04:03:46.7043143Z 20    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-10-24T04:03:46.7043342Z 21 
2019-10-24T04:03:46.7043469Z 
2019-10-24T04:03:46.7043651Z 25 LL | const X: () = unimplemented!();
2019-10-24T04:03:46.7043651Z 25 LL | const X: () = unimplemented!();
2019-10-24T04:03:46.7044020Z 26    | --------------^^^^^^^^^^^^^^^^-
2019-10-24T04:03:46.7044206Z 27    |               |
2019-10-24T04:03:46.7044656Z -    |               the evaluated program panicked at 'not yet implemented', $DIR/const_panic_libcore_main.rs:15:15
2019-10-24T04:03:46.7045042Z +    |               "calling intrinsic `caller_location`" needs an rfc before being allowed inside constants
2019-10-24T04:03:46.7046470Z 30    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-10-24T04:03:46.7046768Z 31 
2019-10-24T04:03:46.7046902Z 
2019-10-24T04:03:46.7047071Z 
2019-10-24T04:03:46.7047071Z 
2019-10-24T04:03:46.7047224Z The actual stderr differed from the expected stderr.
2019-10-24T04:03:46.7047712Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore_main/const_panic_libcore_main.stderr
2019-10-24T04:03:46.7048424Z To update references, rerun the tests and pass the `--bless` flag
2019-10-24T04:03:46.7048890Z To only update this specific test, also pass `--test-args consts/const-eval/const_panic_libcore_main.rs`
2019-10-24T04:03:46.7049232Z error: 1 errors occurred comparing output.
2019-10-24T04:03:46.7049395Z status: exit code: 1
2019-10-24T04:03:46.7049395Z status: exit code: 1
2019-10-24T04:03:46.7050343Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_panic_libcore_main.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore_main" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore_main/auxiliary" "-A" "unused"
2019-10-24T04:03:46.7050987Z ------------------------------------------
2019-10-24T04:03:46.7051156Z 
2019-10-24T04:03:46.7051519Z ------------------------------------------
2019-10-24T04:03:46.7051725Z stderr:
2019-10-24T04:03:46.7051725Z stderr:
2019-10-24T04:03:46.7052087Z ------------------------------------------
2019-10-24T04:03:46.7052287Z error: any use of this value will cause an error
2019-10-24T04:03:46.7052714Z   --> /checkout/src/test/ui/consts/const-eval/const_panic_libcore_main.rs:9:15
2019-10-24T04:03:46.7052904Z    |
2019-10-24T04:03:46.7053072Z LL | const Z: () = panic!("cheese");
2019-10-24T04:03:46.7053618Z    |               |
2019-10-24T04:03:46.7053618Z    |               |
2019-10-24T04:03:46.7053792Z    |               "calling intrinsic `caller_location`" needs an rfc before being allowed inside constants
2019-10-24T04:03:46.7054207Z    = note: `#[deny(const_err)]` on by default
2019-10-24T04:03:46.7054673Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-10-24T04:03:46.7054986Z 
2019-10-24T04:03:46.7055161Z error: any use of this value will cause an error
2019-10-24T04:03:46.7055161Z error: any use of this value will cause an error
2019-10-24T04:03:46.7055925Z   --> /checkout/src/test/ui/consts/const-eval/const_panic_libcore_main.rs:12:15
2019-10-24T04:03:46.7056212Z    |
2019-10-24T04:03:46.7056398Z LL | const Y: () = unreachable!();
2019-10-24T04:03:46.7057002Z    |               |
2019-10-24T04:03:46.7057002Z    |               |
2019-10-24T04:03:46.7057181Z    |               "calling intrinsic `caller_location`" needs an rfc before being allowed inside constants
2019-10-24T04:03:46.7057786Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-10-24T04:03:46.7057982Z 
2019-10-24T04:03:46.7058255Z error: any use of this value will cause an error
2019-10-24T04:03:46.7058711Z   --> /checkout/src/test/ui/consts/const-eval/const_panic_libcore_main.rs:15:15
2019-10-24T04:03:46.7058711Z   --> /checkout/src/test/ui/consts/const-eval/const_panic_libcore_main.rs:15:15
2019-10-24T04:03:46.7058927Z    |
2019-10-24T04:03:46.7059073Z LL | const X: () = unimplemented!();
2019-10-24T04:03:46.7059451Z    | --------------^^^^^^^^^^^^^^^^-
2019-10-24T04:03:46.7059638Z    |               |
2019-10-24T04:03:46.7059939Z    |               "calling intrinsic `caller_location`" needs an rfc before being allowed inside constants
2019-10-24T04:03:46.7061754Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-10-24T04:03:46.7062458Z 
2019-10-24T04:03:46.7062756Z error: aborting due to 3 previous errors
2019-10-24T04:03:46.7063221Z 
---
2019-10-24T04:03:46.7102995Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-24T04:03:46.7103114Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-24T04:03:46.7103155Z 
2019-10-24T04:03:46.7103211Z 
2019-10-24T04:03:46.7104774Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-24T04:03:46.7105133Z 
2019-10-24T04:03:46.7105163Z 
2019-10-24T04:03:46.7105211Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-24T04:03:46.7105279Z Build completed unsuccessfully in 1:11:04
2019-10-24T04:03:46.7105279Z Build completed unsuccessfully in 1:11:04
2019-10-24T04:03:46.7142010Z == clock drift check ==
2019-10-24T04:03:46.7158973Z   local time: Thu Oct 24 04:03:46 UTC 2019
2019-10-24T04:03:47.0018751Z   network time: Thu, 24 Oct 2019 04:03:47 GMT
2019-10-24T04:03:47.0018918Z == end clock drift check ==
2019-10-24T04:03:48.1647432Z 
2019-10-24T04:03:48.1774546Z ##[error]Bash exited with code '1'.
2019-10-24T04:03:48.1824270Z ##[section]Starting: Checkout
2019-10-24T04:03:48.1826875Z ==============================================================================
2019-10-24T04:03:48.1826944Z Task         : Get sources
2019-10-24T04:03:48.1827017Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
