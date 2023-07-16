plain
travis_time:end:21faf308:start=1545385030851718348,finish=1545385033194630567,duration=2342912219
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:01:15] 
######################################################################## 100.0%
[00:01:15] extracting /checkout/obj/build/cache/2018-12-09/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:15]     Updating crates.io index
[00:01:23] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:23] Build completed unsuccessfully in 0:00:24
[00:01:23] Makefile:81: recipe for target 'prepare' failed
[00:01:23] make: *** [prepare] Error 1
[00:01:24] Command failed. Attempt 2/5:
[00:01:24] Command failed. Attempt 2/5:
[00:01:24] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:24] Build completed unsuccessfully in 0:00:00
[00:01:24] make: *** [prepare] Error 1
[00:01:24] Makefile:81: recipe for target 'prepare' failed
[00:01:26] Command failed. Attempt 3/5:
[00:01:26] Command failed. Attempt 3/5:
[00:01:27] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:27] Build completed unsuccessfully in 0:00:00
[00:01:27] make: *** [prepare] Error 1
[00:01:27] Makefile:81: recipe for target 'prepare' failed
[00:01:30] Command failed. Attempt 4/5:
[00:01:30] Command failed. Attempt 4/5:
[00:01:30] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:30] Build completed unsuccessfully in 0:00:00
[00:01:30] Makefile:81: recipe for target 'prepare' failed
[00:01:30] make: *** [prepare] Error 1
[00:01:34] Command failed. Attempt 5/5:
[00:01:34] Command failed. Attempt 5/5:
[00:01:34] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:35] Build completed unsuccessfully in 0:00:00
[00:01:35] make: *** [prepare] Error 1
[00:01:35] Makefile:81: recipe for target 'prepare' failed
[00:01:35] The command has failed after 5 attempts.
---
travis_time:end:0b7c6932:start=1545385138000036742,finish=1545385138009461694,duration=9424952
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:24dde1b1
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:09caee9a
travis_time:start:09caee9a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:17b4ebe0
$ dmesg | grep -i kill
