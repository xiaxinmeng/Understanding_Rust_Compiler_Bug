plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/55/c8/ead928869792a4182e52a80eb26edb24b89fcbe24e1971532565c4ea1b42/awscli-1.15.48-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 12.5MB/s eta 0:00:01
    1% |▌                               | 20kB 1.9MB/s eta 0:00:01
    2% |▊                               | 30kB 2.2MB/s eta 0:00:01
    3% |█                               | 40kB 2.0MB/s eta 0:00:01
---
    100% |████████████████████████████████| 552kB 2.0MB/s 
Collecting botocore==1.10.48 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/0b/56/44067a8f0cae5f33007e7cbdbaac67cbd9fa598c733ad25eb8f252288fe9/botocore-1.10.48-py2.py3-none-any.whl (4.4MB)
    0% |                                | 10kB 45.3MB/s eta 0:00:01
    0% |▏                               | 20kB 42.5MB/s eta 0:00:01
    0% |▎                               | 30kB 49.0MB/s eta 0:00:01
    0% |▎                               | 40kB 51.6MB/s eta 0:00:01
---
[00:51:57] ....................................................................................................
[00:52:09] ....................................................................................................
[00:52:33] ..................................................................i.................................
[00:52:47] ...................................................................................................i
[00:53:01] ................................................F......................................i............
[00:53:12] .............................................................test [run-pass] run-pass/issue-29227.rs has been running for over 60 seconds
[00:53:47] ....................................................................................................
[00:54:03] ....................................................................................................
[00:54:20] ...............................................................i....................................
[00:54:38] ..............................................................i.....................................
---
[00:58:04] ---- [run-pass] run-pass/issue-44056.rs stdout ----
[00:58:04] 
[00:58:04] error: compilation failed!
[00:58:04] status: exit code: 101
[00:58:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issue-44056.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-44056/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Ctarget-feature=+avx" "-Clto" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issue-44056/auxiliary"
[00:58:04] ------------------------------------------
[00:58:04] 
[00:58:04] ------------------------------------------
[00:58:04] stderr:
[00:58:04] stderr:
[00:58:04] ------------------------------------------
[00:58:04] error: cannot prefer dynamic linking when performing LTO
[00:58:04] 
[00:58:04] note: only 'staticlib', 'bin', and 'cdylib' outputs are supported with LTO
[00:58:04] error: aborting due to previous error
[00:58:04] 
[00:58:04] 
[00:58:04] ------------------------------------------
---
[00:58:04] 
[00:58:04] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:58:04] 
[00:58:04] 
[00:58:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:58:04] 
[00:58:04] 
[00:58:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:58:04] Build completed unsuccessfully in 0:13:01
[00:58:04] Build completed unsuccessfully in 0:13:01
[00:58:04] Makefile:58: recipe for target 'check' failed
[00:58:04] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04cdf6a5
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:00868dc0:start=1530227863606589888,finish=1530227863617234305,duration=10644417
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04af06bd
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1336cac1
$ dmesg | grep -i kill
