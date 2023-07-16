plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/3f/6c/dbbd5992740649134e597833bea5a95e1fc093a7123e9b8156d838b960e4/awscli-1.15.11-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 14.0MB/s eta 0:00:01
    1% |▌                               | 20kB 1.9MB/s eta 0:00:01
    2% |▉                               | 30kB 2.2MB/s eta 0:00:01
    3% |█                               | 40kB 2.0MB/s eta 0:00:01
---
    100% |████████████████████████████████| 61kB 6.6MB/s 
Collecting botocore==1.10.11 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/15/c0/04ec8aec3cdf7dd4887f2666044550eb3370a4f29668e53519cc7143bdcf/botocore-1.10.11-py2.py3-none-any.whl (4.2MB)
    0% |                                | 10kB 29.0MB/s eta 0:00:01
    0% |▏                               | 20kB 29.3MB/s eta 0:00:01
    0% |▎                               | 30kB 33.3MB/s eta 0:00:01
    0% |▎                               | 40kB 17.3MB/s eta 0:00:01
---
[00:59:55] ....................................................................................................
[01:00:03] .........................................................................i..........................
[01:00:12] ..................i.................................................................................
[01:00:22] ....................................................................................................
[01:00:30] ..............................................................F.....................................
[01:00:40] ..........
[01:00:40] failures:
[01:00:40] 
[01:00:40] ---- [compile-fail] compile-fail/unreachable-try-pattern.rs stdout ----
[01:00:40] ---- [compile-fail] compile-fail/unreachable-try-pattern.rs stdout ----
[01:00:40]  
[01:00:40] error: /checkout/src/test/compile-fail/unreachable-try-pattern.rs:19: unexpected warning: '19:29: 21:6: unreachable expression [unreachable_code]'
[01:00:40] 
[01:00:40] error: /checkout/src/test/compile-fail/unreachable-try-pattern.rs:20: unexpected warning: '20:9: 20:19: unreachable expression [unreachable_code]'
[01:00:40] 
[01:00:40] error: /checkout/src/test/compile-fail/unreachable-try-pattern.rs:40: unexpected warning: '40:50: 40:56: unreachable expression [unreachable_code]'
[01:00:40] 
[01:00:40] error: 3 unexpected errors found, 0 expected errors not found
[01:00:40] status: exit code: 101
[01:00:40] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/unreachable-try-pattern.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/unreachable-try-pattern.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/unreachable-try-pattern.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[01:00:40] unexpected errors (from JSON output): [
[01:00:40]     Error {
[01:00:40]         line_num: 19,
[01:00:40]         kind: Some(
[01:00:40]         ),
[01:00:40]         ),
[01:00:40]         msg: "19:29: 21:6: unreachable expression [unreachable_code]"
[01:00:40]     Error {
[01:00:40]         line_num: 20,
[01:00:40]         kind: Some(
[01:00:40]             Warning
[01:00:40]             Warning
[01:00:40]         ),
[01:00:40]         msg: "20:9: 20:19: unreachable expression [unreachable_code]"
[01:00:40]     Error {
[01:00:40]         line_num: 40,
[01:00:40]         kind: Some(
[01:00:40]             Warning
[01:00:40]             Warning
[01:00:40]         ),
[01:00:40]         msg: "40:50: 40:56: unreachable expression [unreachable_code]"
[01:00:40] ]
[01:00:40] 
[01:00:40] thread '[compile-fail] compile-fail/unreachable-try-pattern.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1278:13
[01:00:40] 
---
[01:00:40] 
[01:00:40] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:488:22
[01:00:40] 
[01:00:40] 
[01:00:40] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/compile-fail" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:00:40] 
[01:00:40] 
[01:00:40] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:00:40] Build completed unsuccessfully in 0:17:19
