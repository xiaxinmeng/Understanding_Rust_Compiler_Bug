text
2019-10-24T08:07:34.2493477Z 
2019-10-24T08:07:34.2493553Z warning: Rust code block is empty
---
2019-10-24T08:07:34.2494979Z 
2019-10-24T08:07:34.2495019Z error: unknown start of token: \
2019-10-24T08:07:34.2495235Z  --> <doctest>:1:1
2019-10-24T08:07:34.2495299Z   |
2019-10-24T08:07:34.2495339Z 1 | \____/
2019-10-24T08:07:34.2495403Z 
2019-10-24T08:07:34.2495465Z warning: could not parse code block as Rust code
2019-10-24T08:07:34.2495717Z   --> /checkout/src/test/rustdoc-ui/invalid-syntax.rs:82:9
2019-10-24T08:07:34.2495762Z    |
2019-10-24T08:07:34.2495762Z    |
2019-10-24T08:07:34.2495822Z LL | ///     \____/
2019-10-24T08:07:34.2495863Z    |         ^^^^^^
2019-10-24T08:07:34.2495890Z 
2019-10-24T08:07:34.2495930Z error: unknown start of token: \
2019-10-24T08:07:34.2496162Z  --> <rustdoc-highlighting>:1:1
2019-10-24T08:07:34.2520604Z   |
2019-10-24T08:07:34.2520665Z 1 | \____/
2019-10-24T08:07:34.2520737Z 
2019-10-24T08:07:34.2520796Z error: unknown start of token: \
2019-10-24T08:07:34.2521235Z  --> <rustdoc-highlighting>:1:1
2019-10-24T08:07:34.2521301Z   |
---
2019-10-24T08:07:34.2530477Z 
2019-10-24T08:07:34.2530741Z error: unknown start of token: \
2019-10-24T08:07:34.2531092Z  --> <rustdoc-highlighting>:1:1
2019-10-24T08:07:34.2531139Z   |
2019-10-24T08:07:34.2531407Z 1 | \__________pkt->size___________/          \_result->size_/ \__pkt->size__/
2019-10-24T08:07:34.2531493Z 
2019-10-24T08:07:34.2531518Z 
2019-10-24T08:07:34.2531751Z ------------------------------------------
2019-10-24T08:07:34.2531782Z 
---
2019-10-24T08:07:34.2533006Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-24T08:07:34.2533064Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-24T08:07:34.2533175Z 
2019-10-24T08:07:34.2533202Z 
2019-10-24T08:07:34.2541911Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-24T08:07:34.2542464Z 
2019-10-24T08:07:34.2542511Z 
2019-10-24T08:07:34.2542563Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-24T08:07:34.2542707Z Build completed unsuccessfully in 1:49:39
2019-10-24T08:07:34.2542707Z Build completed unsuccessfully in 1:49:39
2019-10-24T08:07:34.2572009Z == clock drift check ==
2019-10-24T08:07:34.2572295Z   local time: Thu Oct 24 08:07:34 UTC 2019
2019-10-24T08:07:34.5341562Z   network time: Thu, 24 Oct 2019 08:07:34 GMT
2019-10-24T08:07:34.5341798Z == end clock drift check ==
2019-10-24T08:07:35.9877133Z 
2019-10-24T08:07:36.0075510Z ##[error]Bash exited with code '1'.
2019-10-24T08:07:36.0118081Z ##[section]Starting: Checkout
2019-10-24T08:07:36.0120543Z ==============================================================================
2019-10-24T08:07:36.0120610Z Task         : Get sources
2019-10-24T08:07:36.0120684Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
