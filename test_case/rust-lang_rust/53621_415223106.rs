plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/4e/cf/0c313db4b8e3b231447d3807657db4f5e7fad26d5eaeb294b3cfa1388a6c/awscli-1.15.84-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 10.4MB/s eta 0:00:01
    1% |▌                               | 20kB 1.9MB/s eta 0:00:01
    2% |▊                               | 30kB 2.2MB/s eta 0:00:01
    3% |█                               | 40kB 2.0MB/s eta 0:00:01
---

[00:04:40] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:40] tidy error: /checkout/src/librustc_target/spec/thumbv7a_pc_windows_msvc.rs:17: line longer than 100 chars
[00:04:40] tidy error: /checkout/src/librustc_target/spec/thumbv7a_pc_windows_msvc.rs:20: line longer than 100 chars
[00:04:40] tidy error: /checkout/src/librustc_target/spec/thumbv7a_pc_windows_msvc.rs:23: line longer than 100 chars
[00:04:40] tidy error: /checkout/src/libpanic_unwind/seh.rs:122: TODO is deprecated; use FIXME
[00:04:40] tidy error: /checkout/src/libstd/sys/windows/c.rs:836: line longer than 100 chars
[00:04:41] some tidy checks failed
[00:04:41] 
[00:04:41] 
[00:04:41] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:41] 
[00:04:41] 
[00:04:41] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:41] Build completed unsuccessfully in 0:00:50
[00:04:41] Build completed unsuccessfully in 0:00:50
[00:04:41] Makefile:79: recipe for target 'tidy' failed
[00:04:41] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0775b603
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0cc3f686:start=1534980842492236391,finish=1534980842499233517,duration=6997126
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05105ced
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:055f48bf
travis_time:start:055f48bf
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:003d29e4
$ dmesg | grep -i kill
