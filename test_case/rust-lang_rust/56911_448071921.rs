plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/63/dc/c8bfd1bd77113c033161ce31730510d1c479cf9bcc8e99edf3c906f30cce/awscli-1.16.77-py2.py3-none-any.whl (1.4MB)
    0% |▎                               | 10kB 29.0MB/s eta 0:00:01
    1% |▌                               | 20kB 2.1MB/s eta 0:00:01
    2% |▊                               | 30kB 3.1MB/s eta 0:00:01
    2% |█                               | 40kB 2.0MB/s eta 0:00:01
---
[01:23:47]    Compiling rls v1.31.7 (/checkout/src/tools/rls)
[01:23:48] error[E0599]: no method named `is_whitespace` found for type `&&str` in the current scope
[01:23:48]     --> tools/rls/tests/tests.rs:1306:28
[01:23:48]      |
[01:23:48] 1306 |     new_text.retain(|c| !c.is_whitespace());
[01:23:48]      |
[01:23:48]      = help: did you mean `split_whitespace`?
[01:23:48] 
[01:23:48] error: aborting due to previous error
---
[01:32:20] Verifying status of rust-by-example...
[01:32:20] Verifying status of rls...
[01:32:20] This PR updated 'src/tools/rls', verifying if status is 'test-pass'...
[01:32:20] 
[01:32:20] ⚠️ We detected that this PR updated 'rls', but its tests failed.
[01:32:20] 
[01:32:20] If you do intend to update 'rls', please check the error messages above and
[01:32:20] commit another update.
[01:32:20] 
[01:32:20] If you do NOT intend to update 'rls', please ensure you did not accidentally
[01:32:20] change the submodule at 'src/tools/rls'. You may ask your reviewer for the
[01:32:20] proper steps.
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:3232d660
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Dec 18 02:17:31 UTC 2018
---
travis_time:end:062ad1d0:start=1545099453523094964,finish=1545099453535281239,duration=12186275
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2254ad90
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1487e02a
travis_time:start:1487e02a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:3482c721
$ dmesg | grep -i kill
