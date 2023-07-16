plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/e7/d9/f7e8d725a8f19dddfb4807753ad70d2935e1b8a425f4e9d5c87e06e83931/awscli-1.15.54-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 39.2MB/s eta 0:00:01
    1% |▌                               | 20kB 1.7MB/s eta 0:00:01
    2% |▊                               | 30kB 2.1MB/s eta 0:00:01
    3% |█                               | 40kB 1.8MB/s eta 0:00:01
---

[00:03:56] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:56] tidy error: /checkout/src/librustdoc/plugins.rs: missing trailing newline
[00:03:57] some tidy checks failed
[00:03:57] 
[00:03:57] 
[00:03:57] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:57] 
[00:03:57] 
[00:03:57] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:57] Build completed unsuccessfully in 0:00:48
[00:03:57] Build completed unsuccessfully in 0:00:48
[00:03:57] Makefile:79: recipe for target 'tidy' failed
[00:03:57] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:12fca9fb
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0fa3bb99:start=1531175017357319914,finish=1531175017364949793,duration=7629879
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1b29ad16
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0df10810
$ dmesg | grep -i kill
