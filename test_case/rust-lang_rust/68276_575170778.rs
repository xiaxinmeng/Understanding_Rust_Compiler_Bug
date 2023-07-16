plain
2020-01-16T14:16:03.8956350Z diff of stderr:
2020-01-16T14:16:03.8956418Z 
2020-01-16T14:16:03.8956496Z 11    |                  ^^^ this module is private
2020-01-16T14:16:03.8956594Z 12    |
2020-01-16T14:16:03.8956666Z 13 note: the module `sys` is defined here
2020-01-16T14:16:03.8956983Z -   --> $SRC_DIR/libstd/lib.rs:LL:COL
2020-01-16T14:16:03.8957470Z - LL | mod sys;
2020-01-16T14:16:03.8957715Z -    | ^^^^^^^^
2020-01-16T14:16:03.8957789Z 18 
2020-01-16T14:16:03.8957876Z 19 error: aborting due to 2 previous errors
2020-01-16T14:16:03.8957876Z 19 error: aborting due to 2 previous errors
2020-01-16T14:16:03.8957952Z 20 
2020-01-16T14:16:03.8958015Z 
2020-01-16T14:16:03.8958068Z 
2020-01-16T14:16:03.8958145Z The actual stderr differed from the expected stderr.
2020-01-16T14:16:03.8958576Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-38857/issue-38857.stderr
2020-01-16T14:16:03.8958921Z To update references, rerun the tests and pass the `--bless` flag
2020-01-16T14:16:03.8959282Z To only update this specific test, also pass `--test-args issues/issue-38857.rs`
2020-01-16T14:16:03.8959439Z error: 1 errors occurred comparing output.
2020-01-16T14:16:03.8959518Z status: exit code: 1
2020-01-16T14:16:03.8959518Z status: exit code: 1
2020-01-16T14:16:03.8960641Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-38857.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-38857" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-38857/auxiliary" "-A" "unused"
2020-01-16T14:16:03.8961257Z ------------------------------------------
2020-01-16T14:16:03.8961313Z 
2020-01-16T14:16:03.8961576Z ------------------------------------------
2020-01-16T14:16:03.8961671Z stderr:
2020-01-16T14:16:03.8961671Z stderr:
2020-01-16T14:16:03.8961930Z ------------------------------------------
2020-01-16T14:16:03.8962072Z error[E0433]: failed to resolve: could not find `imp` in `sys`
2020-01-16T14:16:03.8962517Z   --> /checkout/src/test/ui/issues/issue-38857.rs:2:23
2020-01-16T14:16:03.8962605Z    |
2020-01-16T14:16:03.8962708Z LL |     let a = std::sys::imp::process::process_common::StdioPipes { ..panic!() };
2020-01-16T14:16:03.8962816Z    |                       ^^^ could not find `imp` in `sys`
2020-01-16T14:16:03.8963204Z error[E0603]: module `sys` is private
2020-01-16T14:16:03.8963585Z   --> /checkout/src/test/ui/issues/issue-38857.rs:2:18
2020-01-16T14:16:03.8963680Z    |
2020-01-16T14:16:03.8963680Z    |
2020-01-16T14:16:03.8963782Z LL |     let a = std::sys::imp::process::process_common::StdioPipes { ..panic!() };
2020-01-16T14:16:03.8963882Z    |                  ^^^ this module is private
2020-01-16T14:16:03.8964045Z note: the module `sys` is defined here
2020-01-16T14:16:03.8964111Z 
2020-01-16T14:16:03.8964180Z error: aborting due to 2 previous errors
2020-01-16T14:16:03.8964244Z 
---
2020-01-16T14:16:03.8965545Z diff of stderr:
2020-01-16T14:16:03.8965604Z 
2020-01-16T14:16:03.8965800Z 5    |                          ^^^^^^^^^^^ this module is private
2020-01-16T14:16:03.8965915Z 6    |
2020-01-16T14:16:03.8965990Z 7 note: the module `thread_info` is defined here
2020-01-16T14:16:03.8966337Z -   --> $SRC_DIR/libstd/thread/mod.rs:LL:COL
2020-01-16T14:16:03.8966847Z - LL | use crate::sys_common::thread_info;
2020-01-16T14:16:03.8967118Z -    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-01-16T14:16:03.8967214Z 12 
2020-01-16T14:16:03.8967282Z 13 error: aborting due to previous error
2020-01-16T14:16:03.8967282Z 13 error: aborting due to previous error
2020-01-16T14:16:03.8967373Z 14 
2020-01-16T14:16:03.8967413Z 
2020-01-16T14:16:03.8967451Z 
2020-01-16T14:16:03.8967543Z The actual stderr differed from the expected stderr.
2020-01-16T14:16:03.8967982Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-in-private-module/stability-in-private-module.stderr
2020-01-16T14:16:03.8968327Z To update references, rerun the tests and pass the `--bless` flag
2020-01-16T14:16:03.8968716Z To only update this specific test, also pass `--test-args stability-in-private-module.rs`
2020-01-16T14:16:03.8968875Z error: 1 errors occurred comparing output.
2020-01-16T14:16:03.8968954Z status: exit code: 1
2020-01-16T14:16:03.8968954Z status: exit code: 1
2020-01-16T14:16:03.8970083Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/stability-in-private-module.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-in-private-module" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-in-private-module/auxiliary" "-A" "unused"
2020-01-16T14:16:03.8970689Z ------------------------------------------
2020-01-16T14:16:03.8970760Z 
2020-01-16T14:16:03.8971033Z ------------------------------------------
2020-01-16T14:16:03.8971131Z stderr:
---
2020-01-16T14:16:03.9005983Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:387:22
2020-01-16T14:16:03.9006242Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-01-16T14:16:03.9025889Z 
2020-01-16T14:16:03.9026077Z 
2020-01-16T14:16:03.9028878Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "ui" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.42.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-16T14:16:03.9034088Z 
2020-01-16T14:16:03.9034137Z 
2020-01-16T14:16:03.9034809Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
2020-01-16T14:16:03.9035393Z Build completed unsuccessfully in 1:20:13
2020-01-16T14:16:03.9035393Z Build completed unsuccessfully in 1:20:13
2020-01-16T14:16:03.9092692Z == clock drift check ==
2020-01-16T14:16:03.9113702Z   local time: Thu Jan 16 14:16:03 UTC 2020
2020-01-16T14:16:04.4742088Z   network time: Thu, 16 Jan 2020 14:16:04 GMT
2020-01-16T14:16:04.4742705Z == end clock drift check ==
2020-01-16T14:16:05.0297071Z 
2020-01-16T14:16:05.0421376Z ##[error]Bash exited with code '1'.
2020-01-16T14:16:05.0466183Z ##[section]Starting: Checkout
2020-01-16T14:16:05.0468597Z ==============================================================================
2020-01-16T14:16:05.0468718Z Task         : Get sources
2020-01-16T14:16:05.0468812Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
