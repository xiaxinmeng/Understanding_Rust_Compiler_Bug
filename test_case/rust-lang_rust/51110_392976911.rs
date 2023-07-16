plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/40/7b/dd2e1823627f38afb7e92e4e9792d81902c084ab7501c1f748169a5336ae/awscli-1.15.29-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 6.0MB/s eta 0:00:01
    1% |▌                               | 20kB 1.3MB/s eta 0:00:01
    2% |▉                               | 30kB 1.5MB/s eta 0:00:01
    3% |█                               | 40kB 1.3MB/s eta 0:00:01
---
[00:44:55] ..............................................................i.....................................
[00:44:59] ....................................................................................................
[00:45:04] ....................................................................................................
[00:45:11] ...........................................................................................i........
[00:45:14] .........iiiiiiiii...................................................
[00:45:14] 
[00:45:14] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:46:02] ..............................................................i.....................................
[00:46:06] ....................................................................................................
[00:46:11] ....................................................................................................
[00:46:17] ...........................................................................................i........
[00:46:20] .........iiiiiiiii...................................................
[00:46:20] 
[00:46:20]  finished in 66.019
[00:46:20] travis_fold:end:test_ui_nll

---
[00:56:17] .ii.................................................................................................
[00:56:24] ....................................................................................................
[00:56:28] ....................................................................................................
[00:56:32] ..........................................................................i.........................
[00:56:38] ....................i.....................................F.........................................
[00:56:47] ....................................................................................................
[00:56:51] ....................................................................................................
[00:56:52] ................
[00:56:52] failures:
[00:56:52] failures:
[00:56:52] 
[00:56:52] ---- [compile-fail] compile-fail/static-array-across-crate.rs stdout ----
[00:56:52] 
[00:56:52] error: compiler panicked
[00:56:52] status: exit code: 101
[00:56:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/static-array-across-crate.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/static-array-across-crate/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/static-array-across-crate/auxiliary" "-A" "unused"
[00:56:52] ------------------------------------------
[00:56:52] 
[00:56:52] ------------------------------------------
[00:56:52] stderr:
[00:56:52] stderr:
[00:56:52] ------------------------------------------
[00:56:52] thread 'main' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:945:5
[00:56:52] 
[00:56:52] error: internal compiler error: unexpected panic
[00:56:52] 
[00:56:52] 
[00:56:52] note: the compiler unexpectedly panicked. this is a bug.
[00:56:52] 
[00:56:52] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:56:52] note: rustc 1.28.0-dev running on x86_64-unknown-linux-gnu
[00:56:52] 
[00:56:52] 
[00:56:52] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:56:52] 
[00:56:52] ------------------------------------------
[00:56:52] 
[00:56:52] thread '[compile-fail] compile-fail/static-array-across-crate.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3053:9
---
[00:56:52] 
[00:56:52] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:56:52] 
[00:56:52] 
[00:56:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:56:52] 
[00:56:52] 
[00:56:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:56:52] Build completed unsuccessfully in 0:14:13
[00:56:52] Build completed unsuccessfully in 0:14:13
[00:56:52] make: *** [check] Error 1
[00:56:52] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:079ea7f2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
