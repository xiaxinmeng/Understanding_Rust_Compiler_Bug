plain
travis_time:end:0ac0be40:start=1558690454608116148,finish=1558690540971523343,duration=86363407195
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:01:09] 
######################################################################## 100.0%
[00:01:09] extracting /checkout/obj/build/cache/2019-04-11/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:09]     Updating crates.io index
[00:01:27] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:27] Build completed unsuccessfully in 0:00:31
[00:01:27] make: *** [prepare] Error 1
[00:01:27] Makefile:69: recipe for target 'prepare' failed
[00:01:28] Command failed. Attempt 2/5:
[00:01:28] Command failed. Attempt 2/5:
[00:01:29] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:29] Build completed unsuccessfully in 0:00:00
[00:01:29] make: *** [prepare] Error 1
[00:01:29] Makefile:69: recipe for target 'prepare' failed
[00:01:31] Command failed. Attempt 3/5:
[00:01:31] Command failed. Attempt 3/5:
[00:01:31] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:31] Build completed unsuccessfully in 0:00:00
[00:01:31] Makefile:69: recipe for target 'prepare' failed
[00:01:31] make: *** [prepare] Error 1
[00:01:34] Command failed. Attempt 4/5:
[00:01:34] Command failed. Attempt 4/5:
[00:01:35] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:35] Build completed unsuccessfully in 0:00:00
[00:01:35] make: *** [prepare] Error 1
[00:01:35] Makefile:69: recipe for target 'prepare' failed
[00:01:39] Command failed. Attempt 5/5:
[00:01:39] Command failed. Attempt 5/5:
[00:01:39] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:39] Build completed unsuccessfully in 0:00:00
[00:01:39] Makefile:69: recipe for target 'prepare' failed
[00:01:39] make: *** [prepare] Error 1
[00:01:39] The command has failed after 5 attempts.
---
travis_time:end:019a811a:start=1558690650708242223,finish=1558690650714068415,duration=5826192
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04cf2ea0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:08ab9816
travis_time:start:08ab9816
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1e673699
$ dmesg | grep -i kill
