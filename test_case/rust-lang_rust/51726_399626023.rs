plain
    100% |████████████████████████████████| 1.3MB 857kB/s 
Collecting botocore==1.10.45 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/2a/c8/b180fb83fa46d2b56ea059177dc3c00647d622987daf5e7ffbc658555ede/botocore-1.10.45-py2.py3-none-any.whl (4.4MB)
    0% |                                | 10kB 27.4MB/s eta 0:00:01
    0% |▏                               | 20kB 31.3MB/s eta 0:00:01
    0% |▎                               | 30kB 38.7MB/s eta 0:00:01
    0% |▎                               | 40kB 29.0MB/s eta 0:00:01
---
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:59:42] 
[00:59:42] running 94 tests
[01:01:27] ....................................FF..................F.........F..........test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
[01:04:04] ...F.............
[01:04:04] 
[01:04:04] ---- [run-pass] run-pass-fulldeps/macro-quote-cond.rs stdout ----
[01:04:04] 
[01:04:04] 
[01:04:04] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/cond_plugin.rs" failed to compile: 
[01:04:04] status: exit code: 101
[01:04:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/auxiliary/cond_plugin.rs" "--target=x86_64-unknown-linux-gnu" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-quote-cond/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-quote-cond/auxiliary"
[01:04:04] ------------------------------------------
[01:04:04] 
[01:04:04] ------------------------------------------
[01:04:04] stderr:
[01:04:04] stderr:
[01:04:04] ------------------------------------------
[01:04:04] thread 'main' panicked at 'expansion info is reset for the mark 4
[01:04:04] old: ExpnInfo {
[01:04:04]     call_site: Span {
[01:04:04]         lo: BytePos(
[01:04:04]         ),
[01:04:04]         ),
[01:04:04]         hi: BytePos(
[01:04:04]         ),
[01:04:04]         ),
[01:04:04]         ctxt: #0
[01:04:04]     },
[01:04:04]     callee: NameAndSpan {
[01:04:04]         format: MacroBang(
[01:04:04]         ),
[01:04:04]         allow_internal_unstable: false,
[01:04:04]         allow_internal_unsafe: false,
[01:04:04]         edition: Edition2015,
[01:04:04]         edition: Edition2015,
[01:04:04]         span: None
[01:04:04]     }
[01:04:04] }
[01:04:04] new: ExpnInfo {
[01:04:04]     call_site: Span {
[01:04:04]         lo: BytePos(
[01:04:04]         ),
[01:04:04]         ),
[01:04:04]         hi: BytePos(
[01:04:04]         ),
[01:04:04]         ),
[01:04:04]         ctxt: #0
[01:04:04]     },
[01:04:04]     callee: NameAndSpan {
[01:04:04]         format: MacroBang(
[01:04:04]         ),
[01:04:04]         allow_internal_unstable: true,
[01:04:04]         allow_internal_unsafe: false,
[01:04:04]         edition: Edition2015,
---
[01:04:04] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:04:04] 
[01:04:04] error: internal compiler error: unexpected panic
[01:04:04] 
[01:04:04] note: the compiler unexpectedly panicked. this is a bug.
[01:04:04] 
[01:04:04] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:04:04] note: rustc 1.28.0-dev running on x86_64-unknown-linux-gnu
[01:04:04] 
[01:04:04] 
[01:04:04] note: compiler flags: -Z unstable-options -C rpath
[01:04:04] 
[01:04:04] ------------------------------------------
[01:04:04] 
[01:04:04] thread '[run-pass] run-pass-fulldeps/macro-quote-cond.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[01:04:04] thread '[run-pass] run-pass-fulldeps/macro-quote-cond.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[01:04:04] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:04:04] 
[01:04:04] ---- [run-pass] run-pass-fulldeps/macro-quote-test.rs stdout ----
[01:04:04] 
[01:04:04] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/hello_macro.rs" failed to compile: 
[01:04:04] status: exit code: 101
[01:04:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/auxiliary/hello_macro.rs" "--target=x86_64-unknown-linux-gnu" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-quote-test/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-quote-test/auxiliary"
[01:04:04] ------------------------------------------
[01:04:04] 
[01:04:04] ------------------------------------------
[01:04:04] stderr:
[01:04:04] stderr:
[01:04:04] ------------------------------------------
[01:04:04] thread 'main' panicked at 'expansion info is reset for the mark 2
[01:04:04] old: ExpnInfo {
[01:04:04]     call_site: Span {
[01:04:04]         lo: BytePos(
[01:04:04]         ),
[01:04:04]         ),
[01:04:04]         hi: BytePos(
[01:04:04]         ),
[01:04:04]         ),
[01:04:04]         ctxt: #0
[01:04:04]     },
[01:04:04]     callee: NameAndSpan {
[01:04:04]         format: MacroBang(
[01:04:04]         ),
[01:04:04]         allow_internal_unstable: false,
[01:04:04]         allow_internal_unsafe: false,
[01:04:04]         edition: Edition2015,
[01:04:04]         edition: Edition2015,
[01:04:04]         span: None
[01:04:04]     }
[01:04:04] }
[01:04:04] new: ExpnInfo {
[01:04:04]     call_site: Span {
[01:04:04]         lo: BytePos(
[01:04:04]         ),
[01:04:04]         ),
[01:04:04]         hi: BytePos(
[01:04:04]         ),
[01:04:04]         ),
[01:04:04]         ctxt: #0
[01:04:04]     },
[01:04:04]     callee: NameAndSpan {
[01:04:04]         format: MacroBang(
[01:04:04]         ),
[01:04:04]         allow_internal_unstable: true,
[01:04:04]         allow_internal_unsafe: false,
[01:04:04]         edition: Edition2015,
[01:04:04]         edition: Edition2015,
[01:04:04]         span: None
[01:04:04]     }
[01:04:04] }', libsyntax_pos/hygiene.rs:111:17
[01:04:04] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:04:04] 
[01:04:04] error: internal compiler error: unexpected panic
[01:04:04] 
[01:04:04] note: the compiler unexpectedly panicked. this is a bug.
[01:04:04] 
[01:04:04] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:04:04] note: rustc 1.28.0-dev running on x86_64-unknown-linux-gnu
[01:04:04] 
[01:04:04] 
[01:04:04] note: compiler flags: -Z unstable-options -C rpath
[01:04:04] 
[01:04:04] ------------------------------------------
[01:04:04] 
[01:04:04] thread '[run-pass] run-pass-fulldeps/macro-quote-test.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[01:04:04] thread '[run-pass] run-pass-fulldeps/macro-quote-test.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[01:04:04] 
[01:04:04] ---- [run-pass] run-pass-fulldeps/proc-macro/count_compound_ops.rs stdout ----
[01:04:04] 
[01:04:04] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/proc-macro/auxiliary/count_compound_ops.rs" failed to compile: 
[01:04:04] status: exit code: 101
[01:04:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/proc-macro/auxiliary/count_compound_ops.rs" "--target=x86_64-unknown-linux-gnu" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/proc-macro/count_compound_ops/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/proc-macro/count_compound_ops/auxiliary"
[01:04:04] ------------------------------------------
[01:04:04] 
[01:04:04] ------------------------------------------
[01:04:04] stderr:
[01:04:04] stderr:
[01:04:04] ------------------------------------------
[01:04:04] thread 'main' panicked at 'expansion info is reset for the mark 3
[01:04:04] old: ExpnInfo {
[01:04:04]     call_site: Span {
[01:04:04]         lo: BytePos(
[01:04:04]         ),
[01:04:04]         ),
[01:04:04]         hi: BytePos(
[01:04:04]         ),
[01:04:04]         ),
[01:04:04]         ctxt: #0
[01:04:04]     },
[01:04:04]     callee: NameAndSpan {
[01:04:04]         format: MacroBang(
[01:04:04]         ),
[01:04:04]         allow_internal_unstable: false,
[01:04:04]         allow_internal_unsafe: false,
[01:04:04]         edition: Edition2015,
[01:04:04]         edition: Edition2015,
[01:04:04]         span: None
[01:04:04]     }
[01:04:04] }
[01:04:04] new: ExpnInfo {
[01:04:04]     call_site: Span {
[01:04:04]         lo: BytePos(
[01:04:04]         ),
[01:04:04]         ),
[01:04:04]         hi: BytePos(
[01:04:04]         ),
[01:04:04]         ),
[01:04:04]         ctxt: #0
[01:04:04]     },
[01:04:04]     callee: NameAndSpan {
[01:04:04]         format: MacroBang(
[01:04:04]         ),
[01:04:04]         allow_internal_unstable: true,
[01:04:04]         allow_internal_unsafe: false,
[01:04:04]         edition: Edition2015,
[01:04:04]         edition: Edition2015,
[01:04:04]         span: None
[01:04:04]     }
[01:04:04] }', libsyntax_pos/hygiene.rs:111:17
[01:04:04] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:04:04] 
[01:04:04] error: internal compiler error: unexpected panic
[01:04:04] 
[01:04:04] note: the compiler unexpectedly panicked. this is a bug.
[01:04:04] 
[01:04:04] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:04:04] note: rustc 1.28.0-dev running on x86_64-unknown-linux-gnu
[01:04:04] 
[01:04:04] 
[01:04:04] note: compiler flags: -Z unstable-options -C rpath
[01:04:04] 
[01:04:04] ------------------------------------------
[01:04:04] 
[01:04:04] thread '[run-pass] run-pass-fulldeps/proc-macro/count_compound_ops.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[01:04:04] thread '[run-pass] run-pass-fulldeps/proc-macro/count_compound_ops.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[01:04:04] 
[01:04:04] ---- [run-pass] run-pass-fulldeps/proc-macro/hygiene_example.rs stdout ----
[01:04:04] 
[01:04:04] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/proc-macro/auxiliary/hygiene_example_codegen.rs" failed to compile: 
[01:04:04] status: exit code: 101
[01:04:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/proc-macro/auxiliary/hygiene_example_codegen.rs" "--target=x86_64-unknown-linux-gnu" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/proc-macro/hygiene_example/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/proc-macro/hygiene_example/auxiliary"
[01:04:04] ------------------------------------------
[01:04:04] 
[01:04:04] ------------------------------------------
[01:04:04] stderr:
[01:04:04] stderr:
[01:04:04] ------------------------------------------
[01:04:04] thread 'main' panicked at 'expansion info is reset for the mark 2
[01:04:04] old: ExpnInfo {
[01:04:04]     call_site: Span {
[01:04:04]         lo: BytePos(
[01:04:04]         ),
[01:04:04]         ),
[01:04:04]         hi: BytePos(
[01:04:04]         ),
[01:04:04]         ),
[01:04:04]         ctxt: #0
[01:04:04]     },
[01:04:04]     callee: NameAndSpan {
[01:04:04]         format: MacroBang(
[01:04:04]         ),
[01:04:04]         allow_internal_unstable: false,
[01:04:04]         allow_internal_unsafe: false,
[01:04:04]         edition: Edition2015,
[01:04:04]         edition: Edition2015,
[01:04:04]         span: None
[01:04:04]     }
[01:04:04] }
[01:04:04] new: ExpnInfo {
[01:04:04]     call_site: Span {
[01:04:04]         lo: BytePos(
[01:04:04]         ),
[01:04:04]         ),
[01:04:04]         hi: BytePos(
[01:04:04]         ),
[01:04:04]         ),
[01:04:04]         ctxt: #0
[01:04:04]     },
[01:04:04]     callee: NameAndSpan {
[01:04:04]         format: MacroBang(
[01:04:04]         ),
[01:04:04]         allow_internal_unstable: true,
[01:04:04]         allow_internal_unsafe: false,
[01:04:04]         edition: Edition2015,
[01:04:04]         edition: Edition2015,
[01:04:04]         span: None
[01:04:04]     }
[01:04:04] }', libsyntax_pos/hygiene.rs:111:17
[01:04:04] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:04:04] 
[01:04:04] error: internal compiler error: unexpected panic
[01:04:04] 
[01:04:04] note: the compiler unexpectedly panicked. this is a bug.
[01:04:04] 
[01:04:04] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:04:04] note: rustc 1.28.0-dev running on x86_64-unknown-linux-gnu
[01:04:04] 
[01:04:04] 
[01:04:04] note: compiler flags: -Z unstable-options -C rpath
[01:04:04] 
[01:04:04] ------------------------------------------
[01:04:04] 
[01:04:04] thread '[run-pass] run-pass-fulldeps/proc-macro/hygiene_example.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[01:04:04] thread '[run-pass] run-pass-fulldeps/proc-macro/hygiene_example.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[01:04:04] 
[01:04:04] ---- [run-pass] run-pass-fulldeps/proc_macro.rs stdout ----
[01:04:04] 
[01:04:04] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/proc_macro_def.rs" failed to compile: 
[01:04:04] status: exit code: 101
[01:04:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/auxiliary/proc_macro_def.rs" "--target=x86_64-unknown-linux-gnu" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/proc_macro/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/proc_macro/auxiliary"
[01:04:04] ------------------------------------------
[01:04:04] 
[01:04:04] ------------------------------------------
[01:04:04] stderr:
[01:04:04] stderr:
[01:04:04] ------------------------------------------
[01:04:04] thread 'main' panicked at 'expansion info is reset for the mark 2
[01:04:04] old: ExpnInfo {
[01:04:04]     call_site: Span {
[01:04:04]         lo: BytePos(
[01:04:04]         ),
[01:04:04]         ),
[01:04:04]         hi: BytePos(
[01:04:04]         ),
[01:04:04]         ),
[01:04:04]         ctxt: #0
[01:04:04]     },
[01:04:04]     callee: NameAndSpan {
[01:04:04]         format: MacroBang(
[01:04:04]         ),
[01:04:04]         allow_internal_unstable: false,
[01:04:04]         allow_internal_unsafe: false,
[01:04:04]         edition: Edition2015,
[01:04:04]         edition: Edition2015,
[01:04:04]         span: None
[01:04:04]     }
[01:04:04] }
[01:04:04] new: ExpnInfo {
[01:04:04]     call_site: Span {
[01:04:04]         lo: BytePos(
[01:04:04]         ),
[01:04:04]         ),
[01:04:04]         hi: BytePos(
[01:04:04]         ),
[01:04:04]         ),
[01:04:04]         ctxt: #0
[01:04:04]     },
[01:04:04]     callee: NameAndSpan {
[01:04:04]         format: MacroBang(
[01:04:04]         ),
[01:04:04]         allow_internal_unstable: true,
[01:04:04]         allow_internal_unsafe: false,
[01:04:04]         edition: Edition2015,
[01:04:04]         edition: Edition2015,
[01:04:04]         span: None
[01:04:04]     }
[01:04:04] }', libsyntax_pos/hygiene.rs:111:17
[01:04:04] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:04:04] 
[01:04:04] error: internal compiler error: unexpected panic
[01:04:04] 
[01:04:04] note: the compiler unexpectedly panicked. this is a bug.
[01:04:04] 
[01:04:04] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:04:04] note: rustc 1.28.0-dev running on x86_64-unknown-linux-gnu
[01:04:04] 
[01:04:04] 
[01:04:04] note: compiler flags: -Z unstable-options -C rpath
[01:04:04] 
[01:04:04] ------------------------------------------
[01:04:04] 
[01:04:04] thread '[run-pass] run-pass-fulldeps/proc_macro.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
---
[01:04:04] test result: FAILED. 89 passed; 5 failed; 0 ignored; 0 measured; 0 filtered out
[01:04:04] 
[01:04:04] 
[01:04:04] 
[01:04:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:04:04] 
[01:04:04] 
[01:04:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:04:04] Build completed unsuccessfully in 0:20:28
[01:04:04] Build completed unsuccessfully in 0:20:28
[01:04:04] Makefile:58: recipe for target 'check' failed
[01:04:04] make: *** [check] Error 1
Sat, 23 Jun 2018 03:05:56 GMT
travis_time:end:24ee2e04:start=1529723156183009615,finish=1529723156284799143,duration=101789528

The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
