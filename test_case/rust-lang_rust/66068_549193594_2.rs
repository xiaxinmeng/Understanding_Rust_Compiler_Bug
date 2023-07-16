text
2019-11-04T00:04:15.1241061Z 
2019-11-04T00:04:15.1241102Z warning: Rust code block is empty
---
2019-11-04T00:04:15.1242486Z 
2019-11-04T00:04:15.1242552Z error: unknown start of token: \
2019-11-04T00:04:15.1242751Z  --> <doctest>:1:1
2019-11-04T00:04:15.1242794Z   |
2019-11-04T00:04:15.1242832Z 1 | \____/
2019-11-04T00:04:15.1242916Z 
2019-11-04T00:04:15.1242958Z warning: could not parse code block as Rust code
2019-11-04T00:04:15.1243190Z   --> /checkout/src/test/rustdoc-ui/invalid-syntax.rs:82:9
2019-11-04T00:04:15.1243253Z    |
---
2019-11-04T00:04:15.1244391Z test result: FAILED. 31 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2019-11-04T00:04:15.1244581Z 
2019-11-04T00:04:15.1244606Z 
2019-11-04T00:04:15.1244631Z 
2019-11-04T00:04:15.1247179Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-04T00:04:15.1247459Z 
2019-11-04T00:04:15.1247490Z 
2019-11-04T00:04:15.1247836Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-04T00:04:15.1247908Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-04T00:04:15.1247908Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-04T00:04:15.1247985Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-04T00:04:15.1248035Z Build completed unsuccessfully in 1:48:48
2019-11-04T00:04:15.1248079Z == clock drift check ==
2019-11-04T00:04:15.1248133Z   local time: Mon Nov  4 00:04:15 UTC 2019
2019-11-04T00:04:15.4009413Z   network time: Mon, 04 Nov 2019 00:04:15 GMT
2019-11-04T00:04:15.4014062Z == end clock drift check ==
2019-11-04T00:04:16.7902185Z 
2019-11-04T00:04:16.8034839Z ##[error]Bash exited with code '1'.
2019-11-04T00:04:16.8068349Z ##[section]Starting: Checkout
2019-11-04T00:04:16.8070304Z ==============================================================================
2019-11-04T00:04:16.8070558Z Task         : Get sources
2019-11-04T00:04:16.8070603Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
