plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/e7/d9/f7e8d725a8f19dddfb4807753ad70d2935e1b8a425f4e9d5c87e06e83931/awscli-1.15.54-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 17.5MB/s eta 0:00:01
    1% |▌                               | 20kB 1.9MB/s eta 0:00:01
    2% |▊                               | 30kB 2.1MB/s eta 0:00:01
    3% |█                               | 40kB 1.9MB/s eta 0:00:01
---

[00:03:56] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:57] tidy error: /checkout/src/test/mir-opt/issue-49232.rs:44: trailing whitespace
[00:03:57] tidy error: /checkout/src/test/mir-opt/issue-49232.rs:47: trailing whitespace
[00:03:57] tidy error: /checkout/src/test/mir-opt/issue-49232.rs:50: trailing whitespace
[00:03:57] tidy error: /checkout/src/test/mir-opt/issue-49232.rs:58: trailing whitespace
[00:03:57] tidy error: /checkout/src/test/mir-opt/issue-49232.rs:69: trailing whitespace
[00:03:57] tidy error: /checkout/src/test/mir-opt/issue-49232.rs:75: trailing whitespace
[00:03:57] tidy error: /checkout/src/test/mir-opt/issue-49232.rs:79: trailing whitespace
[00:03:57] tidy error: /checkout/src/test/mir-opt/issue-49232.rs:82: trailing whitespace
[00:03:57] tidy error: /checkout/src/test/mir-opt/issue-49232.rs:85: trailing whitespace
[00:03:57] tidy error: /checkout/src/test/mir-opt/issue-49232.rs:88: trailing whitespace
[00:03:57] tidy error: /checkout/src/test/mir-opt/issue-49232.rs:91: trailing whitespace
[00:03:57] tidy error: /checkout/src/test/mir-opt/issue-49232.rs:94: trailing whitespace
[00:03:57] tidy error: /checkout/src/test/mir-opt/issue-49232.rs:97: trailing whitespace
[00:03:57] tidy error: /checkout/src/test/mir-opt/issue-49232.rs:100: trailing whitespace
[00:03:57] tidy error: /checkout/src/test/mir-opt/issue-49232.rs:114: trailing whitespace
[00:03:57] tidy error: /checkout/src/test/mir-opt/issue-49232.rs:118: trailing whitespace
[00:03:57] tidy error: /checkout/src/test/mir-opt/issue-49232.rs:122: trailing whitespace
[00:03:57] tidy error: /checkout/src/test/mir-opt/issue-49232.rs:126: trailing whitespace
[00:03:57] tidy error: /checkout/src/test/mir-opt/issue-49232.rs:130: trailing whitespace
[00:03:57] tidy error: /checkout/src/test/mir-opt/issue-49232.rs:134: trailing whitespace
[00:03:57] tidy error: /checkout/src/test/mir-opt/issue-49232.rs:139: trailing whitespace
[00:03:57] tidy error: /checkout/src/test/mir-opt/issue-49232.rs:143: trailing whitespace
[00:03:57] tidy error: /checkout/src/test/mir-opt/issue-49232.rs:147: trailing whitespace
[00:03:57] tidy error: /checkout/src/test/mir-opt/issue-49232.rs:151: trailing whitespace
[00:03:57] tidy error: /checkout/src/test/mir-opt/issue-49232.rs:155: trailing whitespace
[00:03:57] tidy error: /checkout/src/test/mir-opt/issue-49232.rs:159: trailing whitespace
[00:03:57] tidy error: /checkout/src/test/mir-opt/issue-49232.rs:163: trailing whitespace
[00:03:57] tidy error: /checkout/src/test/mir-opt/issue-49232.rs:169: trailing whitespace
[00:03:57] tidy error: /checkout/src/test/mir-opt/issue-49232.rs:181: trailing whitespace
[00:03:58] some tidy checks failed
[00:03:58] 
[00:03:58] 
[00:03:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:58] 
[00:03:58] 
[00:03:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:58] Build completed unsuccessfully in 0:00:50
[00:03:58] Build completed unsuccessfully in 0:00:50
[00:03:58] Makefile:79: recipe for target 'tidy' failed
[00:03:58] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2ddd9c84
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:10237be2:start=1531174307364301347,finish=1531174307373984434,duration=9683087
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00e5b5a0
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1ad76094
$ dmesg | grep -i kill
