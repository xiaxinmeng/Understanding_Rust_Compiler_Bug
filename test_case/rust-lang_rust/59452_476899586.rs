plain
travis_time:end:00207538:start=1553643969636459964,finish=1553644062502086956,duration=92865626992
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
#####################################################################     96.2%
######################################################################## 100.0%
[00:01:39] extracting /checkout/obj/build/cache/2019-03-20/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:39]     Updating crates.io index
[00:01:54] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:54] Build completed unsuccessfully in 0:00:40
[00:01:54] Makefile:69: recipe for target 'prepare' failed
[00:01:54] make: *** [prepare] Error 1
[00:01:55] Command failed. Attempt 2/5:
[00:01:55] Command failed. Attempt 2/5:
[00:01:55]     Updating crates.io index
[00:01:56] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:56] Build completed unsuccessfully in 0:00:00
[00:01:56] make: *** [prepare] Error 1
[00:01:56] Makefile:69: recipe for target 'prepare' failed
[00:01:58] Command failed. Attempt 3/5:
[00:01:58] Command failed. Attempt 3/5:
[00:01:58]     Updating crates.io index
[00:01:58] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:58] Build completed unsuccessfully in 0:00:00
[00:01:58] Makefile:69: recipe for target 'prepare' failed
[00:01:58] make: *** [prepare] Error 1
[00:02:01] Command failed. Attempt 4/5:
[00:02:01] Command failed. Attempt 4/5:
[00:02:02]     Updating crates.io index
[00:02:02] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:02] Build completed unsuccessfully in 0:00:00
[00:02:02] Makefile:69: recipe for target 'prepare' failed
[00:02:02] make: *** [prepare] Error 1
[00:02:06] Command failed. Attempt 5/5:
[00:02:06] Command failed. Attempt 5/5:
[00:02:06]     Updating crates.io index
[00:02:07] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:07] Build completed unsuccessfully in 0:00:00
[00:02:07] Makefile:69: recipe for target 'prepare' failed
[00:02:07] make: *** [prepare] Error 1
[00:02:07] The command has failed after 5 attempts.
---
travis_time:end:1644d217:start=1553644201017268099,finish=1553644201024008180,duration=6740081
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:24ac9d00
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01cbc54a
travis_time:start:01cbc54a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02b40a96
$ dmesg | grep -i kill
