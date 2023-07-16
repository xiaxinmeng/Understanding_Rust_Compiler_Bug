plain
2019-09-11T21:13:08.3091247Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-11T21:13:08.3298715Z ##[command]git config gc.auto 0
2019-09-11T21:13:08.3393183Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-11T21:13:08.3463494Z ##[command]git config --get-all http.proxy
2019-09-11T21:13:08.3618863Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64290/merge:refs/remotes/pull/64290/merge
---
2019-09-11T22:19:23.6176974Z .................................................................................................... 1500/9009
2019-09-11T22:19:29.9380266Z .................................................................................................... 1600/9009
2019-09-11T22:19:43.5991812Z .........................................................i...............i.......................... 1700/9009
2019-09-11T22:19:51.9229548Z .................................................................................................... 1800/9009
2019-09-11T22:20:07.7873325Z ................................................iiiii............................................... 1900/9009
2019-09-11T22:20:19.4589587Z .................................................................................F.................. 2100/9009
2019-09-11T22:20:22.1991700Z .................................................................................................... 2200/9009
2019-09-11T22:20:26.1129633Z .................................................................................................... 2300/9009
2019-09-11T22:20:34.7183751Z .................................................................................................... 2400/9009
---
2019-09-11T22:23:44.1183613Z ...................................i...............i................................................ 4700/9009
2019-09-11T22:23:56.1561673Z .................................................................................................... 4800/9009
2019-09-11T22:24:03.0407744Z .................................................................................................... 4900/9009
2019-09-11T22:24:14.2800291Z .................................................................................................... 5000/9009
2019-09-11T22:24:20.7975117Z ..................ii.ii............................................................................. 5100/9009
2019-09-11T22:24:31.9576742Z .................................................................................................... 5300/9009
2019-09-11T22:24:43.0783203Z .................................................................................i.................. 5400/9009
2019-09-11T22:24:51.5099669Z .................................................................................................... 5500/9009
2019-09-11T22:24:57.5814914Z .................................................................................................... 5600/9009
2019-09-11T22:24:57.5814914Z .................................................................................................... 5600/9009
2019-09-11T22:25:09.1328405Z ...........................................................................ii...i..ii...........i... 5700/9009
2019-09-11T22:25:35.9919748Z .................................................................................................... 5900/9009
2019-09-11T22:25:46.6365612Z .................................................................................................... 6000/9009
2019-09-11T22:25:46.6365612Z .................................................................................................... 6000/9009
2019-09-11T22:25:52.7091133Z .............................................................................i..ii.................. 6100/9009
2019-09-11T22:26:23.8756147Z .................................................................................................... 6300/9009
2019-09-11T22:26:26.1775461Z ....................................i............................................................... 6400/9009
2019-09-11T22:26:28.5386333Z .................................................................................................... 6500/9009
2019-09-11T22:26:31.3997416Z ........i........................................................................................... 6600/9009
---
2019-09-11T22:30:53.1343519Z - error[E0138]: multiple 'start' functions
2019-09-11T22:30:53.1343591Z + error[E0138]: multiple `start` functions
2019-09-11T22:30:53.1343814Z 2   --> $DIR/E0138.rs:7:1
2019-09-11T22:30:53.1344109Z 3    |
2019-09-11T22:30:53.1344451Z 4 LL | fn foo(argc: isize, argv: *const *const u8) -> isize { 0 }
2019-09-11T22:30:53.1344531Z 
2019-09-11T22:30:53.1344580Z The actual stderr differed from the expected stderr.
2019-09-11T22:30:53.1344907Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0138/E0138.stderr
2019-09-11T22:30:53.1344907Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0138/E0138.stderr
2019-09-11T22:30:53.1345168Z To update references, rerun the tests and pass the `--bless` flag
2019-09-11T22:30:53.1345476Z To only update this specific test, also pass `--test-args error-codes/E0138.rs`
2019-09-11T22:30:53.1345584Z error: 1 errors occurred comparing output.
2019-09-11T22:30:53.1345632Z status: exit code: 1
2019-09-11T22:30:53.1345632Z status: exit code: 1
2019-09-11T22:30:53.1346416Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0138.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0138" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0138/auxiliary" "-A" "unused"
2019-09-11T22:30:53.1346795Z ------------------------------------------
2019-09-11T22:30:53.1346831Z 
2019-09-11T22:30:53.1347402Z ------------------------------------------
2019-09-11T22:30:53.1347457Z stderr:
2019-09-11T22:30:53.1347457Z stderr:
2019-09-11T22:30:53.1347734Z ------------------------------------------
2019-09-11T22:30:53.1347784Z error[E0138]: multiple `start` functions
2019-09-11T22:30:53.1348032Z   --> /checkout/src/test/ui/error-codes/E0138.rs:7:1
2019-09-11T22:30:53.1348102Z    |
2019-09-11T22:30:53.1348373Z LL | fn foo(argc: isize, argv: *const *const u8) -> isize { 0 }
2019-09-11T22:30:53.1348679Z    | ---------------------------------------------------------- previous `start` function here
2019-09-11T22:30:53.1348769Z ...
2019-09-11T22:30:53.1349042Z LL | fn f(argc: isize, argv: *const *const u8) -> isize { 0 }
2019-09-11T22:30:53.1349113Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ multiple `start` functions
2019-09-11T22:30:53.1349217Z error: aborting due to previous error
2019-09-11T22:30:53.1349248Z 
2019-09-11T22:30:53.1349522Z For more information about this error, try `rustc --explain E0138`.
2019-09-11T22:30:53.1349558Z 
---
2019-09-11T22:30:53.1386029Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-11T22:30:53.1386120Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-11T22:30:53.1406675Z 
2019-09-11T22:30:53.1406956Z 
2019-09-11T22:30:53.1409736Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-11T22:30:53.1410049Z 
2019-09-11T22:30:53.1410080Z 
2019-09-11T22:30:53.1415857Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-11T22:30:53.1415950Z Build completed unsuccessfully in 1:09:53
2019-09-11T22:30:53.1415950Z Build completed unsuccessfully in 1:09:53
2019-09-11T22:30:53.1479708Z == clock drift check ==
2019-09-11T22:30:53.1494411Z   local time: Wed Sep 11 22:30:53 UTC 2019
2019-09-11T22:30:53.4278775Z   network time: Wed, 11 Sep 2019 22:30:53 GMT
2019-09-11T22:30:53.4283546Z == end clock drift check ==
2019-09-11T22:30:55.4984405Z ##[error]Bash exited with code '1'.
2019-09-11T22:30:55.5019934Z ##[section]Starting: Checkout
2019-09-11T22:30:55.5021816Z ==============================================================================
2019-09-11T22:30:55.5021880Z Task         : Get sources
2019-09-11T22:30:55.5022092Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
