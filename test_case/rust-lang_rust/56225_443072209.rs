plain
travis_time:end:1154c89e:start=1543545696024028996,finish=1543545698790359536,duration=2766330540
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/aa/e5/ebd5896ad5ae353d23bea05ebb8edd3d49f1471784f6afa12a9cf11710de/awscli-1.16.67-py2.py3-none-any.whl (1.4MB)
    0% |▎                               | 10kB 17.7MB/s eta 0:00:01
    1% |▌                               | 20kB 1.8MB/s eta 0:00:01
    2% |▊                               | 30kB 2.1MB/s eta 0:00:01
    2% |█                               | 40kB 2.0MB/s eta 0:00:01
---
travis_time:start:tidy
tidy check
[00:03:10] * 568 error codes
[00:03:10] * highest error code: E0721
[00:03:11] tidy error: /checkout/src/test/ui/feature-gates/feature-gate-type_alias_enum_variants.rs:11: The file is already marked as gate test through its name, no need for a 'gate-test-type_alias_enum_variants' comment
[00:03:11] tidy error: /checkout/src/test/ui/feature-gates/feature-gate-type_alias_enum_variants.rs:11: The file is already marked as gate test through its name, no need for a 'gate-test-type_alias_enum_variants' comment
[00:03:11] some tidy checks failed
[00:03:11] 
[00:03:11] 
[00:03:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:11] 
[00:03:11] 
[00:03:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:11] Build completed unsuccessfully in 0:00:57
[00:03:11] Build completed unsuccessfully in 0:00:57
[00:03:11] Makefile:79: recipe for target 'tidy' failed
[00:03:11] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:103b94da
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Nov 30 02:44:59 UTC 2018
---
travis_time:end:055ae1ca:start=1543545899831816988,finish=1543545899836994697,duration=5177709
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:003ab5f4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0b0ebe64
travis_time:start:0b0ebe64
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:33dd2029
$ dmesg | grep -i kill
