plain
travis_time:end:0e932368:start=1549059928388902721,finish=1549060016420114565,duration=88031211844
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:04:13] .................................................................................................... 1500/2946
[01:04:23] ......................................................................i............................. 1600/2946
[01:04:36] .................................................................................................... 1700/2946
[01:04:49] .................................................................................................... 1800/2946
[01:04:58] ...................................................................F..............................i. 1900/2946
[01:05:37] .................................................................................................... 2100/2946
[01:05:59] .................................................................................................... 2200/2946
[01:06:10] ...................................................................ii............................... 2300/2946
[01:06:26] .....................................i.............................................................. 2400/2946
[01:06:26] .....................................i.............................................................. 2400/2946
[01:06:38] .................................................................................................... 2500/2946
[01:07:09] .................................................................................................... 2600/2946
[01:07:19] .................................................................................................... 2700/2946
[01:07:30] .................................................................................................... 2800/2946
[01:07:42] .................................................................................................... 2900/2946
[01:07:49] .....F........................................
[01:07:49] 
[01:07:49] ---- [run-pass] run-pass/mir/mir_codegen_calls_variadic.rs stdout ----
[01:07:49] normalized stderr:
[01:07:49] normalized stderr:
[01:07:49] warning: `extern` block uses type `()` which is not FFI-safe: tuples have unspecified layout
[01:07:49]    |
[01:07:49]    |
[01:07:49] LL |     fn rust_interesting_average(_: i64, ...) -> f64;
[01:07:49]    |
[01:07:49]    = note: #[warn(improper_ctypes)] on by default
[01:07:49]    = help: consider using a struct instead
[01:07:49] 
[01:07:49] 
[01:07:49] 
[01:07:49] 
[01:07:49] 
[01:07:49] The actual stderr differed from the expected stderr.
[01:07:49] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/mir/mir_codegen_calls_variadic/mir_codegen_calls_variadic.stderr
[01:07:49] To update references, rerun the tests and pass the `--bless` flag
[01:07:49] To only update this specific test, also pass `--test-args mir/mir_codegen_calls_variadic.rs`
[01:07:49] error: 1 errors occurred comparing output.
[01:07:49] status: exit code: 0
[01:07:49] status: exit code: 0
[01:07:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/mir/mir_codegen_calls_variadic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/mir/mir_codegen_calls_variadic/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/mir/mir_codegen_calls_variadic/auxiliary"
[01:07:49] ------------------------------------------
[01:07:49] 
[01:07:49] ------------------------------------------
[01:07:49] stderr:
[01:07:49] stderr:
[01:07:49] ------------------------------------------
[01:07:49] {"message":"`extern` block uses type `()` which is not FFI-safe: tuples have unspecified layout","code":{"code":"improper_ctypes","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/mir/mir_codegen_calls_variadic.rs","byte_start":162,"byte_end":165,"line_start":6,"line_end":6,"column_start":41,"column_end":44,"is_primary":true,"text":[{"text":"    fn rust_interesting_average(_: i64, ...) -> f64;","highlight_start":41,"highlight_end":44}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(improper_ctypes)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using a struct instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"warning: `extern` block uses type `()` which is not FFI-safe: tuples have unspecified layout\n  --> /checkout/src/test/run-pass/mir/mir_codegen_calls_variadic.rs:6:41\n   |\nLL |     fn rust_interesting_average(_: i64, ...) -> f64;\n   |                                         ^^^\n   |\n   = note: #[warn(improper_ctypes)] on by default\n   = help: consider using a struct instead\n\n"}
[01:07:49] ------------------------------------------
[01:07:49] 
[01:07:49] thread '[run-pass] run-pass/mir/mir_codegen_calls_variadic.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:07:49] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:07:49] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:07:49] 
[01:07:49] ---- [run-pass] run-pass/variadic-ffi.rs stdout ----
[01:07:49] normalized stderr:
[01:07:49] warning: `extern` block uses type `()` which is not FFI-safe: tuples have unspecified layout
[01:07:49]    |
[01:07:49]    |
[01:07:49] LL |     fn rust_interesting_average(_: u64, ...) -> f64;
[01:07:49]    |
[01:07:49]    = note: #[warn(improper_ctypes)] on by default
[01:07:49]    = help: consider using a struct instead
[01:07:49] 
[01:07:49] 
[01:07:49] 
[01:07:49] 
[01:07:49] 
[01:07:49] The actual stderr differed from the expected stderr.
[01:07:49] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/variadic-ffi/variadic-ffi.stderr
[01:07:49] To update references, rerun the tests and pass the `--bless` flag
[01:07:49] To only update this specific test, also pass `--test-args variadic-ffi.rs`
[01:07:49] error: 1 errors occurred comparing output.
[01:07:49] status: exit code: 0
[01:07:49] status: exit code: 0
[01:07:49] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/variadic-ffi.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/variadic-ffi/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/variadic-ffi/auxiliary"
[01:07:49] ------------------------------------------
[01:07:49] 
[01:07:49] ------------------------------------------
[01:07:49] stderr:
[01:07:49] stderr:
[01:07:49] ------------------------------------------
[01:07:49] {"message":"`extern` block uses type `()` which is not FFI-safe: tuples have unspecified layout","code":{"code":"improper_ctypes","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/variadic-ffi.rs","byte_start":150,"byte_end":153,"line_start":5,"line_end":5,"column_start":41,"column_end":44,"is_primary":true,"text":[{"text":"    fn rust_interesting_average(_: u64, ...) -> f64;","highlight_start":41,"highlight_end":44}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(improper_ctypes)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using a struct instead","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"warning: `extern` block uses type `()` which is not FFI-safe: tuples have unspecified layout\n  --> /checkout/src/test/run-pass/variadic-ffi.rs:5:41\n   |\nLL |     fn rust_interesting_average(_: u64, ...) -> f64;\n   |                                         ^^^\n   |\n   = note: #[warn(improper_ctypes)] on by default\n   = help: consider using a struct instead\n\n"}
[01:07:49] ------------------------------------------
[01:07:49] 
[01:07:49] thread '[run-pass] run-pass/variadic-ffi.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:07:49] 
---
[01:07:49] 
[01:07:49] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:502:22
[01:07:49] 
[01:07:49] 
[01:07:49] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:07:49] 
[01:07:49] 
[01:07:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:07:49] Build completed unsuccessfully in 0:10:29
[01:07:49] Build completed unsuccessfully in 0:10:29
[01:07:49] Makefile:48: recipe for target 'check' failed
[01:07:49] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0c017076
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Feb  1 23:34:54 UTC 2019
