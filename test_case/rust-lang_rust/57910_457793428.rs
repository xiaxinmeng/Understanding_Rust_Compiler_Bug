plain
travis_time:end:0098198d:start=1548467548149189141,finish=1548467549329551371,duration=1180362230
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
    97% |███████████████████████████████▍| 532kB 67.4MB/s eta 0:00:01
    99% |██████���█████████████████████████| 542kB 66.0MB/s eta 0:00:01
    100% |████████████████████████████████| 552kB 29.0MB/s 
Collecting botocore==1.12.86 (from awscli)
  Downloading https://files.pythonhosted.org/packages/d7/af/fd9c0f1f0fdc03d3367a56f35093f8b1020ba1a97ead9fa580156895944b/botocore-1.12.86-py2.py3-none-any.whl (5.2MB)
    0% |▏                               | 20kB 28.4MB/s eta 0:00:01
    0% |▏                               | 30kB 31.5MB/s eta 0:00:01
    0% |▎                               | 40kB 30.6MB/s eta 0:00:01
    0% |▎                               | 51kB 32.5MB/s eta 0:00:01
---
tidy check
[00:04:03] * 564 error codes
[00:04:03] * highest error code: E0721
[00:04:03] * 246 features
[00:04:04] Stray file with UI testing output: "/checkout/src/test/ui/parser/regions-out-of-scope-slice.stderr"
[00:04:04] some tidy checks failed
[00:04:04] 
[00:04:04] 
[00:04:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:04] 
[00:04:04] 
[00:04:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:04] Build completed unsuccessfully in 0:00:47
[00:04:04] Build completed unsuccessfully in 0:00:47
[00:04:04] Makefile:68: recipe for target 'tidy' failed
[00:04:04] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03e67f76
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Jan 26 01:56:44 UTC 2019
---
travis_time:end:01a0a36b:start=1548467805170177541,finish=1548467805175002110,duration=4824569
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03031c23
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:25b1d050
travis_time:start:25b1d050
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1e8190fc
$ dmesg | grep -i kill
