plain
2020-02-16T22:24:06.2451111Z 
2020-02-16T22:24:06.2451588Z ------------------------------------------
2020-02-16T22:24:06.2451818Z stderr:
2020-02-16T22:24:06.2452156Z ------------------------------------------
2020-02-16T22:24:06.2452713Z thread 'main' panicked at 'used 976 bytes of stack, but `struct Big` is only 384 bytes', /checkout/src/test/ui/issues/issue-40883.rs:83:9
2020-02-16T22:24:06.2453368Z 
2020-02-16T22:24:06.2453718Z ------------------------------------------
2020-02-16T22:24:06.2453937Z 
2020-02-16T22:24:06.2454011Z 
---
2020-02-16T22:24:06.2467133Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-16T22:24:06.2467594Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-16T22:24:06.2481147Z 
2020-02-16T22:24:06.2481407Z 
2020-02-16T22:24:06.2485478Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.1-rust-1.43.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-16T22:24:06.2486609Z 
2020-02-16T22:24:06.2486778Z 
2020-02-16T22:24:06.2499656Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-16T22:24:06.2500170Z Build completed unsuccessfully in 1:47:08
2020-02-16T22:24:06.2500170Z Build completed unsuccessfully in 1:47:08
2020-02-16T22:24:06.2555345Z == clock drift check ==
2020-02-16T22:24:06.2575769Z   local time: Sun Feb 16 22:24:06 UTC 2020
2020-02-16T22:24:06.4924850Z   network time: Sun, 16 Feb 2020 22:24:06 GMT
2020-02-16T22:24:06.4924988Z == end clock drift check ==
2020-02-16T22:24:07.0506196Z 
2020-02-16T22:24:07.0611018Z ##[error]Bash exited with code '1'.
2020-02-16T22:24:07.0673530Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-02-16T22:24:07.0675676Z ==============================================================================
2020-02-16T22:24:07.0675946Z Task         : Get sources
2020-02-16T22:24:07.0676047Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
