plain
travis_time:end:201a0023:start=1548654816817273338,finish=1548654817768772249,duration=951498911
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
#######################################################                   77.4%
######################################################################## 100.0%
[00:02:15] extracting /checkout/obj/build/cache/2019-01-18/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:15]     Updating crates.io index
[00:02:26] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:26] Build completed unsuccessfully in 0:00:32
[00:02:26] make: *** [prepare] Error 1
[00:02:26] Makefile:70: recipe for target 'prepare' failed
[00:02:27] Command failed. Attempt 2/5:
[00:02:27] Command failed. Attempt 2/5:
[00:02:27]     Updating crates.io index
[00:02:27] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:27] Build completed unsuccessfully in 0:00:00
[00:02:27] make: *** [prepare] Error 1
[00:02:27] Makefile:70: recipe for target 'prepare' failed
[00:02:29] Command failed. Attempt 3/5:
[00:02:29] Command failed. Attempt 3/5:
[00:02:29]     Updating crates.io index
[00:02:30] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:30] Build completed unsuccessfully in 0:00:00
[00:02:30] Makefile:70: recipe for target 'prepare' failed
[00:02:30] make: *** [prepare] Error 1
[00:02:33] Command failed. Attempt 4/5:
[00:02:33] Command failed. Attempt 4/5:
[00:02:33]     Updating crates.io index
[00:02:33] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:33] Build completed unsuccessfully in 0:00:00
[00:02:33] make: *** [prepare] Error 1
[00:02:33] Makefile:70: recipe for target 'prepare' failed
[00:02:37] Command failed. Attempt 5/5:
[00:02:37] Command failed. Attempt 5/5:
[00:02:37]     Updating crates.io index
[00:02:38] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:38] Build completed unsuccessfully in 0:00:00
[00:02:38] Makefile:70: recipe for target 'prepare' failed
[00:02:38] make: *** [prepare] Error 1
[00:02:38] The command has failed after 5 attempts.
---
travis_time:end:2170f8ba:start=1548654990355502093,finish=1548654990361212371,duration=5710278
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:120f30e2
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:23faba66
travis_time:start:23faba66
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:010bbd90
$ dmesg | grep -i kill
