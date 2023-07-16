plain
    100% |████████████████████████████████| 61kB 4.8MB/s 
Collecting botocore==1.10.41 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/38/37/80162be9e5a50c293ab1646d35d0a4886752df96dbc5a14843733bcaecd0/botocore-1.10.41-py2.py3-none-any.whl (4.3MB)
    0% |                                | 10kB 7.4MB/s eta 0:00:01
    0% |▏                               | 20kB 6.8MB/s eta 0:00:01
    0% |▎                               | 30kB 9.7MB/s eta 0:00:01
    0% |▎                               | 40kB 10.5MB/s eta 0:00:01
---

[00:05:09] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:09] tidy error: duplicate error code: 697
[00:05:09] tidy error: /checkout/src/libsyntax/diagnostic_list.rs:400:     E0697, // invalid ABI
[00:05:09] tidy error: /checkout/src/librustc/diagnostics.rs:2134:     E0697, // closures cannot be static
[00:05:09] tidy error: duplicate error code: 698
[00:05:09] tidy error: /checkout/src/libsyntax/diagnostic_list.rs:401:     E0698, // incorrect visibility restriction
[00:05:09] tidy error: /checkout/src/librustc_typeck/diagnostics.rs:4794:     E0698, // type inside generator must be known in this context
[00:05:11] some tidy checks failed
[00:05:11] 
[00:05:11] 
[00:05:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:11] 
[00:05:11] 
[00:05:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:11] Build completed unsuccessfully in 0:01:56
[00:05:11] Build completed unsuccessfully in 0:01:56
[00:05:11] Makefile:79: recipe for target 'tidy' failed
[00:05:11] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0537cd14
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0323f5ea:start=1529449265760608394,finish=1529449265767095704,duration=6487310
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0a1fa788
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01748ece
$ dmesg | grep -i kill
