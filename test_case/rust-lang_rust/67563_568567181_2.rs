text
2019-12-23T19:55:23.1113691Z 
2019-12-23T19:55:23.1113753Z warning: Rust code block is empty
---
2019-12-23T19:55:23.1116913Z test result: FAILED. 33 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2019-12-23T19:55:23.1116950Z 
2019-12-23T19:55:23.1116975Z 
2019-12-23T19:55:23.1117009Z 
2019-12-23T19:55:23.1118747Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-23T19:55:23.1119018Z 
2019-12-23T19:55:23.1119052Z 
2019-12-23T19:55:23.1119119Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-23T19:55:23.1119172Z Build completed unsuccessfully in 1:48:41
2019-12-23T19:55:23.1119172Z Build completed unsuccessfully in 1:48:41
2019-12-23T19:55:23.1119222Z == clock drift check ==
2019-12-23T19:55:23.1119360Z   local time: Mon Dec 23 19:55:23 UTC 2019
2019-12-23T19:55:23.4018386Z   network time: Mon, 23 Dec 2019 19:55:23 GMT
2019-12-23T19:55:23.4018588Z == end clock drift check ==
2019-12-23T19:55:24.9575475Z 
2019-12-23T19:55:24.9679420Z ##[error]Bash exited with code '1'.
2019-12-23T19:55:24.9724425Z ##[section]Starting: Checkout
2019-12-23T19:55:24.9726464Z ==============================================================================
2019-12-23T19:55:24.9726525Z Task         : Get sources
2019-12-23T19:55:24.9726596Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
