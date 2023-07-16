plain
2019-08-05T20:16:28.4377167Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-05T20:16:28.4666803Z ##[command]git config gc.auto 0
2019-08-05T20:16:28.4732474Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-05T20:16:28.4782331Z ##[command]git config --get-all http.proxy
2019-08-05T20:16:28.4913802Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62457/merge:refs/remotes/pull/62457/merge
---
2019-08-05T20:17:03.9496661Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-05T20:17:03.9496689Z 
2019-08-05T20:17:03.9496886Z   git checkout -b <new-branch-name>
2019-08-05T20:17:03.9496913Z 
2019-08-05T20:17:03.9496956Z HEAD is now at a1a9f24ca Merge ed4d5b5c09285a62e146d46e9c93dbc03c9bf726 into f6ecdc2f61b96de199be956cad853a7c02bcfb58
2019-08-05T20:17:03.9645014Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-05T20:17:03.9647629Z ==============================================================================
2019-08-05T20:17:03.9647700Z Task         : Bash
2019-08-05T20:17:03.9647741Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-05T21:17:29.9732915Z .................................................................................................... 1400/8827
2019-08-05T21:17:35.9277259Z .................................................................................................... 1500/8827
2019-08-05T21:17:48.5410711Z ....................................................................i...............i............... 1600/8827
2019-08-05T21:17:55.6339979Z .................................................................................................... 1700/8827
2019-08-05T21:18:10.9110936Z ......................................................iiiii......................................... 1800/8827
2019-08-05T21:18:22.4755630Z .................................................................................................... 2000/8827
2019-08-05T21:18:24.9892513Z .................................................................................................... 2100/8827
2019-08-05T21:18:28.3001395Z .................................................................................................... 2200/8827
2019-08-05T21:18:36.2499690Z .................................................................................................... 2300/8827
---
2019-08-05T21:22:26.7315771Z .................................................................................................... 5200/8827
2019-08-05T21:22:35.1439749Z .....................................................................i.............................. 5300/8827
2019-08-05T21:22:42.7034351Z .................................................................................................... 5400/8827
2019-08-05T21:22:49.7097123Z .................................................................................................... 5500/8827
2019-08-05T21:23:01.7405966Z ...............................................................ii...i..ii...........i............... 5600/8827
2019-08-05T21:23:22.2265157Z .................................................................................................... 5800/8827
2019-08-05T21:23:27.2680505Z .................................................................................................... 5900/8827
2019-08-05T21:23:27.2680505Z .................................................................................................... 5900/8827
2019-08-05T21:23:33.3484488Z ................................................................i..ii............................... 6000/8827
2019-08-05T21:24:02.3180084Z .................................................................................................... 6200/8827
2019-08-05T21:24:04.5577117Z .......i............................................................................................ 6300/8827
2019-08-05T21:24:06.6929090Z ...............................................................................i.................... 6400/8827
2019-08-05T21:24:09.3859958Z .................................................................................................... 6500/8827
---
2019-08-05T21:29:02.9298998Z ---- [mir-opt] mir-opt/lower_128bit_debug_test.rs stdout ----
2019-08-05T21:29:02.9299408Z 
2019-08-05T21:29:02.9299460Z error: compilation failed!
2019-08-05T21:29:02.9299510Z status: exit code: 1
2019-08-05T21:29:02.9300612Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/mir-opt/lower_128bit_debug_test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Zdump-mir=all" "-Zmir-opt-level=3" "-Zdump-mir-exclude-pass-number" "-Zdump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/lower_128bit_debug_test" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/lower_128bit_debug_test/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "lower_128bit_ops=yes" "-C" "debug_assertions=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/lower_128bit_debug_test/auxiliary"
2019-08-05T21:29:02.9300998Z ------------------------------------------
2019-08-05T21:29:02.9301034Z 
2019-08-05T21:29:02.9301278Z ------------------------------------------
2019-08-05T21:29:02.9301326Z stderr:
2019-08-05T21:29:02.9301326Z stderr:
2019-08-05T21:29:02.9301543Z ------------------------------------------
2019-08-05T21:29:02.9301616Z error: unknown debugging option: `lower_128bit_ops`
2019-08-05T21:29:02.9301684Z 
2019-08-05T21:29:02.9301905Z ------------------------------------------
2019-08-05T21:29:02.9301954Z 
2019-08-05T21:29:02.9301982Z 
2019-08-05T21:29:02.9301982Z 
2019-08-05T21:29:02.9302216Z ---- [mir-opt] mir-opt/lower_128bit_test.rs stdout ----
2019-08-05T21:29:02.9302251Z 
2019-08-05T21:29:02.9302317Z error: compilation failed!
2019-08-05T21:29:02.9302362Z status: exit code: 1
2019-08-05T21:29:02.9303363Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/mir-opt/lower_128bit_test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Zdump-mir=all" "-Zmir-opt-level=3" "-Zdump-mir-exclude-pass-number" "-Zdump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/lower_128bit_test" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/lower_128bit_test/a" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "lower_128bit_ops=yes" "-C" "debug_assertions=no" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/lower_128bit_test/auxiliary"
2019-08-05T21:29:02.9304004Z ------------------------------------------
2019-08-05T21:29:02.9304229Z 
2019-08-05T21:29:02.9304417Z ------------------------------------------
2019-08-05T21:29:02.9304456Z stderr:
2019-08-05T21:29:02.9304456Z stderr:
2019-08-05T21:29:02.9304832Z ------------------------------------------
2019-08-05T21:29:02.9304890Z error: unknown debugging option: `lower_128bit_ops`
2019-08-05T21:29:02.9304939Z 
2019-08-05T21:29:02.9305179Z ------------------------------------------
2019-08-05T21:29:02.9305206Z 
2019-08-05T21:29:02.9305228Z 
---
2019-08-05T21:29:02.9306081Z test result: FAILED. 55 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
2019-08-05T21:29:02.9306392Z 
2019-08-05T21:29:02.9306420Z 
2019-08-05T21:29:02.9306442Z 
2019-08-05T21:29:02.9307910Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-08-05T21:29:02.9308128Z 
2019-08-05T21:29:02.9308152Z 
2019-08-05T21:29:02.9308397Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-08-05T21:29:02.9308447Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-05T21:29:02.9308447Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-05T21:29:02.9315307Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-05T21:29:02.9315419Z Build completed unsuccessfully in 1:05:47
2019-08-05T21:29:05.8953558Z ##[error]Bash exited with code '1'.
2019-08-05T21:29:05.9028393Z ##[section]Starting: Checkout
2019-08-05T21:29:05.9030581Z ==============================================================================
2019-08-05T21:29:05.9030639Z Task         : Get sources
2019-08-05T21:29:05.9030689Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
