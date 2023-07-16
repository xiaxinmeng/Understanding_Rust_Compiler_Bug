plain
2019-12-07T09:27:52.1772427Z failures:
2019-12-07T09:27:52.1812995Z 
2019-12-07T09:27:52.1813969Z ---- [ui] ui/traits/cycle-cache-err-60010.rs stdout ----
2019-12-07T09:27:52.1814343Z 
2019-12-07T09:27:52.1815120Z error: /checkout/src/test/ui/traits/cycle-cache-err-60010.rs:31: unexpected error: '31:5: 31:33: overflow evaluating the requirement `RootDatabase: SourceDatabase` [E0275]'
2019-12-07T09:27:52.1815902Z error: 1 unexpected errors found, 0 expected errors not found
2019-12-07T09:27:52.1816140Z status: exit code: 1
2019-12-07T09:27:52.1816140Z status: exit code: 1
2019-12-07T09:27:52.1817781Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/cycle-cache-err-60010.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/cycle-cache-err-60010" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/cycle-cache-err-60010/auxiliary" "-A" "unused"
2019-12-07T09:27:52.1818399Z unexpected errors (from JSON output): [
2019-12-07T09:27:52.1818859Z         line_num: 31,
2019-12-07T09:27:52.1819055Z         kind: Some(
2019-12-07T09:27:52.1819278Z             Error,
2019-12-07T09:27:52.1819464Z         ),
2019-12-07T09:27:52.1819464Z         ),
2019-12-07T09:27:52.1819718Z         msg: "31:5: 31:33: overflow evaluating the requirement `RootDatabase: SourceDatabase` [E0275]",
2019-12-07T09:27:52.1820149Z ]
2019-12-07T09:27:52.1820317Z 
2019-12-07T09:27:52.1821116Z thread '[ui] ui/traits/cycle-cache-err-60010.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1520:13
2019-12-07T09:27:52.1821481Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
---
2019-12-07T09:27:52.1823560Z 
2019-12-07T09:27:52.1853820Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-07T09:27:52.1870689Z 
2019-12-07T09:27:52.1870799Z 
2019-12-07T09:27:52.1873179Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-07T09:27:52.1873964Z 
2019-12-07T09:27:52.1874006Z 
2019-12-07T09:27:52.1888581Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-07T09:27:52.1888726Z Build completed unsuccessfully in 1:03:16
2019-12-07T09:27:52.1888726Z Build completed unsuccessfully in 1:03:16
2019-12-07T09:27:52.1940851Z == clock drift check ==
2019-12-07T09:27:52.1967425Z   local time: Sat Dec  7 09:27:52 UTC 2019
2019-12-07T09:27:52.2116720Z   network time: Sat, 07 Dec 2019 09:27:52 GMT
2019-12-07T09:27:52.2118414Z == end clock drift check ==
2019-12-07T09:27:53.0472172Z 
2019-12-07T09:27:53.0584570Z ##[error]Bash exited with code '1'.
2019-12-07T09:27:53.0625429Z ##[section]Starting: Checkout
2019-12-07T09:27:53.0627583Z ==============================================================================
2019-12-07T09:27:53.0627707Z Task         : Get sources
2019-12-07T09:27:53.0627803Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
