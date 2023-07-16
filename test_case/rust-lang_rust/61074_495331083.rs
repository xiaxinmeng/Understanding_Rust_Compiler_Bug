plain
travis_time:end:0f2a6492:start=1558629281290757759,finish=1558629282059453776,duration=768696017
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:39:34] Verifying status of rustfmt...
[01:39:34] Verifying status of clippy-driver...
[01:39:34] This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
[01:39:34] 
[01:39:34] ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
[01:39:34] 
[01:39:34] If you do intend to update 'clippy-driver', please check the error messages above and
[01:39:34] commit another update.
[01:39:34] 
[01:39:34] If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
[01:39:34] change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
[01:39:34] proper steps.
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:00ad480e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu May 23 18:14:27 UTC 2019
---
travis_time:end:0356f3b2:start=1558635268823562384,finish=1558635268828755642,duration=5193258
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00de3f04
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1288ce30
travis_time:start:1288ce30
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1065513c
$ dmesg | grep -i kill
