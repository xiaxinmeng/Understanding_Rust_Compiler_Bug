plain
travis_time:end:106bdfd0:start=1558731282935856944,finish=1558731283705403293,duration=769546349
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
    99% |████████████████████████████████| 542kB 76.0MB/s eta 0:00:01
    100% |████████████████████████████████| 552kB 23.0MB/s 
Requirement already satisfied: PyYAML<=3.13,>=3.10 in /usr/lib/python2.7/dist-packages (from awscli) (3.11)
Collecting botocore==1.12.156 (from awscli)
  Downloading https://files.pythonhosted.org/packages/f5/aa/79fc47ccc3c7d0f36aafb9d85091d7d8a8f10d8ad24ccf3a89cf126b9f4e/botocore-1.12.156-py2.py3-none-any.whl (5.4MB)
    0% |▏                               | 20kB 26.0MB/s eta 0:00:01
    0% |▏                               | 30kB 28.2MB/s eta 0:00:01
    0% |▎                               | 40kB 31.5MB/s eta 0:00:01
    0% |▎                               | 51kB 34.2MB/s eta 0:00:01
---

[00:05:05] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:05:05] tidy error: /checkout/src/librustc_mir/interpret/place.rs:604: line longer than 100 chars
[00:05:10] some tidy checks failed
[00:05:10] 
[00:05:10] 
[00:05:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:10] 
[00:05:10] 
[00:05:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:10] Build completed unsuccessfully in 0:01:17
[00:05:10] Build completed unsuccessfully in 0:01:17
[00:05:10] Makefile:67: recipe for target 'tidy' failed
[00:05:10] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2155b69a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri May 24 21:00:04 UTC 2019
---
travis_time:end:0061dfa0:start=1558731605122301064,finish=1558731605126848312,duration=4547248
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0c2bc148
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:09f52671
travis_time:start:09f52671
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:18ea1db5
$ dmesg | grep -i kill
