plain
2019-09-16T21:20:42.4497483Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-16T21:20:42.4725592Z ##[command]git config gc.auto 0
2019-09-16T21:20:42.4807499Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-16T21:20:42.4862817Z ##[command]git config --get-all http.proxy
2019-09-16T21:20:42.5016521Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64470/merge:refs/remotes/pull/64470/merge
---
2019-09-16T22:21:43.9233735Z .................................................................................................... 1500/9024
2019-09-16T22:21:49.8504448Z ..................................................................................F....F............ 1600/9024
2019-09-16T22:22:02.1447445Z .................................................................i...............i.................. 1700/9024
2019-09-16T22:22:09.1566601Z .................................................................................................... 1800/9024
2019-09-16T22:22:23.7381273Z ........................................................iiiii....................................... 1900/9024
2019-09-16T22:22:34.9985559Z .................................................................................................... 2100/9024
2019-09-16T22:22:37.4384223Z .................................................................................................... 2200/9024
2019-09-16T22:22:40.6161113Z .................................................................................................... 2300/9024
2019-09-16T22:22:48.7913814Z .................................................................................................... 2400/9024
---
2019-09-16T22:25:42.1643080Z ............................................i...............i....................................... 4700/9024
2019-09-16T22:25:52.3669953Z .................................................................................................... 4800/9024
2019-09-16T22:25:59.2322181Z .................................................................................................... 4900/9024
2019-09-16T22:26:08.5300648Z .................................................................................................... 5000/9024
2019-09-16T22:26:16.0339196Z ............................ii.ii................................................................... 5100/9024
2019-09-16T22:26:26.0895711Z .................................................................................................... 5300/9024
2019-09-16T22:26:36.2376171Z ............................................................................................i....... 5400/9024
2019-09-16T22:26:44.4535866Z .................................................................................................... 5500/9024
2019-09-16T22:26:49.2134695Z .................................................................................................... 5600/9024
2019-09-16T22:26:49.2134695Z .................................................................................................... 5600/9024
2019-09-16T22:26:59.6714323Z .......................................................................................ii...i..ii... 5700/9024
2019-09-16T22:27:24.7627987Z .................................................................................................... 5900/9024
2019-09-16T22:27:34.6472028Z .................................................................................................... 6000/9024
2019-09-16T22:27:34.6472028Z .................................................................................................... 6000/9024
2019-09-16T22:27:40.6518556Z .........................................................................................i..ii...... 6100/9024
2019-09-16T22:28:08.3858221Z .................................................................................................... 6300/9024
2019-09-16T22:28:12.4915633Z ................................................i................................................... 6400/9024
2019-09-16T22:28:14.6487720Z .................................................................................................... 6500/9024
2019-09-16T22:28:17.1084103Z ....................i............................................................................... 6600/9024
---
2019-09-16T22:32:10.3633065Z 
2019-09-16T22:32:10.3633498Z ---- [ui] ui/consts/miri_unleashed/assoc_const.rs stdout ----
2019-09-16T22:32:10.3633545Z diff of stderr:
2019-09-16T22:32:10.3633634Z 
2019-09-16T22:32:10.3633672Z 4 LL |     const F: u32 = (U::X, 42).1;
2019-09-16T22:32:10.3633743Z 6 
2019-09-16T22:32:10.3633953Z - warning: skipping const checks
2019-09-16T22:32:10.3634665Z -   --> $DIR/assoc_const.rs:19:25
2019-09-16T22:32:10.3634854Z -    |
2019-09-16T22:32:10.3634854Z -    |
2019-09-16T22:32:10.3635092Z - LL |     const X: Vec<u32> = Vec::new();
2019-09-16T22:32:10.3635502Z - 
2019-09-16T22:32:10.3635577Z 13 error[E0080]: erroneous constant used
2019-09-16T22:32:10.3635792Z 14   --> $DIR/assoc_const.rs:29:13
2019-09-16T22:32:10.3635839Z 15    |
2019-09-16T22:32:10.3635839Z 15    |
2019-09-16T22:32:10.3635867Z 
2019-09-16T22:32:10.3635911Z 
2019-09-16T22:32:10.3635957Z The actual stderr differed from the expected stderr.
2019-09-16T22:32:10.3636262Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/assoc_const/assoc_const.stderr
2019-09-16T22:32:10.3636505Z To update references, rerun the tests and pass the `--bless` flag
2019-09-16T22:32:10.3636789Z To only update this specific test, also pass `--test-args consts/miri_unleashed/assoc_const.rs`
2019-09-16T22:32:10.3636869Z error: 1 errors occurred comparing output.
2019-09-16T22:32:10.3636932Z status: exit code: 1
2019-09-16T22:32:10.3636932Z status: exit code: 1
2019-09-16T22:32:10.3638043Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/assoc_const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/assoc_const" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/assoc_const/auxiliary" "-A" "unused"
2019-09-16T22:32:10.3638550Z ------------------------------------------
2019-09-16T22:32:10.3638583Z 
2019-09-16T22:32:10.3638790Z ------------------------------------------
2019-09-16T22:32:10.3638832Z stderr:
2019-09-16T22:32:10.3638832Z stderr:
2019-09-16T22:32:10.3639181Z ------------------------------------------
2019-09-16T22:32:10.3639401Z warning: skipping const checks
2019-09-16T22:32:10.3639607Z   --> /checkout/src/test/ui/consts/miri_unleashed/assoc_const.rs:12:20
2019-09-16T22:32:10.3639651Z    |
2019-09-16T22:32:10.3639710Z LL |     const F: u32 = (U::X, 42).1; //~ WARN skipping const checks
2019-09-16T22:32:10.3639793Z 
2019-09-16T22:32:10.3639829Z error[E0080]: erroneous constant used
2019-09-16T22:32:10.3640061Z   --> /checkout/src/test/ui/consts/miri_unleashed/assoc_const.rs:29:13
2019-09-16T22:32:10.3640101Z    |
2019-09-16T22:32:10.3640101Z    |
2019-09-16T22:32:10.3640144Z LL |     let y = <String as Bar<Vec<u32>, String>>::F; //~ ERROR erroneous constant
2019-09-16T22:32:10.3640208Z    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ referenced constant has errors
2019-09-16T22:32:10.3640272Z error: aborting due to previous error
2019-09-16T22:32:10.3640297Z 
2019-09-16T22:32:10.3640513Z For more information about this error, try `rustc --explain E0080`.
2019-09-16T22:32:10.3640542Z 
---
2019-09-16T22:32:10.3641432Z 
2019-09-16T22:32:10.3641484Z 1 warning: skipping const checks
2019-09-16T22:32:10.3641692Z +   --> $DIR/mutable_const.rs:9:38
2019-09-16T22:32:10.3641730Z +    |
2019-09-16T22:32:10.3641770Z + LL | const MUTABLE_BEHIND_RAW: *mut i32 = &UnsafeCell::new(42) as *const _ as *mut _;
2019-09-16T22:32:10.3641868Z + 
2019-09-16T22:32:10.3641902Z + warning: skipping const checks
2019-09-16T22:32:10.3642251Z 2   --> $DIR/mutable_const.rs:14:9
2019-09-16T22:32:10.3642288Z 3    |
2019-09-16T22:32:10.3642288Z 3    |
2019-09-16T22:32:10.3642323Z 4 LL |         *MUTABLE_BEHIND_RAW = 99
2019-09-16T22:32:10.3642382Z 
2019-09-16T22:32:10.3642418Z The actual stderr differed from the expected stderr.
2019-09-16T22:32:10.3642835Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_const/mutable_const.stderr
2019-09-16T22:32:10.3642835Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_const/mutable_const.stderr
2019-09-16T22:32:10.3643050Z To update references, rerun the tests and pass the `--bless` flag
2019-09-16T22:32:10.3643443Z To only update this specific test, also pass `--test-args consts/miri_unleashed/mutable_const.rs`
2019-09-16T22:32:10.3643523Z error: 1 errors occurred comparing output.
2019-09-16T22:32:10.3643559Z status: exit code: 1
2019-09-16T22:32:10.3643559Z status: exit code: 1
2019-09-16T22:32:10.3644706Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/mutable_const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_const" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/mutable_const/auxiliary" "-A" "unused"
2019-09-16T22:32:10.3645072Z ------------------------------------------
2019-09-16T22:32:10.3645121Z 
2019-09-16T22:32:10.3645335Z ------------------------------------------
2019-09-16T22:32:10.3645379Z stderr:
2019-09-16T22:32:10.3645379Z stderr:
2019-09-16T22:32:10.3645587Z ------------------------------------------
2019-09-16T22:32:10.3645651Z warning: skipping const checks
2019-09-16T22:32:10.3645896Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_const.rs:9:38
2019-09-16T22:32:10.3645946Z    |
2019-09-16T22:32:10.3646016Z LL | const MUTABLE_BEHIND_RAW: *mut i32 = &UnsafeCell::new(42) as *const _ as *mut _;
2019-09-16T22:32:10.3646101Z 
2019-09-16T22:32:10.3646162Z warning: skipping const checks
2019-09-16T22:32:10.3646406Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_const.rs:14:9
2019-09-16T22:32:10.3646452Z    |
2019-09-16T22:32:10.3646452Z    |
2019-09-16T22:32:10.3646516Z LL |         *MUTABLE_BEHIND_RAW = 99 //~ WARN skipping const checks
2019-09-16T22:32:10.3646614Z 
2019-09-16T22:32:10.3646658Z error: any use of this value will cause an error
2019-09-16T22:32:10.3646925Z   --> /checkout/src/test/ui/consts/miri_unleashed/mutable_const.rs:14:9
2019-09-16T22:32:10.3646973Z    |
2019-09-16T22:32:10.3646973Z    |
2019-09-16T22:32:10.3647019Z LL | / const MUTATING_BEHIND_RAW: () = {
2019-09-16T22:32:10.3647087Z LL | |     // Test that `MUTABLE_BEHIND_RAW` is actually immutable, by doing this at const time.
2019-09-16T22:32:10.3647140Z LL | |     unsafe {
2019-09-16T22:32:10.3647191Z LL | |         *MUTABLE_BEHIND_RAW = 99 //~ WARN skipping const checks
2019-09-16T22:32:10.3647261Z    | |         ^^^^^^^^^^^^^^^^^^^^^^^^ tried to modify constant memory
2019-09-16T22:32:10.3647347Z LL | |     }
2019-09-16T22:32:10.3647390Z LL | | };
2019-09-16T22:32:10.3647743Z    | |__-
2019-09-16T22:32:10.3647777Z    |
---
2019-09-16T22:32:10.3667813Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-16T22:32:10.3667913Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-16T22:32:10.3683497Z 
2019-09-16T22:32:10.3691009Z 
2019-09-16T22:32:10.3693097Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-16T22:32:10.3693345Z 
2019-09-16T22:32:10.3701088Z 
2019-09-16T22:32:10.3701181Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-16T22:32:10.3701225Z Build completed unsuccessfully in 1:04:07
2019-09-16T22:32:10.3701225Z Build completed unsuccessfully in 1:04:07
2019-09-16T22:32:10.3755889Z == clock drift check ==
2019-09-16T22:32:10.3771931Z   local time: Mon Sep 16 22:32:10 UTC 2019
2019-09-16T22:32:10.4648778Z   network time: Mon, 16 Sep 2019 22:32:10 GMT
2019-09-16T22:32:10.4649569Z == end clock drift check ==
2019-09-16T22:32:11.3285773Z ##[error]Bash exited with code '1'.
2019-09-16T22:32:11.3322751Z ##[section]Starting: Checkout
2019-09-16T22:32:11.3324980Z ==============================================================================
2019-09-16T22:32:11.3325058Z Task         : Get sources
2019-09-16T22:32:11.3325108Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
