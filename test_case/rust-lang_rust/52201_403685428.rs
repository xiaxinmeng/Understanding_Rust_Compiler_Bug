plain
    100% |████████████████████████████████| 1.3MB 931kB/s 
Collecting botocore==1.10.54 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/2c/84/ca1e66a4c87afdac3ca0dc720e3907e94526947d5094faf8704c0eedaa67/botocore-1.10.54-py2.py3-none-any.whl (4.4MB)
    0% |                                | 10kB 35.1MB/s eta 0:00:01
    0% |▏                               | 20kB 39.4MB/s eta 0:00:01
    0% |▎                               | 30kB 47.0MB/s eta 0:00:01
    0% |▎                               | 40kB 34.0MB/s eta 0:00:01
---

[00:04:07] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:07] tidy error: /checkout/src/libcore/sync/atomic.rs:1187: line longer than 100 chars
[00:04:08] some tidy checks failed
[00:04:08] 
[00:04:08] 
[00:04:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:08] 
[00:04:08] 
[00:04:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:08] Build completed unsuccessfully in 0:00:46
[00:04:08] Build completed unsuccessfully in 0:00:46
[00:04:08] Makefile:79: recipe for target 'tidy' failed
[00:04:08] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:09ff7b66
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:01c105bc:start=1531191351044154911,finish=1531191351053142280,duration=8987369
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:36d9fba4
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1dead568
$ dmesg | grep -i kill
