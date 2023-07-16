plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/f5/bf/3da21b6dc4c32bc8e60a4d145b1cf74944dc34557322a123795c6d13207f/awscli-1.16.34-py2.py3-none-any.whl (1.4MB)
    0% |▎                               | 10kB 6.2MB/s eta 0:00:01
    1% |▌                               | 20kB 1.2MB/s eta 0:00:02
    2% |▊                               | 30kB 1.4MB/s eta 0:00:01
    3% |█                               | 40kB 1.2MB/s eta 0:00:02
---

[00:04:00] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:01] tidy error: /checkout/src/test/run-make/thumb-none-qemu/script.sh: missing trailing newline
[00:04:02] some tidy checks failed
[00:04:02] 
[00:04:02] 
[00:04:02] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:02] 
[00:04:02] 
[00:04:02] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:02] Build completed unsuccessfully in 0:00:45
[00:04:02] Build completed unsuccessfully in 0:00:45
[00:04:02] Makefile:79: recipe for target 'tidy' failed
[00:04:02] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0355ae30
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:010f10d0:start=1539647422757234333,finish=1539647422761291527,duration=4057194
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0ca5d94e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:119c1ee8
travis_time:start:119c1ee8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:066f65d0
$ dmesg | grep -i kill
