plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/26/a5/981fcedb0cba5cd167382878c92956229d14f0ac6846c71f2cd87906d474/awscli-1.15.10-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 13.7MB/s eta 0:00:01
    1% |▌                               | 20kB 1.9MB/s eta 0:00:01
    2% |▉                               | 30kB 2.2MB/s eta 0:00:01
    3% |█                               | 40kB 1.9MB/s eta 0:00:01
---
  Downloading https://files.pythonhosted.org/packages/b7/8e/ddb32ddaabd431813e180ca224e844bab8ad42fbb47ee07553f0ec44cd86/colorama-0.3.7-py2.py3-none-any.whl
Collecting botocore==1.10.10 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/77/c9/a40ebce24bbab4c7986fccdac9dade097385ad2feae73dcc47d31a1b4dc8/botocore-1.10.10-py2.py3-none-any.whl (4.2MB)
    0% |                                | 10kB 33.1MB/s eta 0:00:01
    0% |▏                               | 20kB 34.9MB/s eta 0:00:01
    0% |▎                               | 30kB 41.9MB/s eta 0:00:01
    0% |▎                               | 40kB 16.2MB/s eta 0:00:01
---
[00:58:26] ....................................................................................................
[00:58:35] ....................................................................................................
[00:58:45] ....................................................................................................
[00:58:57] ....................................................................................................
[00:59:05] ...i...................................................F............................................
[00:59:23] ....................................................................................................
[00:59:32] ....................................................................................................
[00:59:40] .........................................................................i..........................
[00:59:50] ..................i.................................................................................
[00:59:50] ..................i.................................................................................
[01:00:00] ....................................................................................................
[01:00:09] ......................................F.............................................................
[01:00:19] .........
[01:00:19] failures:
[01:00:19] 
[01:00:19] ---- [compile-fail] compile-fail/match-no-arms-unreachable-after.rs stdout ----
[01:00:19] ---- [compile-fail] compile-fail/match-no-arms-unreachable-after.rs stdout ----
[01:00:19]  
[01:00:19] error: compile-fail test compiled successfully!
[01:00:19] status: exit code: 0
[01:00:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/match-no-arms-unreachable-after.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/match-no-arms-unreachable-after.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/match-no-arms-unreachable-after.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[01:00:19] ------------------------------------------
[01:00:19] 
[01:00:19] ------------------------------------------
[01:00:19] stderr:
---
[01:00:19] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:00:19] 
[01:00:19] ---- [compile-fail] compile-fail/uninhabited-matches-feature-gated.rs stdout ----
[01:00:19]  
[01:00:19] error: /checkout/src/test/compile-fail/uninhabited-matches-feature-gated.rs:23: expected error not found: non-exhaustive
[01:00:19] 
[01:00:19] error: 0 unexpected errors found, 1 expected errors not found
[01:00:19] status: exit code: 101
[01:00:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/uninhabited-matches-feature-gated.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/uninhabited-matches-feature-gated.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/uninhabited-matches-feature-gated.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[01:00:19] not found errors (from test file): [
[01:00:19]     Error {
[01:00:19]         line_num: 23,
[01:00:19]         kind: Some(
[01:00:19]             Error
[01:00:19]         ),
[01:00:19]         msg: "non-exhaustive"
[01:00:19] ]
[01:00:19] 
[01:00:19] thread '[compile-fail] compile-fail/uninhabited-matches-feature-gated.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1276:13
[01:00:19] 
---
[01:00:19] 
[01:00:19] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:488:22
[01:00:19] 
[01:00:19] 
[01:00:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:00:19] 
[01:00:19] 
[01:00:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:00:19] Build completed unsuccessfully in 0:17:33
[01:00:19] Build completed unsuccessfully in 0:17:33
[01:00:19] Makefile:58: recipe for target 'check' failed
[01:00:19] make: *** [check] Error 1
