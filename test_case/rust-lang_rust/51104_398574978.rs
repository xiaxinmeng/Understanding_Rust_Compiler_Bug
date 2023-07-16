plain
    100% |████████████████████████████████| 61kB 6.3MB/s 
Collecting botocore==1.10.41 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/38/37/80162be9e5a50c293ab1646d35d0a4886752df96dbc5a14843733bcaecd0/botocore-1.10.41-py2.py3-none-any.whl (4.3MB)
    0% |                                | 10kB 49.3MB/s eta 0:00:01
    0% |▏                               | 20kB 16.0MB/s eta 0:00:01
    0% |▎                               | 30kB 21.2MB/s eta 0:00:01
    0% |▎                               | 40kB 12.7MB/s eta 0:00:01
---

[00:05:03] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:03] tidy error: /checkout/src/test/ui/const-unsized.rs:14: line longer than 100 chars
[00:05:03] tidy error: /checkout/src/test/ui/const-unsized.rs:20: line longer than 100 chars
[00:05:05] some tidy checks failed
[00:05:05] 
[00:05:05] 
[00:05:05] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:05] 
[00:05:05] 
[00:05:05] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:05] Build completed unsuccessfully in 0:01:54
[00:05:05] Build completed unsuccessfully in 0:01:54
[00:05:05] Makefile:79: recipe for target 'tidy' failed
[00:05:05] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0b2cb858
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:228fde7a:start=1529449734105508991,finish=1529449734111732822,duration=6223831
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:16303c80
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:066de91c
$ dmesg | grep -i kill
