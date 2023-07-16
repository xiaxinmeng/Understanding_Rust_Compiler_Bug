plain
2019-10-07T05:33:36.1694113Z failures:
2019-10-07T05:33:36.1694192Z 
2019-10-07T05:33:36.1694657Z ---- [ui] ui/where-clauses/where-for-self-2.rs stdout ----
2019-10-07T05:33:36.1694734Z 
2019-10-07T05:33:36.1694809Z error: Error: expected failure status (Some(1)) but received status Some(101).
2019-10-07T05:33:36.1694910Z status: exit code: 101
2019-10-07T05:33:36.1696016Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/where-clauses/where-for-self-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/where-clauses/where-for-self-2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/where-clauses/where-for-self-2/auxiliary" "-A" "unused"
2019-10-07T05:33:36.1697543Z ------------------------------------------
2019-10-07T05:33:36.1697600Z 
2019-10-07T05:33:36.1697867Z ------------------------------------------
2019-10-07T05:33:36.1697944Z stderr:
2019-10-07T05:33:36.1697944Z stderr:
2019-10-07T05:33:36.1698248Z ------------------------------------------
2019-10-07T05:33:36.1698606Z thread 'rustc' panicked at 'assertion failed: !value.has_escaping_bound_vars()', src/librustc/ty/sty.rs:896:9
2019-10-07T05:33:36.1698944Z 
2019-10-07T05:33:36.1699016Z error: internal compiler error: unexpected panic
2019-10-07T05:33:36.1699082Z 
2019-10-07T05:33:36.1699155Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-07T05:33:36.1699155Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-07T05:33:36.1699206Z 
2019-10-07T05:33:36.1699580Z note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
2019-10-07T05:33:36.1699670Z 
2019-10-07T05:33:36.1699956Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-07T05:33:36.1700014Z 
2019-10-07T05:33:36.1700350Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-07T05:33:36.1700459Z 
2019-10-07T05:33:36.1700712Z ------------------------------------------
2019-10-07T05:33:36.1700765Z 
2019-10-07T05:33:36.1700818Z 
---
2019-10-07T05:33:36.1701949Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-07T05:33:36.1702057Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-07T05:33:36.1702142Z 
2019-10-07T05:33:36.1702177Z 
2019-10-07T05:33:36.1704115Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.40.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-07T05:33:36.1704816Z 
2019-10-07T05:33:36.1704854Z 
2019-10-07T05:33:36.1704952Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-07T05:33:36.1705040Z Build completed unsuccessfully in 1:37:42
2019-10-07T05:33:36.1705040Z Build completed unsuccessfully in 1:37:42
2019-10-07T05:33:36.1705134Z == clock drift check ==
2019-10-07T05:33:36.1705207Z   local time: Mon Oct  7 05:33:35 UTC 2019
2019-10-07T05:33:36.1705365Z   network time: Mon, 07 Oct 2019 05:33:35 GMT
2019-10-07T05:33:36.1705440Z == end clock drift check ==
2019-10-07T05:33:37.2179742Z ##[error]Bash exited with code '1'.
2019-10-07T05:33:37.2220971Z ##[section]Starting: Upload CPU usage statistics
2019-10-07T05:33:37.2233301Z ==============================================================================
2019-10-07T05:33:37.2233381Z Task         : Bash
2019-10-07T05:33:37.2233459Z Description  : Run a Bash script on macOS, Linux, or Windows
