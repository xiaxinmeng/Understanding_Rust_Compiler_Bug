plain
travis_time:end:05dc0cea:start=1561462561497019956,finish=1561462665896871152,duration=104399851196
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
#####################################################                     74.7%
######################################################################## 100.0%
[00:01:09] extracting /checkout/obj/build/cache/2019-05-23/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:09]     Updating crates.io index
[00:01:30] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:30] Build completed unsuccessfully in 0:00:34
[00:01:30] Makefile:69: recipe for target 'prepare' failed
[00:01:30] make: *** [prepare] Error 1
[00:01:31] Command failed. Attempt 2/5:
[00:01:31] Command failed. Attempt 2/5:
[00:01:31]     Updating crates.io index
[00:01:31] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:31] Build completed unsuccessfully in 0:00:00
[00:01:31] make: *** [prepare] Error 1
[00:01:31] Makefile:69: recipe for target 'prepare' failed
[00:01:33] Command failed. Attempt 3/5:
[00:01:33] Command failed. Attempt 3/5:
[00:01:33]     Updating crates.io index
[00:01:34] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:34] Build completed unsuccessfully in 0:00:00
[00:01:34] Makefile:69: recipe for target 'prepare' failed
[00:01:34] make: *** [prepare] Error 1
[00:01:37] Command failed. Attempt 4/5:
[00:01:37] Command failed. Attempt 4/5:
[00:01:37]     Updating crates.io index
[00:01:37] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:37] Build completed unsuccessfully in 0:00:00
[00:01:37] make: *** [prepare] Error 1
[00:01:37] Makefile:69: recipe for target 'prepare' failed
[00:01:41] Command failed. Attempt 5/5:
[00:01:41] Command failed. Attempt 5/5:
[00:01:42]     Updating crates.io index
[00:01:42] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:42] Build completed unsuccessfully in 0:00:00
[00:01:42] make: *** [prepare] Error 1
[00:01:42] Makefile:69: recipe for target 'prepare' failed
[00:01:42] The command has failed after 5 attempts.
---
travis_time:end:26f5c81e:start=1561462778747324620,finish=1561462778753870501,duration=6545881
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:08a5e2f6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0cfe76a3
travis_time:start:0cfe76a3
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:10283460
$ dmesg | grep -i kill
