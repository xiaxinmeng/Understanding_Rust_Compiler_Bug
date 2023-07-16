
2019-07-30T12:40:57.4153308Z   | ^
2019-07-30T12:40:57.4153717Z help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
2019-07-30T12:40:57.4154080Z 1 | '``
2019-07-30T12:40:57.4154113Z   | ^
2019-07-30T12:40:57.4154139Z 
2019-07-30T12:40:57.4154171Z error: unknown start of token: \
---
2019-07-30T12:40:57.4155811Z  --> <rustdoc-highlighting>:3:30
2019-07-30T12:40:57.4155856Z   |
2019-07-30T12:40:57.4155913Z 3 |    |     ^^^^^^ did you mean `baz::foobar`?
2019-07-30T12:40:57.4155960Z   |                              ^
2019-07-30T12:40:57.4156232Z help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
2019-07-30T12:40:57.4156288Z   |
2019-07-30T12:40:57.4156530Z 3 |    |     ^^^^^^ did you mean 'baz::foobar`?
2019-07-30T12:40:57.4156609Z 
2019-07-30T12:40:57.4156666Z error: unknown start of token: \
2019-07-30T12:40:57.4156884Z  --> <rustdoc-highlighting>:1:1
2019-07-30T12:40:57.4156928Z   |
2019-07-30T12:40:57.4156928Z   |
2019-07-30T12:40:57.4157192Z 1 | \__________pkt->size___________/          \_result->size_/ \__pkt->size__/
2019-07-30T12:40:57.4157268Z 
2019-07-30T12:40:57.4157293Z 
2019-07-30T12:40:57.4157517Z ------------------------------------------
2019-07-30T12:40:57.4157556Z 
---
2019-07-30T12:40:57.4158497Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:533:22
2019-07-30T12:40:57.4158555Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-30T12:40:57.4158596Z 
2019-07-30T12:40:57.4158621Z 
2019-07-30T12:40:57.4160285Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-30T12:40:57.4160575Z 
2019-07-30T12:40:57.4160774Z 
2019-07-30T12:40:57.4160814Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-30T12:40:57.4161016Z Build completed unsuccessfully in 1:48:13
2019-07-30T12:40:57.4161016Z Build completed unsuccessfully in 1:48:13
2019-07-30T12:40:58.4342962Z ##[error]Bash exited with code '1'.
2019-07-30T12:40:58.4394792Z ##[section]Starting: Checkout
2019-07-30T12:40:58.4397192Z ==============================================================================
2019-07-30T12:40:58.4397261Z Task         : Get sources
2019-07-30T12:40:58.4397326Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
