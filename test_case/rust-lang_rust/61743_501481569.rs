plain
travis_time:end:09752ce4:start=1560373149209283791,finish=1560373150057726687,duration=848442896
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:41:28] This PR updated 'src/tools/miri', verifying if status is 'test-pass'...
[01:41:28] 
[01:41:28] ⚠️ We detected that this PR updated 'miri', but its tests failed.
[01:41:28] 
[01:41:28] If you do intend to update 'miri', please check the error messages above and
[01:41:28] commit another update.
[01:41:28] 
[01:41:28] If you do NOT intend to update 'miri', please ensure you did not accidentally
[01:41:28] change the submodule at 'src/tools/miri'. You may ask your reviewer for the
[01:41:28] proper steps.
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:02a2db40
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Jun 12 22:40:48 UTC 2019
---
travis_time:end:009943f5:start=1560379250254729541,finish=1560379250259709260,duration=4979719
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:274684d4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:003d45b3
travis_time:start:003d45b3
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0c6f7b18
$ dmesg | grep -i kill
