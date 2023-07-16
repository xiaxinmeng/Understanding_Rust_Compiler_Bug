plain
    100% |████████████████████████████████| 61kB 6.3MB/s 
Collecting botocore==1.10.41 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/38/37/80162be9e5a50c293ab1646d35d0a4886752df96dbc5a14843733bcaecd0/botocore-1.10.41-py2.py3-none-any.whl (4.3MB)
    0% |                                | 10kB 33.0MB/s eta 0:00:01
    0% |▏                               | 20kB 16.4MB/s eta 0:00:01
    0% |▎                               | 30kB 19.8MB/s eta 0:00:01
    0% |▎                               | 40kB 9.5MB/s eta 0:00:01
---

[00:05:13] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:13] tidy error: /checkout/src/test/parse-fail/issue-20711-2.rs:19: line longer than 100 chars
[00:05:13] tidy error: /checkout/src/test/parse-fail/issue-20711.rs:17: line longer than 100 chars
[00:05:13] tidy error: /checkout/src/test/parse-fail/trait-pub-assoc-ty.rs:13: line longer than 100 chars
[00:05:13] tidy error: /checkout/src/test/parse-fail/trait-pub-assoc-const.rs:13: line longer than 100 chars
[00:05:13] tidy error: /checkout/src/test/parse-fail/trait-pub-method.rs:13: line longer than 100 chars
[00:05:13] tidy error: /checkout/src/test/parse-fail/removed-syntax-static-fn.rs:18: line longer than 100 chars
[00:05:15] some tidy checks failed
[00:05:15] 
[00:05:15] 
[00:05:15] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:15] 
[00:05:15] 
[00:05:15] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:15] Build completed unsuccessfully in 0:01:55
[00:05:15] Build completed unsuccessfully in 0:01:55
[00:05:15] make: *** [tidy] Error 1
[00:05:15] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0ae32a9c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:33a0db14:start=1529446569192764490,finish=1529446569199525400,duration=6760910
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0070dc41
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0650a45f
$ dmesg | grep -i kill
