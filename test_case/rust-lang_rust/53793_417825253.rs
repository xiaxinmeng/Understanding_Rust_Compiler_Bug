plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/d7/d2/04d4fb0da66c7a8850477ff1fb8f9e80597147286e118e111f7d548cf1d7/awscli-1.16.6-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 16.8MB/s eta 0:00:01
    1% |▌                               | 20kB 1.8MB/s eta 0:00:01
    2% |▊                               | 30kB 2.1MB/s eta 0:00:01
    3% |█                               | 40kB 1.9MB/s eta 0:00:01
---

[00:04:27] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:27] tidy error: /checkout/src/test/ui/regions/regions-enum-not-wf.rs:31: line longer than 100 chars
[00:04:27] tidy error: /checkout/src/test/ui/regions/regions-enum-not-wf.rs:43: line longer than 100 chars
[00:04:27] tidy error: /checkout/src/test/ui/rfc-2093-infer-outlives/regions-enum-not-wf.rs:31: line longer than 100 chars
[00:04:27] tidy error: /checkout/src/test/ui/rfc-2093-infer-outlives/regions-enum-not-wf.rs:43: line longer than 100 chars
[00:04:28] some tidy checks failed
[00:04:28] 
[00:04:28] 
[00:04:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:28] 
[00:04:28] 
[00:04:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:28] Build completed unsuccessfully in 0:00:50
[00:04:28] Build completed unsuccessfully in 0:00:50
[00:04:28] Makefile:79: recipe for target 'tidy' failed
[00:04:28] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00b18c74
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:03af33f8:start=1535767004578960878,finish=1535767004586695589,duration=7734711
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0e8ec186
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06983afd
travis_time:start:06983afd
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2945a45f
$ dmesg | grep -i kill
