plain
travis_time:end:04c002f6:start=1548940429167378583,finish=1548940430284462371,duration=1117083788
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
##############################################                            65.0%
######################################################################## 100.0%
[00:01:55] extracting /checkout/obj/build/cache/2019-01-18/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:55]     Updating crates.io index
[00:02:05] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:05] Build completed unsuccessfully in 0:00:30
[00:02:05] Makefile:70: recipe for target 'prepare' failed
[00:02:05] make: *** [prepare] Error 1
[00:02:06] Command failed. Attempt 2/5:
[00:02:06] Command failed. Attempt 2/5:
[00:02:06]     Updating crates.io index
[00:02:07] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:07] Build completed unsuccessfully in 0:00:00
[00:02:07] make: *** [prepare] Error 1
[00:02:07] Makefile:70: recipe for target 'prepare' failed
[00:02:09] Command failed. Attempt 3/5:
[00:02:09] Command failed. Attempt 3/5:
[00:02:09]     Updating crates.io index
[00:02:09] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:09] Build completed unsuccessfully in 0:00:00
[00:02:09] make: *** [prepare] Error 1
[00:02:09] Makefile:70: recipe for target 'prepare' failed
[00:02:12] Command failed. Attempt 4/5:
[00:02:12] Command failed. Attempt 4/5:
[00:02:12]     Updating crates.io index
[00:02:13] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:13] Build completed unsuccessfully in 0:00:00
[00:02:13] make: *** [prepare] Error 1
[00:02:13] Makefile:70: recipe for target 'prepare' failed
[00:02:17] Command failed. Attempt 5/5:
[00:02:17] Command failed. Attempt 5/5:
[00:02:17]     Updating crates.io index
[00:02:17] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:17] Build completed unsuccessfully in 0:00:00
[00:02:17] make: *** [prepare] Error 1
[00:02:17] Makefile:70: recipe for target 'prepare' failed
[00:02:17] The command has failed after 5 attempts.
---
travis_time:end:30d4da9d:start=1548940581342948184,finish=1548940581348176412,duration=5228228
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1042e934
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04b44ca3
travis_time:start:04b44ca3
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0d7a0ffe
$ dmesg | grep -i kill
