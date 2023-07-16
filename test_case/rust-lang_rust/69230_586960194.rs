plain
2020-02-17T11:52:22.9667023Z 
2020-02-17T11:52:22.9667225Z ------------------------------------------
2020-02-17T11:52:22.9667297Z stderr:
2020-02-17T11:52:22.9667494Z ------------------------------------------
2020-02-17T11:52:22.9667834Z thread 'main' panicked at 'used 976 bytes of stack, but `struct Big` is only 384 bytes', /checkout/src/test/ui/issues/issue-40883.rs:83:9
2020-02-17T11:52:22.9667988Z 
2020-02-17T11:52:22.9668192Z ------------------------------------------
2020-02-17T11:52:22.9668245Z 
2020-02-17T11:52:22.9668420Z 
---
2020-02-17T11:52:22.9677198Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-17T11:52:22.9677365Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-17T11:52:22.9681252Z 
2020-02-17T11:52:22.9681320Z 
2020-02-17T11:52:22.9701568Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.1-rust-1.43.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-17T11:52:22.9704630Z 
2020-02-17T11:52:22.9704703Z 
2020-02-17T11:52:22.9704769Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-17T11:52:22.9704866Z Build completed unsuccessfully in 1:27:06
2020-02-17T11:52:22.9704866Z Build completed unsuccessfully in 1:27:06
2020-02-17T11:52:22.9744327Z == clock drift check ==
2020-02-17T11:52:22.9763092Z   local time: Mon Feb 17 11:52:22 UTC 2020
2020-02-17T11:52:23.1960516Z   network time: Mon, 17 Feb 2020 11:52:23 GMT
2020-02-17T11:52:23.1961775Z == end clock drift check ==
2020-02-17T11:52:23.6065184Z 
2020-02-17T11:52:23.6158820Z ##[error]Bash exited with code '1'.
2020-02-17T11:52:23.6198429Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-02-17T11:52:23.6201342Z ==============================================================================
2020-02-17T11:52:23.6201436Z Task         : Get sources
2020-02-17T11:52:23.6201505Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
