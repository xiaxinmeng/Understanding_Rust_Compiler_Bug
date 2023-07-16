plain
travis_time:end:24cd1990:start=1550871537593166433,finish=1550871623879320610,duration=86286154177
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
    97% |████████████████���██████████████▍| 532kB 51.2MB/s eta 0:00:01
    99% |████████████████████████████████| 542kB 50.9MB/s eta 0:00:01
    100% |████████████████████████████████| 552kB 23.3MB/s 
Collecting botocore==1.12.101 (from awscli)
  Downloading https://files.pythonhosted.org/packages/d2/c1/bba75ff036a9363d32f597cddd0fbb6b6166a51f896631c4f57e56aacf65/botocore-1.12.101-py2.py3-none-any.whl (5.3MB)
    0% |▏                               | 20kB 23.2MB/s eta 0:00:01
    0% |▏                               | 30kB 26.5MB/s eta 0:00:01
    0% |▎                               | 40kB 27.5MB/s eta 0:00:01
    0% |▎                               | 51kB 28.8MB/s eta 0:00:01
---

[00:03:08] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:08] tidy error: /checkout/src/test/ui/consts/const-eval/ub-ref.rs:8: line longer than 100 chars
[00:03:09] some tidy checks failed
[00:03:09] 
[00:03:09] 
[00:03:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:09] 
[00:03:09] 
[00:03:09] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:09] Build completed unsuccessfully in 0:00:44
[00:03:09] Build completed unsuccessfully in 0:00:44
[00:03:09] make: *** [tidy] Error 1
[00:03:09] Makefile:68: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0a2235c0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Feb 22 21:43:43 UTC 2019
---
travis_time:end:167c4f66:start=1550871823880980254,finish=1550871823885050247,duration=4069993
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:12d2b0dc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:063b3ebc
travis_time:start:063b3ebc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:221dbc22
$ dmesg | grep -i kill
