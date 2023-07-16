plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/e6/8e/129966e5ae7e14a301fe58e81b7ce6dd762745518b6e3f987fb1d1df55a1/awscli-1.16.1-py2.py3-none-any.whl (1.3MB)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/53/fe/eb019c3d0fecc26680f025af10f3f026c33b61fda6c349de931e589ffa80/botocore-1.11.1-py2.py3-none-any.whl (4.6MB)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/db/c8/7dcf9dbcb22429512708fe3a547f8b6101c0d02137acbd892505aee57adf/colorama-0.3.9-py2.py3-none-any.whl
Collecting rsa<=3.5.0,>=3.1.2 (from awscli)
---
[01:50:00] Initialized empty Git repository in /checkout/obj/build/ct/xsv/.git/
[01:50:00] fatal: Could not parse object '66956b6bfd62d6ac767a6b6499c982eae20a2c9f'.
[01:50:21] fatal: unable to access 'https://github.com/BurntSushi/xsv/': Could not resolve host: github.com
[01:50:21] 
[01:50:21] thread 'main' panicked at 'assertion failed: status.success()', tools/cargotest/main.rs:128:13
[01:50:21] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:50:21] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/cargotest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/ct"
[01:50:21] expected success, got: exit code: 101
[01:50:21] 
[01:50:21] 
[01:50:21] 
[01:50:21] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/pretty src/test/run-pass/pretty src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/test/run-pass-fulldeps/pretty src/test/run-fail-fulldeps/pretty src/tools/cargo src/tools/cargotest
[01:50:21] Build completed unsuccessfully in 0:46:02
[01:50:21] make: *** [check-aux] Error 1
[01:50:21] Makefile:60: recipe for target 'check-aux' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:329a0004
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0702d596:start=1535179036669494495,finish=1535179036677628746,duration=8134251
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:293ffe24
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:017d918c
travis_time:start:017d918c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:08f5eec6
$ dmesg | grep -i kill
