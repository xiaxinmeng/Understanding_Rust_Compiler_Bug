plain
2020-02-16T20:17:13.3874604Z 
2020-02-16T20:17:13.3874790Z ------------------------------------------
2020-02-16T20:17:13.3874864Z stderr:
2020-02-16T20:17:13.3875046Z ------------------------------------------
2020-02-16T20:17:13.3875528Z thread 'main' panicked at 'used 976 bytes of stack, but `struct Big` is only 384 bytes', /checkout/src/test/ui/issues/issue-40883.rs:83:9
2020-02-16T20:17:13.3875703Z 
2020-02-16T20:17:13.3876293Z ------------------------------------------
2020-02-16T20:17:13.3876356Z 
2020-02-16T20:17:13.3876390Z 
---
2020-02-16T20:17:13.3886898Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-16T20:17:13.3887028Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-16T20:17:13.3898344Z 
2020-02-16T20:17:13.3900216Z 
2020-02-16T20:17:13.3902125Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.1-rust-1.43.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-16T20:17:13.3902738Z 
2020-02-16T20:17:13.3902937Z 
2020-02-16T20:17:13.3904106Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-16T20:17:13.3904184Z Build completed unsuccessfully in 1:29:20
2020-02-16T20:17:13.3904184Z Build completed unsuccessfully in 1:29:20
2020-02-16T20:17:13.3955421Z == clock drift check ==
2020-02-16T20:17:13.3972908Z   local time: Sun Feb 16 20:17:13 UTC 2020
2020-02-16T20:17:13.6683594Z   network time: Sun, 16 Feb 2020 20:17:13 GMT
2020-02-16T20:17:13.6691391Z == end clock drift check ==
2020-02-16T20:17:14.1083401Z 
2020-02-16T20:17:14.1172472Z ##[error]Bash exited with code '1'.
2020-02-16T20:17:14.1210904Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-02-16T20:17:14.1212585Z ==============================================================================
2020-02-16T20:17:14.1212657Z Task         : Get sources
2020-02-16T20:17:14.1212736Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
