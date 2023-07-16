plain
travis_time:end:1e8d3ef2:start=1558732239545150917,finish=1558732240309206454,duration=764055537
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
    99% |████████████████████████████████| 542kB 41.7MB/s eta 0:00:01
    100% |████████████████████████████████| 552kB 19.2MB/s 
Requirement already satisfied: PyYAML<=3.13,>=3.10 in /usr/lib/python2.7/dist-packages (from awscli) (3.11)
Collecting botocore==1.12.156 (from awscli)
  Downloading https://files.pythonhosted.org/packages/f5/aa/79fc47ccc3c7d0f36aafb9d85091d7d8a8f10d8ad24ccf3a89cf126b9f4e/botocore-1.12.156-py2.py3-none-any.whl (5.4MB)
    0% |▏                               | 20kB 28.9MB/s eta 0:00:01
    0% |▏                               | 30kB 33.5MB/s eta 0:00:01
    0% |▎                               | 40kB 34.8MB/s eta 0:00:01
    0% |▎                               | 51kB 35.5MB/s eta 0:00:01
---
tidy check
[00:04:25] * 574 error codes
[00:04:25] * highest error code: E0729
[00:04:27] * 255 features
[00:04:30] Stray file with UI testing output: "/checkout/src/test/ui/no-args-non-move-async-closure.stderr"
[00:04:30] some tidy checks failed
[00:04:30] 
[00:04:30] 
[00:04:30] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:30] 
[00:04:30] 
[00:04:30] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:30] Build completed unsuccessfully in 0:01:11
[00:04:30] Build completed unsuccessfully in 0:01:11
[00:04:30] Makefile:67: recipe for target 'tidy' failed
[00:04:30] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:13135845
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri May 24 21:15:21 UTC 2019
---
travis_time:end:110686cc:start=1558732522106005623,finish=1558732522110867268,duration=4861645
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:07a437f0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02a4f4eb
travis_time:start:02a4f4eb
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:08854cbf
$ dmesg | grep -i kill
