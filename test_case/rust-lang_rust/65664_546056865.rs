plain
2019-10-24T17:46:08.8576099Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-24T17:46:08.8777270Z ##[command]git config gc.auto 0
2019-10-24T17:46:08.8860681Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-24T17:46:08.8922984Z ##[command]git config --get-all http.proxy
2019-10-24T17:46:08.9076162Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65664/merge:refs/remotes/pull/65664/merge
---
2019-10-24T18:49:18.6641131Z .................................................................................................... 1600/9243
2019-10-24T18:49:24.2069881Z .................................................................................................... 1700/9243
2019-10-24T18:49:36.9556708Z ......................................................i...............i............................. 1800/9243
2019-10-24T18:49:44.8167996Z .................................................................................................... 1900/9243
2019-10-24T18:49:59.7144511Z ............................................iiiii................................................... 2000/9243
2019-10-24T18:50:10.4328428Z .................................................................................................... 2200/9243
2019-10-24T18:50:13.0718179Z .................................................................................................... 2300/9243
2019-10-24T18:50:17.0313206Z .................................................................................................... 2400/9243
2019-10-24T18:50:40.8537763Z .................................................................................................... 2500/9243
2019-10-24T18:50:40.8537763Z .................................................................................................... 2500/9243
2019-10-24T18:50:43.4389520Z .................................................................................................... 2600/9243
2019-10-24T18:50:51.9305782Z .................................................................................................... 2700/9243
2019-10-24T18:51:01.4381558Z ...............i.................................................................................... 2800/9243
2019-10-24T18:51:10.0380369Z .................................................................................................... 2900/9243
2019-10-24T18:51:14.6276701Z ...............i.................................................................................... 3000/9243
2019-10-24T18:51:24.1981113Z .................................................................................................... 3100/9243
2019-10-24T18:51:29.0340190Z ................................................................................................ii.. 3200/9243
2019-10-24T18:51:47.3289778Z .................................................................................................... 3400/9243
2019-10-24T18:51:55.7095944Z ..........................................................................................i......... 3500/9243
2019-10-24T18:52:03.2057510Z .....................................i.............................................................. 3600/9243
2019-10-24T18:52:09.8123840Z .................................................................................................... 3700/9243
---
2019-10-24T18:53:35.2808448Z .................................................i..............i................................... 4800/9243
2019-10-24T18:53:44.2079178Z .................................................................................................... 4900/9243
2019-10-24T18:53:52.9144024Z .................................................................................................... 5000/9243
2019-10-24T18:53:59.6761836Z .................................................................................................... 5100/9243
2019-10-24T18:54:09.7147750Z ................................................ii.ii............................................... 5200/9243
2019-10-24T18:54:19.5774936Z .................................................................................................... 5400/9243
2019-10-24T18:54:29.0693003Z .................................................................................................... 5500/9243
2019-10-24T18:54:36.5773544Z ...............i.................................................................................... 5600/9243
2019-10-24T18:54:42.0988199Z .................................................................................................... 5700/9243
2019-10-24T18:54:42.0988199Z .................................................................................................... 5700/9243
2019-10-24T18:54:54.3202739Z .................................................................................................... 5800/9243
2019-10-24T18:55:06.2328642Z ............ii...i..ii...........i.................................................................. 5900/9243
2019-10-24T18:55:27.7040531Z .................................................................................................... 6100/9243
2019-10-24T18:55:35.9448327Z .................................................................................................... 6200/9243
2019-10-24T18:55:35.9448327Z .................................................................................................... 6200/9243
2019-10-24T18:55:49.9252430Z ..................................i..ii............................................................. 6300/9243
2019-10-24T18:56:11.1575913Z .................................................................................................... 6500/9243
2019-10-24T18:56:13.3813760Z i................................................................................................... 6600/9243
2019-10-24T18:56:15.6923771Z ...........................................................................i........................ 6700/9243
2019-10-24T18:56:18.3783311Z .................................................................................................... 6800/9243
---
2019-10-24T19:00:25.7451231Z 
2019-10-24T19:00:25.7451937Z ---- [ui] ui/consts/const-eval/const_panic_libcore.rs stdout ----
2019-10-24T19:00:25.7452510Z diff of stderr:
2019-10-24T19:00:25.7452584Z 
2019-10-24T19:00:25.7452631Z 4 LL | const Z: () = panic!("cheese");
2019-10-24T19:00:25.7453247Z 6    |               |
2019-10-24T19:00:25.7453247Z 6    |               |
2019-10-24T19:00:25.7453671Z -    |               the evaluated program panicked at 'cheese', $DIR/const_panic_libcore.rs:5:15
2019-10-24T19:00:25.7454213Z +    |               the evaluated program panicked at 'cheese', <::core::macros::panic macros>:12:18
2019-10-24T19:00:25.7454323Z 9    = note: `#[deny(const_err)]` on by default
2019-10-24T19:00:25.7454679Z 10    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-10-24T19:00:25.7454720Z 
2019-10-24T19:00:25.7454720Z 
2019-10-24T19:00:25.7454760Z 15 LL | const Y: () = unreachable!();
2019-10-24T19:00:25.7455149Z 17    |               |
2019-10-24T19:00:25.7455149Z 17    |               |
2019-10-24T19:00:25.7455414Z -    |               the evaluated program panicked at 'internal error: entered unreachable code', $DIR/const_panic_libcore.rs:8:15
2019-10-24T19:00:25.7456707Z +    |               the evaluated program panicked at 'internal error: entered unreachable code', <::core::macros::panic macros>:12:18
2019-10-24T19:00:25.7457350Z 20    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-10-24T19:00:25.7457413Z 21 
2019-10-24T19:00:25.7457443Z 
2019-10-24T19:00:25.7457510Z 25 LL | const X: () = unimplemented!();
2019-10-24T19:00:25.7457510Z 25 LL | const X: () = unimplemented!();
2019-10-24T19:00:25.7457770Z 26    | --------------^^^^^^^^^^^^^^^^-
2019-10-24T19:00:25.7457820Z 27    |               |
2019-10-24T19:00:25.7458152Z -    |               the evaluated program panicked at 'not yet implemented', $DIR/const_panic_libcore.rs:11:15
2019-10-24T19:00:25.7458472Z +    |               the evaluated program panicked at 'not yet implemented', <::core::macros::panic macros>:12:18
2019-10-24T19:00:25.7458889Z 30    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-10-24T19:00:25.7458956Z 31 
2019-10-24T19:00:25.7458985Z 
2019-10-24T19:00:25.7459012Z 
2019-10-24T19:00:25.7459012Z 
2019-10-24T19:00:25.7459090Z The actual stderr differed from the expected stderr.
2019-10-24T19:00:25.7459909Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore/const_panic_libcore.stderr
2019-10-24T19:00:25.7460131Z To update references, rerun the tests and pass the `--bless` flag
2019-10-24T19:00:25.7460388Z To only update this specific test, also pass `--test-args consts/const-eval/const_panic_libcore.rs`
2019-10-24T19:00:25.7460457Z error: 1 errors occurred comparing output.
2019-10-24T19:00:25.7460614Z status: exit code: 1
2019-10-24T19:00:25.7460614Z status: exit code: 1
2019-10-24T19:00:25.7462121Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_panic_libcore.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore/auxiliary" "-A" "unused"
2019-10-24T19:00:25.7462471Z ------------------------------------------
2019-10-24T19:00:25.7462506Z 
2019-10-24T19:00:25.7462749Z ------------------------------------------
2019-10-24T19:00:25.7462796Z stderr:
2019-10-24T19:00:25.7462796Z stderr:
2019-10-24T19:00:25.7463011Z ------------------------------------------
2019-10-24T19:00:25.7463062Z error: any use of this value will cause an error
2019-10-24T19:00:25.7463336Z   --> /checkout/src/test/ui/consts/const-eval/const_panic_libcore.rs:5:15
2019-10-24T19:00:25.7463388Z    |
2019-10-24T19:00:25.7463435Z LL | const Z: () = panic!("cheese");
2019-10-24T19:00:25.7463731Z    |               |
2019-10-24T19:00:25.7463731Z    |               |
2019-10-24T19:00:25.7464111Z    |               the evaluated program panicked at 'cheese', <::core::macros::panic macros>:12:18
2019-10-24T19:00:25.7464256Z    = note: `#[deny(const_err)]` on by default
2019-10-24T19:00:25.7464634Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-10-24T19:00:25.7464702Z 
2019-10-24T19:00:25.7464753Z error: any use of this value will cause an error
2019-10-24T19:00:25.7464753Z error: any use of this value will cause an error
2019-10-24T19:00:25.7465031Z   --> /checkout/src/test/ui/consts/const-eval/const_panic_libcore.rs:8:15
2019-10-24T19:00:25.7465113Z    |
2019-10-24T19:00:25.7465350Z LL | const Y: () = unreachable!();
2019-10-24T19:00:25.7465748Z    |               |
2019-10-24T19:00:25.7465748Z    |               |
2019-10-24T19:00:25.7466061Z    |               the evaluated program panicked at 'internal error: entered unreachable code', <::core::macros::panic macros>:12:18
2019-10-24T19:00:25.7466630Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-10-24T19:00:25.7466676Z 
2019-10-24T19:00:25.7466721Z error: any use of this value will cause an error
2019-10-24T19:00:25.7466982Z   --> /checkout/src/test/ui/consts/const-eval/const_panic_libcore.rs:11:15
2019-10-24T19:00:25.7466982Z   --> /checkout/src/test/ui/consts/const-eval/const_panic_libcore.rs:11:15
2019-10-24T19:00:25.7467051Z    |
2019-10-24T19:00:25.7467095Z LL | const X: () = unimplemented!();
2019-10-24T19:00:25.7467311Z    | --------------^^^^^^^^^^^^^^^^-
2019-10-24T19:00:25.7467377Z    |               |
2019-10-24T19:00:25.7467666Z    |               the evaluated program panicked at 'not yet implemented', <::core::macros::panic macros>:12:18
2019-10-24T19:00:25.7468049Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-10-24T19:00:25.7468100Z 
2019-10-24T19:00:25.7468146Z error: aborting due to 3 previous errors
2019-10-24T19:00:25.7468178Z 
2019-10-24T19:00:25.7468178Z 
2019-10-24T19:00:25.7468205Z 
2019-10-24T19:00:25.7468470Z ------------------------------------------
2019-10-24T19:00:25.7468505Z 
2019-10-24T19:00:25.7468533Z 
2019-10-24T19:00:25.7469088Z ---- [ui] ui/consts/const-eval/const_panic_libcore_main.rs stdout ----
2019-10-24T19:00:25.7469150Z diff of stderr:
2019-10-24T19:00:25.7469174Z 
2019-10-24T19:00:25.7469210Z 4 LL | const Z: () = panic!("cheese");
2019-10-24T19:00:25.7469453Z 6    |               |
2019-10-24T19:00:25.7469453Z 6    |               |
2019-10-24T19:00:25.7469690Z -    |               the evaluated program panicked at 'cheese', $DIR/const_panic_libcore_main.rs:9:15
2019-10-24T19:00:25.7469948Z +    |               the evaluated program panicked at 'cheese', <::core::macros::panic macros>:12:18
2019-10-24T19:00:25.7470034Z 9    = note: `#[deny(const_err)]` on by default
2019-10-24T19:00:25.7470331Z 10    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-10-24T19:00:25.7470395Z 
2019-10-24T19:00:25.7470395Z 
2019-10-24T19:00:25.7470432Z 15 LL | const Y: () = unreachable!();
2019-10-24T19:00:25.7470698Z 17    |               |
2019-10-24T19:00:25.7470698Z 17    |               |
2019-10-24T19:00:25.7471151Z -    |               the evaluated program panicked at 'internal error: entered unreachable code', $DIR/const_panic_libcore_main.rs:12:15
2019-10-24T19:00:25.7471433Z +    |               the evaluated program panicked at 'internal error: entered unreachable code', <::core::macros::panic macros>:12:18
2019-10-24T19:00:25.7471779Z 20    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-10-24T19:00:25.7471827Z 21 
2019-10-24T19:00:25.7471877Z 
2019-10-24T19:00:25.7471917Z 25 LL | const X: () = unimplemented!();
2019-10-24T19:00:25.7471917Z 25 LL | const X: () = unimplemented!();
2019-10-24T19:00:25.7472128Z 26    | --------------^^^^^^^^^^^^^^^^-
2019-10-24T19:00:25.7472171Z 27    |               |
2019-10-24T19:00:25.7472554Z -    |               the evaluated program panicked at 'not yet implemented', $DIR/const_panic_libcore_main.rs:15:15
2019-10-24T19:00:25.7472879Z +    |               the evaluated program panicked at 'not yet implemented', <::core::macros::panic macros>:12:18
2019-10-24T19:00:25.7473239Z 30    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-10-24T19:00:25.7473289Z 31 
2019-10-24T19:00:25.7473315Z 
2019-10-24T19:00:25.7473339Z 
2019-10-24T19:00:25.7473339Z 
2019-10-24T19:00:25.7473398Z The actual stderr differed from the expected stderr.
2019-10-24T19:00:25.7473702Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore_main/const_panic_libcore_main.stderr
2019-10-24T19:00:25.7474062Z To update references, rerun the tests and pass the `--bless` flag
2019-10-24T19:00:25.7474371Z To only update this specific test, also pass `--test-args consts/const-eval/const_panic_libcore_main.rs`
2019-10-24T19:00:25.7474446Z error: 1 errors occurred comparing output.
2019-10-24T19:00:25.7474509Z status: exit code: 1
2019-10-24T19:00:25.7474509Z status: exit code: 1
2019-10-24T19:00:25.7475203Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_panic_libcore_main.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore_main" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore_main/auxiliary" "-A" "unused"
2019-10-24T19:00:25.7475908Z ------------------------------------------
2019-10-24T19:00:25.7475949Z 
2019-10-24T19:00:25.7476213Z ------------------------------------------
2019-10-24T19:00:25.7476261Z stderr:
2019-10-24T19:00:25.7476261Z stderr:
2019-10-24T19:00:25.7476485Z ------------------------------------------
2019-10-24T19:00:25.7476556Z error: any use of this value will cause an error
2019-10-24T19:00:25.7476816Z   --> /checkout/src/test/ui/consts/const-eval/const_panic_libcore_main.rs:9:15
2019-10-24T19:00:25.7476866Z    |
2019-10-24T19:00:25.7476932Z LL | const Z: () = panic!("cheese");
2019-10-24T19:00:25.7477197Z    |               |
2019-10-24T19:00:25.7477197Z    |               |
2019-10-24T19:00:25.7477520Z    |               the evaluated program panicked at 'cheese', <::core::macros::panic macros>:12:18
2019-10-24T19:00:25.7477639Z    = note: `#[deny(const_err)]` on by default
2019-10-24T19:00:25.7477969Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-10-24T19:00:25.7478034Z 
2019-10-24T19:00:25.7478092Z error: any use of this value will cause an error
2019-10-24T19:00:25.7478092Z error: any use of this value will cause an error
2019-10-24T19:00:25.7478373Z   --> /checkout/src/test/ui/consts/const-eval/const_panic_libcore_main.rs:12:15
2019-10-24T19:00:25.7478445Z    |
2019-10-24T19:00:25.7478491Z LL | const Y: () = unreachable!();
2019-10-24T19:00:25.7478789Z    |               |
2019-10-24T19:00:25.7478789Z    |               |
2019-10-24T19:00:25.7479258Z    |               the evaluated program panicked at 'internal error: entered unreachable code', <::core::macros::panic macros>:12:18
2019-10-24T19:00:25.7479598Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-10-24T19:00:25.7479635Z 
2019-10-24T19:00:25.7479674Z error: any use of this value will cause an error
2019-10-24T19:00:25.7479906Z   --> /checkout/src/test/ui/consts/const-eval/const_panic_libcore_main.rs:15:15
2019-10-24T19:00:25.7479906Z   --> /checkout/src/test/ui/consts/const-eval/const_panic_libcore_main.rs:15:15
2019-10-24T19:00:25.7479969Z    |
2019-10-24T19:00:25.7480111Z LL | const X: () = unimplemented!();
2019-10-24T19:00:25.7480344Z    | --------------^^^^^^^^^^^^^^^^-
2019-10-24T19:00:25.7480406Z    |               |
2019-10-24T19:00:25.7480659Z    |               the evaluated program panicked at 'not yet implemented', <::core::macros::panic macros>:12:18
2019-10-24T19:00:25.7480995Z    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-10-24T19:00:25.7481031Z 
2019-10-24T19:00:25.7481069Z error: aborting due to 3 previous errors
2019-10-24T19:00:25.7481097Z 
---
2019-10-24T19:00:25.7501496Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-24T19:00:25.7501631Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-24T19:00:25.7521509Z 
2019-10-24T19:00:25.7521597Z 
2019-10-24T19:00:25.7523071Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-24T19:00:25.7523299Z 
2019-10-24T19:00:25.7523324Z 
2019-10-24T19:00:25.7530221Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-24T19:00:25.7530299Z Build completed unsuccessfully in 1:07:52
2019-10-24T19:00:25.7530299Z Build completed unsuccessfully in 1:07:52
2019-10-24T19:00:25.7580584Z == clock drift check ==
2019-10-24T19:00:25.7594501Z   local time: Thu Oct 24 19:00:25 UTC 2019
2019-10-24T19:00:26.0892858Z   network time: Thu, 24 Oct 2019 19:00:26 GMT
2019-10-24T19:00:26.0896436Z == end clock drift check ==
2019-10-24T19:00:27.3151977Z 
2019-10-24T19:00:27.3252951Z ##[error]Bash exited with code '1'.
2019-10-24T19:00:27.3301863Z ##[section]Starting: Checkout
2019-10-24T19:00:27.3304008Z ==============================================================================
2019-10-24T19:00:27.3304076Z Task         : Get sources
2019-10-24T19:00:27.3304118Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
