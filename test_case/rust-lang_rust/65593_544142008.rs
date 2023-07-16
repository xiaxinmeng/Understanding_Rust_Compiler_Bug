plain
2019-10-19T11:54:27.9806839Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-19T11:54:27.9999940Z ##[command]git config gc.auto 0
2019-10-19T11:54:28.0080090Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-19T11:54:28.0132398Z ##[command]git config --get-all http.proxy
2019-10-19T11:54:28.0263054Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65593/merge:refs/remotes/pull/65593/merge
---
2019-10-19T12:56:57.7615693Z .................................................................................................... 1600/9200
2019-10-19T12:57:03.1967556Z ......................................................F............................................. 1700/9200
2019-10-19T12:57:16.3920285Z ...............................i...............i.................................................... 1800/9200
2019-10-19T12:57:23.9393744Z .................................................................................................... 1900/9200
2019-10-19T12:57:38.5493977Z .....................iiiii.......................................................................... 2000/9200
2019-10-19T12:57:49.1955785Z .................................................................................................... 2200/9200
2019-10-19T12:57:51.9632470Z .................................................................................................... 2300/9200
2019-10-19T12:57:57.4647991Z .................................................................................................... 2400/9200
2019-10-19T12:58:19.4960423Z .................................................................................................... 2500/9200
---
2019-10-19T13:01:16.3007997Z ........................i...............i........................................................... 4800/9200
2019-10-19T13:01:28.1904995Z .................................................................................................... 4900/9200
2019-10-19T13:01:34.5750619Z .................................................................................................... 5000/9200
2019-10-19T13:01:43.9723729Z .................................................................................................... 5100/9200
2019-10-19T13:01:51.4463945Z ........................ii.ii....................................................................... 5200/9200
2019-10-19T13:02:01.7224664Z .................................................................................................... 5400/9200
2019-10-19T13:02:11.9076115Z ..........................................................................................i......... 5500/9200
2019-10-19T13:02:20.2263418Z .................................................................................................... 5600/9200
2019-10-19T13:02:25.1716254Z .................................................................................................... 5700/9200
2019-10-19T13:02:25.1716254Z .................................................................................................... 5700/9200
2019-10-19T13:02:36.0995475Z .......................................................................................ii...i..ii... 5800/9200
2019-10-19T13:03:02.6485208Z .................................................................................................... 6000/9200
2019-10-19T13:03:12.4488628Z .................................................................................................... 6100/9200
2019-10-19T13:03:20.4980750Z .................................................................................................... 6200/9200
2019-10-19T13:03:20.4980750Z .................................................................................................... 6200/9200
2019-10-19T13:03:34.5557964Z .........i..ii...................................................................................... 6300/9200
2019-10-19T13:03:54.3550468Z .....................................................................i.............................. 6500/9200
2019-10-19T13:03:56.5727031Z .................................................................................................... 6600/9200
2019-10-19T13:03:59.0848275Z ............................................i....................................................... 6700/9200
2019-10-19T13:04:02.7503567Z .................................................................................................... 6800/9200
---
2019-10-19T13:08:08.2396132Z 7 error[E0080]: evaluation of constant expression failed
2019-10-19T13:08:08.2396914Z -   --> $DIR/non_const_fn.rs:10:22
2019-10-19T13:08:08.2397310Z +   --> $DIR/non_const_fn.rs:11:22
2019-10-19T13:08:08.2397491Z 9    |
2019-10-19T13:08:08.2397635Z 10 LL |     println!("{:?}", C);
2019-10-19T13:08:08.2397798Z 11    |                      ^ referenced constant has errors
2019-10-19T13:08:08.2398048Z 
2019-10-19T13:08:08.2398191Z The actual stderr differed from the expected stderr.
2019-10-19T13:08:08.2398793Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/non_const_fn/non_const_fn.stderr
2019-10-19T13:08:08.2398793Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/non_const_fn/non_const_fn.stderr
2019-10-19T13:08:08.2400058Z To update references, rerun the tests and pass the `--bless` flag
2019-10-19T13:08:08.2400531Z To only update this specific test, also pass `--test-args consts/miri_unleashed/non_const_fn.rs`
2019-10-19T13:08:08.2400839Z error: 1 errors occurred comparing output.
2019-10-19T13:08:08.2401575Z status: exit code: 1
2019-10-19T13:08:08.2401575Z status: exit code: 1
2019-10-19T13:08:08.2402815Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/non_const_fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/non_const_fn" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/non_const_fn/auxiliary" "-A" "unused"
2019-10-19T13:08:08.2404330Z ------------------------------------------
2019-10-19T13:08:08.2405484Z 
2019-10-19T13:08:08.2406442Z ------------------------------------------
2019-10-19T13:08:08.2407353Z stderr:
2019-10-19T13:08:08.2407353Z stderr:
2019-10-19T13:08:08.2407683Z ------------------------------------------
2019-10-19T13:08:08.2407751Z warning: skipping const checks
2019-10-19T13:08:08.2408022Z   --> /checkout/src/test/ui/consts/miri_unleashed/non_const_fn.rs:8:15
2019-10-19T13:08:08.2408096Z    |
2019-10-19T13:08:08.2408148Z LL | const C: () = foo(); //~ WARN: skipping const checks
2019-10-19T13:08:08.2408225Z 
2019-10-19T13:08:08.2408290Z error[E0080]: evaluation of constant expression failed
2019-10-19T13:08:08.2408561Z   --> /checkout/src/test/ui/consts/miri_unleashed/non_const_fn.rs:11:22
2019-10-19T13:08:08.2408612Z    |
2019-10-19T13:08:08.2408612Z    |
2019-10-19T13:08:08.2408679Z LL |     println!("{:?}", C); //~ ERROR: evaluation of constant expression failed
2019-10-19T13:08:08.2408743Z    |                      ^ referenced constant has errors
2019-10-19T13:08:08.2408836Z error: aborting due to previous error
2019-10-19T13:08:08.2408866Z 
2019-10-19T13:08:08.2409128Z For more information about this error, try `rustc --explain E0080`.
2019-10-19T13:08:08.2409163Z 
---
2019-10-19T13:08:08.2433655Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-19T13:08:08.2433745Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-19T13:08:08.2453776Z 
2019-10-19T13:08:08.2454478Z 
2019-10-19T13:08:08.2457627Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-19T13:08:08.2462334Z 
2019-10-19T13:08:08.2462369Z 
2019-10-19T13:08:08.2481050Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-19T13:08:08.2481395Z Build completed unsuccessfully in 1:07:08
2019-10-19T13:08:08.2481395Z Build completed unsuccessfully in 1:07:08
2019-10-19T13:08:08.2520531Z == clock drift check ==
2019-10-19T13:08:08.2540012Z   local time: Sat Oct 19 13:08:08 UTC 2019
2019-10-19T13:08:08.4059045Z   network time: Sat, 19 Oct 2019 13:08:08 GMT
2019-10-19T13:08:08.4062113Z == end clock drift check ==
2019-10-19T13:08:09.5444500Z 
2019-10-19T13:08:09.5550954Z ##[error]Bash exited with code '1'.
2019-10-19T13:08:09.5612957Z ##[section]Starting: Checkout
2019-10-19T13:08:09.5614955Z ==============================================================================
2019-10-19T13:08:09.5615001Z Task         : Get sources
2019-10-19T13:08:09.5615057Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
