plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/ab/4e/ae2cf04ce20a86790605a67c0a5010df912625dc826a3b1c82ea71746668/awscli-1.16.54-py2.py3-none-any.whl (1.4MB)
    0% |▎                               | 10kB 10.9MB/s eta 0:00:01
    1% |▌                               | 20kB 1.9MB/s eta 0:00:01
    2% |▊                               | 30kB 2.2MB/s eta 0:00:01
    2% |█                               | 40kB 2.0MB/s eta 0:00:01
---
[01:23:13] ---- [run-pass] run-pass/command-exec.rs stdout ----
[01:23:13] 
[01:23:13] error: test run failed!
[01:23:13] status: exit code: 101
[01:23:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "run" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/command-exec/a"
[01:23:13] ------------------------------------------
[01:23:13] uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/command-exec/a", waiting for result
[01:23:13] 
[01:23:13] ------------------------------------------
[01:23:13] ------------------------------------------
[01:23:13] stderr:
[01:23:13] ------------------------------------------
[01:23:13] thread 'main' panicked at 'assertion failed: output.status.success()', /checkout/src/test/run-pass/command-exec.rs:99:5
[01:23:13] 
[01:23:13] ------------------------------------------
[01:23:13] 
[01:23:13] thread '[run-pass] run-pass/command-exec.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
---
[01:23:13] 
[01:23:13] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[01:23:13] 
[01:23:13] 
[01:23:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-arm-linux-androideabi" "--mode" "run-pass" "--target" "arm-linux-androideabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--remote-test-client" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "/android/ndk/arm-14" "--color" "always"
[01:23:13] 
[01:23:13] 
[01:23:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target arm-linux-androideabi
[01:23:13] Build completed unsuccessfully in 1:11:14
---
travis_time:end:056cab44:start=1542163449675642587,finish=1542163449694083174,duration=18440587
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00532b1b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2b26fcf0
travis_time:start:2b26fcf0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2a84b523
$ dmesg | grep -i kill
