plain
2019-09-28T03:28:03.5629105Z failures:
2019-09-28T03:28:03.5673334Z 
2019-09-28T03:28:03.5674245Z ---- [ui (nll)] ui/c-variadic/variadic-ffi-4.rs stdout ----
2019-09-28T03:28:03.5674534Z 
2019-09-28T03:28:03.5674745Z error: Error: expected failure status (Some(1)) but received status Some(101).
2019-09-28T03:28:03.5674966Z status: exit code: 101
2019-09-28T03:28:03.5676134Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/c-variadic/variadic-ffi-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-4.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-4.nll/auxiliary" "-A" "unused"
2019-09-28T03:28:03.5677466Z ------------------------------------------
2019-09-28T03:28:03.5677682Z 
2019-09-28T03:28:03.5678376Z ------------------------------------------
2019-09-28T03:28:03.5678602Z stderr:
---
2019-09-28T03:28:03.5681867Z note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
2019-09-28T03:28:03.5682083Z 
2019-09-28T03:28:03.5682528Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-09-28T03:28:03.5682869Z 
2019-09-28T03:28:03.5683513Z note: compiler flags: -Z threads=1 -Z ui-testing -Z borrowck=mir -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2019-09-28T03:28:03.5683902Z 
2019-09-28T03:28:03.5684291Z ------------------------------------------
2019-09-28T03:28:03.5684483Z 
2019-09-28T03:28:03.5684610Z 
---
2019-09-28T03:28:03.5717858Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-09-28T03:28:03.5718294Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-28T03:28:03.5735299Z 
2019-09-28T03:28:03.5735682Z 
2019-09-28T03:28:03.5738301Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.40.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
2019-09-28T03:28:03.5740093Z 
2019-09-28T03:28:03.5740240Z 
2019-09-28T03:28:03.5746372Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-28T03:28:03.5747262Z Build completed unsuccessfully in 1:58:04
2019-09-28T03:28:03.5747262Z Build completed unsuccessfully in 1:58:04
2019-09-28T03:28:03.5799732Z == clock drift check ==
2019-09-28T03:28:03.5813263Z   local time: Sat Sep 28 03:28:03 UTC 2019
2019-09-28T03:28:04.1086971Z   network time: Sat, 28 Sep 2019 03:28:04 GMT
2019-09-28T03:28:04.1087301Z == end clock drift check ==
2019-09-28T03:28:04.9691167Z ##[error]Bash exited with code '1'.
2019-09-28T03:28:04.9760493Z ##[section]Starting: Upload CPU usage statistics
2019-09-28T03:28:04.9769136Z ==============================================================================
2019-09-28T03:28:04.9769245Z Task         : Bash
2019-09-28T03:28:04.9769467Z Description  : Run a Bash script on macOS, Linux, or Windows
