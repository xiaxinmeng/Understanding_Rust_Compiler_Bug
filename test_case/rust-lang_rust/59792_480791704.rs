plain
travis_time:end:10c77460:start=1554722388144955011,finish=1554722389028801325,duration=883846314
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:02:04] 
######################################################################## 100.0%
[00:02:05] extracting /checkout/obj/build/cache/2019-03-20/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:05]     Updating crates.io index
[00:02:20] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:20] Build completed unsuccessfully in 0:00:29
[00:02:20] Makefile:69: recipe for target 'prepare' failed
[00:02:20] make: *** [prepare] Error 1
[00:02:21] Command failed. Attempt 2/5:
[00:02:21] Command failed. Attempt 2/5:
[00:02:21]     Updating crates.io index
[00:02:22] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:22] Build completed unsuccessfully in 0:00:00
[00:02:22] make: *** [prepare] Error 1
[00:02:22] Makefile:69: recipe for target 'prepare' failed
[00:02:24] Command failed. Attempt 3/5:
[00:02:24] Command failed. Attempt 3/5:
[00:02:24]     Updating crates.io index
[00:02:25] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:25] Build completed unsuccessfully in 0:00:00
[00:02:25] Makefile:69: recipe for target 'prepare' failed
[00:02:25] make: *** [prepare] Error 1
[00:02:28] Command failed. Attempt 4/5:
[00:02:28] Command failed. Attempt 4/5:
[00:02:28]     Updating crates.io index
[00:02:28] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:28] Build completed unsuccessfully in 0:00:00
[00:02:28] Makefile:69: recipe for target 'prepare' failed
[00:02:28] make: *** [prepare] Error 1
[00:02:32] Command failed. Attempt 5/5:
[00:02:32] Command failed. Attempt 5/5:
[00:02:32]     Updating crates.io index
[00:02:33] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:33] Build completed unsuccessfully in 0:00:00
[00:02:33] make: *** [prepare] Error 1
[00:02:33] Makefile:69: recipe for target 'prepare' failed
[00:02:33] The command has failed after 5 attempts.
---
travis_time:end:0408bebc:start=1554722553656117906,finish=1554722553662580743,duration=6462837
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0513ca3b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:086308e8
travis_time:start:086308e8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:15f4b9c3
$ dmesg | grep -i kill
