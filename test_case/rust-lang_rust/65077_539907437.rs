plain
2019-10-09T08:54:11.8362428Z failures:
2019-10-09T08:54:11.8370550Z 
2019-10-09T08:54:11.8372261Z ---- [ui] ui/where-clauses/where-for-self-2.rs stdout ----
2019-10-09T08:54:11.8372852Z 
2019-10-09T08:54:11.8373106Z error: Error: expected failure status (Some(1)) but received status Some(101).
2019-10-09T08:54:11.8373789Z status: exit code: 101
2019-10-09T08:54:11.8375439Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/where-clauses/where-for-self-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/where-clauses/where-for-self-2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/where-clauses/where-for-self-2/auxiliary" "-A" "unused"
2019-10-09T08:54:11.8376644Z ------------------------------------------
2019-10-09T08:54:11.8376840Z 
2019-10-09T08:54:11.8377256Z ------------------------------------------
2019-10-09T08:54:11.8377641Z stderr:
2019-10-09T08:54:11.8377641Z stderr:
2019-10-09T08:54:11.8378175Z ------------------------------------------
2019-10-09T08:54:11.8378847Z thread 'rustc' panicked at 'assertion failed: !value.has_escaping_bound_vars()', src/librustc/ty/sty.rs:896:9
2019-10-09T08:54:11.8379276Z 
2019-10-09T08:54:11.8379465Z error: internal compiler error: unexpected panic
2019-10-09T08:54:11.8379606Z 
2019-10-09T08:54:11.8379772Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-09T08:54:11.8379772Z note: the compiler unexpectedly panicked. this is a bug.
2019-10-09T08:54:11.8379932Z 
2019-10-09T08:54:11.8380372Z note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
2019-10-09T08:54:11.8380598Z 
2019-10-09T08:54:11.8381652Z note: rustc 1.40.0-dev running on x86_64-unknown-linux-gnu
2019-10-09T08:54:11.8382011Z 
2019-10-09T08:54:11.8382527Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2019-10-09T08:54:11.8382925Z 
2019-10-09T08:54:11.8383338Z ------------------------------------------
2019-10-09T08:54:11.8383538Z 
2019-10-09T08:54:11.8383704Z 
---
2019-10-09T08:54:11.8390658Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-09T08:54:11.8391688Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-09T08:54:11.8405898Z 
2019-10-09T08:54:11.8406310Z 
2019-10-09T08:54:11.8408537Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.40.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-09T08:54:11.8409462Z 
2019-10-09T08:54:11.8409618Z 
2019-10-09T08:54:11.8413758Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-09T08:54:11.8414085Z Build completed unsuccessfully in 1:36:57
2019-10-09T08:54:11.8414085Z Build completed unsuccessfully in 1:36:57
2019-10-09T08:54:11.8474476Z == clock drift check ==
2019-10-09T08:54:11.8490521Z   local time: Wed Oct  9 08:54:11 UTC 2019
2019-10-09T08:54:11.9809710Z   network time: Wed, 09 Oct 2019 08:54:11 GMT
2019-10-09T08:54:11.9812071Z == end clock drift check ==
2019-10-09T08:54:12.8084696Z ##[error]Bash exited with code '1'.
2019-10-09T08:54:12.8118748Z ##[section]Starting: Upload CPU usage statistics
2019-10-09T08:54:12.8132635Z ==============================================================================
2019-10-09T08:54:12.8132740Z Task         : Bash
2019-10-09T08:54:12.8132837Z Description  : Run a Bash script on macOS, Linux, or Windows
