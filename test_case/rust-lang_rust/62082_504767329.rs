plain
travis_time:end:07e084c1:start=1561307337608009318,finish=1561307338481649161,duration=873639843
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
####################################################################      95.8%
######################################################################## 100.0%
[00:02:06] extracting /checkout/obj/build/cache/2019-05-23/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:06]     Updating crates.io index
[00:02:27] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:27] Build completed unsuccessfully in 0:00:46
[00:02:27] make: *** [prepare] Error 1
[00:02:27] Makefile:69: recipe for target 'prepare' failed
[00:02:28] Command failed. Attempt 2/5:
[00:02:28] Command failed. Attempt 2/5:
[00:02:28]     Updating crates.io index
[00:02:29] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:29] Build completed unsuccessfully in 0:00:00
[00:02:29] Makefile:69: recipe for target 'prepare' failed
[00:02:29] make: *** [prepare] Error 1
[00:02:31] Command failed. Attempt 3/5:
[00:02:31] Command failed. Attempt 3/5:
[00:02:31]     Updating crates.io index
[00:02:31] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:31] Build completed unsuccessfully in 0:00:00
[00:02:31] Makefile:69: recipe for target 'prepare' failed
[00:02:31] make: *** [prepare] Error 1
[00:02:34] Command failed. Attempt 4/5:
[00:02:34] Command failed. Attempt 4/5:
[00:02:34]     Updating crates.io index
[00:02:35] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:35] Build completed unsuccessfully in 0:00:00
[00:02:35] make: *** [prepare] Error 1
[00:02:35] Makefile:69: recipe for target 'prepare' failed
[00:02:39] Command failed. Attempt 5/5:
[00:02:39] Command failed. Attempt 5/5:
[00:02:39]     Updating crates.io index
[00:02:39] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:39] Build completed unsuccessfully in 0:00:00
[00:02:39] Makefile:69: recipe for target 'prepare' failed
[00:02:39] The command has failed after 5 attempts.
[00:02:39] make: *** [prepare] Error 1
---
travis_time:end:002a65dc:start=1561307510311420840,finish=1561307510316179069,duration=4758229
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:10441196
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:27e6c4e4
travis_time:start:27e6c4e4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1eba544b
$ dmesg | grep -i kill
