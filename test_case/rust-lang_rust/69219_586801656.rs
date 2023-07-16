plain
2020-02-17T03:22:50.5983168Z 
2020-02-17T03:22:50.5983346Z ------------------------------------------
2020-02-17T03:22:50.5983417Z stderr:
2020-02-17T03:22:50.5983593Z ------------------------------------------
2020-02-17T03:22:50.5983889Z thread 'main' panicked at 'used 976 bytes of stack, but `struct Big` is only 384 bytes', /checkout/src/test/ui/issues/issue-40883.rs:83:9
2020-02-17T03:22:50.5984041Z 
2020-02-17T03:22:50.5984232Z ------------------------------------------
2020-02-17T03:22:50.5984287Z 
2020-02-17T03:22:50.5984315Z 
---
2020-02-17T03:22:50.5995073Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-17T03:22:50.5995223Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-17T03:22:50.6003719Z 
2020-02-17T03:22:50.6003784Z 
2020-02-17T03:22:50.6005382Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.1-rust-1.43.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-17T03:22:50.6006051Z 
2020-02-17T03:22:50.6006083Z 
2020-02-17T03:22:50.6014190Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-17T03:22:50.6014308Z Build completed unsuccessfully in 1:26:59
2020-02-17T03:22:50.6014308Z Build completed unsuccessfully in 1:26:59
2020-02-17T03:22:50.6060681Z == clock drift check ==
2020-02-17T03:22:50.6079685Z   local time: Mon Feb 17 03:22:50 UTC 2020
2020-02-17T03:22:50.7883541Z   network time: Mon, 17 Feb 2020 03:22:50 GMT
2020-02-17T03:22:50.7884671Z == end clock drift check ==
2020-02-17T03:22:51.1702915Z 
2020-02-17T03:22:51.1785889Z ##[error]Bash exited with code '1'.
2020-02-17T03:22:51.1837244Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-02-17T03:22:51.1838885Z ==============================================================================
2020-02-17T03:22:51.1838971Z Task         : Get sources
2020-02-17T03:22:51.1839035Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
