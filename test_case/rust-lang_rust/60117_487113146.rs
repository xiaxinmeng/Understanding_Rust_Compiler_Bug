plain
travis_time:end:061b4ab7:start=1556294457335962814,finish=1556294554914285015,duration=97578322201
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
###############################################                           66.5%
######################################################################## 100.0%
[00:01:28] extracting /checkout/obj/build/cache/2019-04-11/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:28]     Updating crates.io index
[00:01:53] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:53] Build completed unsuccessfully in 0:00:39
[00:01:53] make: *** [prepare] Error 1
[00:01:53] Makefile:69: recipe for target 'prepare' failed
[00:01:54] Command failed. Attempt 2/5:
[00:01:54] Command failed. Attempt 2/5:
[00:01:54]     Updating crates.io index
[00:01:54] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:54] Build completed unsuccessfully in 0:00:00
[00:01:54] make: *** [prepare] Error 1
[00:01:54] Makefile:69: recipe for target 'prepare' failed
[00:01:56] Command failed. Attempt 3/5:
[00:01:56] Command failed. Attempt 3/5:
[00:01:56]     Updating crates.io index
[00:01:57] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:57] Build completed unsuccessfully in 0:00:00
[00:01:57] Makefile:69: recipe for target 'prepare' failed
[00:01:57] make: *** [prepare] Error 1
[00:02:00] Command failed. Attempt 4/5:
[00:02:00] Command failed. Attempt 4/5:
[00:02:00]     Updating crates.io index
[00:02:01] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:01] Build completed unsuccessfully in 0:00:00
[00:02:01] make: *** [prepare] Error 1
[00:02:01] Makefile:69: recipe for target 'prepare' failed
[00:02:05] Command failed. Attempt 5/5:
[00:02:05] Command failed. Attempt 5/5:
[00:02:05]     Updating crates.io index
[00:02:06] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:06] Build completed unsuccessfully in 0:00:00
[00:02:06] Makefile:69: recipe for target 'prepare' failed
[00:02:06] make: *** [prepare] Error 1
[00:02:06] The command has failed after 5 attempts.
---
travis_time:end:0770ee54:start=1556294690676499040,finish=1556294690680933037,duration=4433997
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:064440b0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01e1feaa
travis_time:start:01e1feaa
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:022dbeec
$ dmesg | grep -i kill
