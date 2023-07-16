plain
2020-01-23T18:10:55.0160707Z 4 LL | const CONSTANT: usize = limit();
2020-01-23T18:10:55.0160947Z 
2020-01-23T18:10:55.0161204Z 
2020-01-23T18:10:55.0161492Z The actual stderr differed from the expected stderr.
2020-01-23T18:10:55.0162347Z Actual stderr saved to /checkout/obj/build/i686-unknown-linux-gnu/test/ui/consts/const_limit/const_eval_limit_reached/const_eval_limit_reached.stderr
2020-01-23T18:10:55.0163101Z To update references, rerun the tests and pass the `--bless` flag
2020-01-23T18:10:55.0164846Z To only update this specific test, also pass `--test-args consts/const_limit/const_eval_limit_reached.rs`
2020-01-23T18:10:55.0165522Z error: 1 errors occurred comparing output.
2020-01-23T18:10:55.0165821Z status: exit code: 0
2020-01-23T18:10:55.0165821Z status: exit code: 0
2020-01-23T18:10:55.0167316Z command: "/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const_limit/const_eval_limit_reached.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/i686-unknown-linux-gnu/test/ui/consts/const_limit/const_eval_limit_reached" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/i686-unknown-linux-gnu/test/ui/consts/const_limit/const_eval_limit_reached/auxiliary" "-A" "unused"
2020-01-23T18:10:55.0168634Z ------------------------------------------
2020-01-23T18:10:55.0168993Z 
2020-01-23T18:10:55.0169628Z ------------------------------------------
2020-01-23T18:10:55.0170012Z stderr:
---
2020-01-23T18:10:55.0183252Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:387:22
2020-01-23T18:10:55.0183577Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-01-23T18:10:55.0200733Z 
2020-01-23T18:10:55.0200918Z 
2020-01-23T18:10:55.0213775Z command did not execute successfully: "/checkout/obj/build/i686-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/i686-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/i686-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i686-unknown-linux-gnu" "--mode" "ui" "--target" "i686-unknown-linux-gnu" "--host" "i686-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/i686-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.1-rust-1.42.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-23T18:10:55.0215880Z 
2020-01-23T18:10:55.0215933Z 
2020-01-23T18:10:55.0222635Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-23T18:10:55.0223348Z Build completed unsuccessfully in 1:19:07
2020-01-23T18:10:55.0223348Z Build completed unsuccessfully in 1:19:07
2020-01-23T18:10:55.0303498Z == clock drift check ==
2020-01-23T18:10:55.0326303Z   local time: Thu Jan 23 18:10:55 UTC 2020
2020-01-23T18:10:55.1763100Z   network time: Thu, 23 Jan 2020 18:10:55 GMT
2020-01-23T18:10:55.1768170Z == end clock drift check ==
2020-01-23T18:10:55.7410008Z 
2020-01-23T18:10:55.7541946Z ##[error]Bash exited with code '1'.
2020-01-23T18:10:55.7587654Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-01-23T18:10:55.7590322Z ==============================================================================
2020-01-23T18:10:55.7590428Z Task         : Get sources
2020-01-23T18:10:55.7590552Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
