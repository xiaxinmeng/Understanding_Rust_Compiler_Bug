plain
travis_time:end:112e117c:start=1561146473104553888,finish=1561146560372898913,duration=87268345025
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:38:40] This PR updated 'src/tools/miri', verifying if status is 'test-pass'...
[01:38:40] 
[01:38:40] ⚠️ We detected that this PR updated 'miri', but its tests failed.
[01:38:40] 
[01:38:40] If you do intend to update 'miri', please check the error messages above and
[01:38:40] commit another update.
[01:38:40] 
[01:38:40] If you do NOT intend to update 'miri', please ensure you did not accidentally
[01:38:40] change the submodule at 'src/tools/miri'. You may ask your reviewer for the
[01:38:40] proper steps.
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:0eacd1ca
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Jun 21 21:28:09 UTC 2019
---
travis_time:end:0906c6e4:start=1561152491021947288,finish=1561152491027568600,duration=5621312
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06a24a40
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00ddb0ed
travis_time:start:00ddb0ed
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0c4c3ddd
$ dmesg | grep -i kill
