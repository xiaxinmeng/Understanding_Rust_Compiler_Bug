plain
2019-10-13T14:09:22.8036782Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-13T14:09:22.8162082Z ##[command]git config gc.auto 0
2019-10-13T14:09:22.8232569Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-13T14:09:22.8304251Z ##[command]git config --get-all http.proxy
2019-10-13T14:09:22.8464032Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65377/merge:refs/remotes/pull/65377/merge
---
2019-10-13T15:15:39.2256950Z .................................................................................................... 1600/9172
2019-10-13T15:15:46.6894682Z .................................................................................................... 1700/9172
2019-10-13T15:16:00.2630890Z ......................i...............i............................................................. 1800/9172
2019-10-13T15:16:08.3905338Z .................................................................................................... 1900/9172
2019-10-13T15:16:24.6953910Z .............iiiii.................................................................................. 2000/9172
2019-10-13T15:16:36.4578588Z .................................................................................................... 2200/9172
2019-10-13T15:16:39.4154209Z .................................................................................................... 2300/9172
2019-10-13T15:16:45.6536332Z .................................................................................................... 2400/9172
2019-10-13T15:17:10.0784313Z .................................................................................................... 2500/9172
---
2019-10-13T15:20:31.5178052Z ....................i...............i............................................................... 4800/9172
2019-10-13T15:20:45.4222913Z .................................................................................................... 4900/9172
2019-10-13T15:20:52.7724359Z .................................................................................................... 5000/9172
2019-10-13T15:21:04.5440461Z .................................................................................................... 5100/9172
2019-10-13T15:21:11.6678815Z ....................ii.ii........................................................................... 5200/9172
2019-10-13T15:21:23.8562760Z .................................................................................................... 5400/9172
2019-10-13T15:21:35.2322608Z ......................................................................................i............. 5500/9172
2019-10-13T15:21:44.3595348Z .................................................................................................... 5600/9172
2019-10-13T15:21:50.6831479Z .................................................................................................... 5700/9172
2019-10-13T15:21:50.6831479Z .................................................................................................... 5700/9172
2019-10-13T15:22:02.4967547Z ...................................................................................ii...i..iiF...... 5800/9172
2019-10-13T15:22:31.2608051Z .................................................................................................... 6000/9172
2019-10-13T15:22:42.4552957Z .................................................................................................... 6100/9172
2019-10-13T15:22:42.4552957Z .................................................................................................... 6100/9172
2019-10-13T15:22:51.4787536Z ..........................................................................................i..ii..... 6200/9172
2019-10-13T15:23:24.1980007Z .................................................................................................... 6400/9172
2019-10-13T15:23:29.1496523Z ..................................................i................................................. 6500/9172
2019-10-13T15:23:31.7062496Z .................................................................................................... 6600/9172
2019-10-13T15:23:34.5698925Z .......................i............................................................................ 6700/9172
---
2019-10-13T15:28:17.1706093Z 1 error[E0425]: cannot find function `bar` in module `mod_file_aux`
2019-10-13T15:28:17.1706512Z -   --> $DIR/mod_file_correct_spans.rs:6:27
2019-10-13T15:28:17.1706917Z +   --> $DIR/mod_file_correct_spans.rs:6:30
2019-10-13T15:28:17.1707120Z 3    |
2019-10-13T15:28:17.1707474Z - LL |     assert!(mod_file_aux::bar() == 10);
2019-10-13T15:28:17.1707891Z -    |                           ^^^ not found in `mod_file_aux`
2019-10-13T15:28:17.1708076Z + LL |     assert_eq!(mod_file_aux::bar(), 10);
2019-10-13T15:28:17.1708230Z +    |                              ^^^ not found in `mod_file_aux`
2019-10-13T15:28:17.1708544Z 7 error: aborting due to previous error
2019-10-13T15:28:17.1708680Z 8 
2019-10-13T15:28:17.1708822Z 
2019-10-13T15:28:17.1709732Z 
2019-10-13T15:28:17.1709732Z 
2019-10-13T15:28:17.1710143Z The actual stderr differed from the expected stderr.
2019-10-13T15:28:17.1710687Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mod/mod_file_correct_spans/mod_file_correct_spans.stderr
2019-10-13T15:28:17.1711170Z To update references, rerun the tests and pass the `--bless` flag
2019-10-13T15:28:17.1711653Z To only update this specific test, also pass `--test-args mod/mod_file_correct_spans.rs`
2019-10-13T15:28:17.1712008Z error: 1 errors occurred comparing output.
2019-10-13T15:28:17.1712159Z status: exit code: 1
2019-10-13T15:28:17.1712159Z status: exit code: 1
2019-10-13T15:28:17.1713103Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mod/mod_file_correct_spans.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mod/mod_file_correct_spans" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mod/mod_file_correct_spans/auxiliary" "-A" "unused"
2019-10-13T15:28:17.1713751Z ------------------------------------------
2019-10-13T15:28:17.1713919Z 
2019-10-13T15:28:17.1714296Z ------------------------------------------
2019-10-13T15:28:17.1714497Z stderr:
2019-10-13T15:28:17.1714497Z stderr:
2019-10-13T15:28:17.1714867Z ------------------------------------------
2019-10-13T15:28:17.1715075Z error[E0425]: cannot find function `bar` in module `mod_file_aux`
2019-10-13T15:28:17.1715471Z   --> /checkout/src/test/ui/mod/mod_file_correct_spans.rs:6:30
2019-10-13T15:28:17.1715658Z    |
2019-10-13T15:28:17.1715833Z LL |     assert_eq!(mod_file_aux::bar(), 10);
2019-10-13T15:28:17.1715988Z    |                              ^^^ not found in `mod_file_aux`
2019-10-13T15:28:17.1716304Z error: aborting due to previous error
2019-10-13T15:28:17.1716433Z 
2019-10-13T15:28:17.1716862Z For more information about this error, try `rustc --explain E0425`.
2019-10-13T15:28:17.1717032Z 
---
2019-10-13T15:28:17.1825043Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-13T15:28:17.1826274Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-13T15:28:17.1842454Z 
2019-10-13T15:28:17.1843809Z 
2019-10-13T15:28:17.1847143Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-13T15:28:17.1851006Z 
2019-10-13T15:28:17.1851065Z 
2019-10-13T15:28:17.1851115Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-13T15:28:17.1851167Z Build completed unsuccessfully in 1:12:14
2019-10-13T15:28:17.1851167Z Build completed unsuccessfully in 1:12:14
2019-10-13T15:28:17.1914486Z == clock drift check ==
2019-10-13T15:28:17.1931785Z   local time: Sun Oct 13 15:28:17 UTC 2019
2019-10-13T15:28:17.4828356Z   network time: Sun, 13 Oct 2019 15:28:17 GMT
2019-10-13T15:28:17.4829624Z == end clock drift check ==
2019-10-13T15:28:18.3230357Z ##[error]Bash exited with code '1'.
2019-10-13T15:28:18.3300920Z ##[section]Starting: Checkout
2019-10-13T15:28:18.3303708Z ==============================================================================
2019-10-13T15:28:18.3303796Z Task         : Get sources
2019-10-13T15:28:18.3303870Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
