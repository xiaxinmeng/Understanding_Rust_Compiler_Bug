plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/4e/cf/0c313db4b8e3b231447d3807657db4f5e7fad26d5eaeb294b3cfa1388a6c/awscli-1.15.84-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 12.4MB/s eta 0:00:01
    1% |▌                               | 20kB 1.9MB/s eta 0:00:01
    2% |▊                               | 30kB 2.3MB/s eta 0:00:01
    3% |█                               | 40kB 2.0MB/s eta 0:00:01
---

[00:04:24] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:24] tidy error: /checkout/src/libsyntax/feature_gate.rs:1108: line longer than 100 chars
[00:04:25] Expected a gate test for the feature 'panic_implementation'.
[00:04:25] Hint: create a failing test file named 'feature-gate-panic_implementation.rs'
[00:04:25]       in the 'ui' test suite, with its failures due to
[00:04:25]       missing usage of #![feature(panic_implementation)].
[00:04:25] Hint: If you already have such a test and don't want to rename it,
[00:04:25]       you can also add a // gate-test-panic_implementation line to the test file.
[00:04:25] tidy error: Found 1 features without a gate test.
[00:04:26] some tidy checks failed
[00:04:26] 
[00:04:26] 
[00:04:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:26] 
[00:04:26] 
[00:04:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:26] Build completed unsuccessfully in 0:00:50
[00:04:26] Build completed unsuccessfully in 0:00:50
[00:04:26] Makefile:79: recipe for target 'tidy' failed
[00:04:26] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2a83a38e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0d089130:start=1534978247571097407,finish=1534978247580420623,duration=9323216
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01b27cd2
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01b44a00
travis_time:start:01b44a00
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0e320b03
$ dmesg | grep -i kill
