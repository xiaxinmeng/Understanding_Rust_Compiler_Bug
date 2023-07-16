plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/d7/d2/04d4fb0da66c7a8850477ff1fb8f9e80597147286e118e111f7d548cf1d7/awscli-1.16.6-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 19.2MB/s eta 0:00:01
    1% |▌                               | 20kB 2.0MB/s eta 0:00:01
    2% |▊                               | 30kB 2.3MB/s eta 0:00:01
    3% |█                               | 40kB 2.1MB/s eta 0:00:01
---

[00:04:44] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:45] tidy error: /checkout/src/librustc_mir/borrow_check/nll/type_check/relate_tys.rs:550: line longer than 100 chars
[00:04:45] tidy error: /checkout/src/librustc_mir/borrow_check/nll/type_check/relate_tys.rs:632: line longer than 100 chars
[00:04:45] tidy error: /checkout/src/librustc_mir/hair/pattern/_match.rs:1238: line longer than 100 chars
[00:04:46] some tidy checks failed
[00:04:46] 
[00:04:46] 
[00:04:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:46] 
[00:04:46] 
[00:04:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:46] Build completed unsuccessfully in 0:00:51
[00:04:46] Build completed unsuccessfully in 0:00:51
[00:04:46] Makefile:79: recipe for target 'tidy' failed
[00:04:46] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:282c617d
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0687f43a:start=1535764279707614206,finish=1535764279717440833,duration=9826627
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02c91f90
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:220b578c
travis_time:start:220b578c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1ec6bfd6
$ dmesg | grep -i kill
