plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/f5/bf/3da21b6dc4c32bc8e60a4d145b1cf74944dc34557322a123795c6d13207f/awscli-1.16.34-py2.py3-none-any.whl (1.4MB)
    0% |▎                               | 10kB 27.1MB/s eta 0:00:01
    1% |▌                               | 20kB 2.0MB/s eta 0:00:01
    2% |▊                               | 30kB 2.3MB/s eta 0:00:01
    3% |█                               | 40kB 2.1MB/s eta 0:00:01
---
[00:46:27] failures:
[00:46:27] 
[00:46:27] ---- [ui] ui/iterators/array-of-ranges.rs stdout ----
[00:46:27] 
[00:46:27] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:46:27] status: exit code: 101
[00:46:27] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/iterators/array-of-ranges.rs" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/array-of-ranges/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/array-of-ranges/auxiliary" "-A" "unused"
[00:46:27] ------------------------------------------
[00:46:27] 
[00:46:27] ------------------------------------------
[00:46:27] stderr:
---
[00:46:27] note: the compiler unexpectedly panicked. this is a bug.
[00:46:27] 
[00:46:27] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:46:27] 
[00:46:27] note: rustc 1.31.0-nightly (d502144fd 2018-10-16) running on x86_64-unknown-linux-gnu
[00:46:27] 
[00:46:27] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C linker=cc
[00:46:27] 
[00:46:27] ------------------------------------------
[00:46:27] 
[00:46:27] thread '[ui] ui/iterators/array-of-ranges.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3277:9
---
[00:46:27] 
[00:46:27] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:501:22
[00:46:27] 
[00:46:27] 
[00:46:27] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "ui" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:46:27] 
[00:46:27] 
[00:46:27] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
[00:46:27] Build completed unsuccessfully in 0:43:14
---
travis_time:end:02799a0c:start=1539655478254320481,finish=1539655478260572730,duration=6252249
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:3e37701a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2a29cbf4
travis_time:start:2a29cbf4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:23c122f8
$ dmesg | grep -i kill
