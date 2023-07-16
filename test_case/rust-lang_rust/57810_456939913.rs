plain
travis_time:end:024047dc:start=1548272480050053777,finish=1548272560443446958,duration=80393393181
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:01:23] 
######################################################################## 100.0%
[00:01:23] extracting /checkout/obj/build/cache/2019-01-04/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:23]     Updating crates.io index
[00:01:33] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:33] Build completed unsuccessfully in 0:00:24
[00:01:33] Makefile:71: recipe for target 'prepare' failed
[00:01:33] make: *** [prepare] Error 1
[00:01:34] Command failed. Attempt 2/5:
[00:01:34] Command failed. Attempt 2/5:
[00:01:35]     Updating crates.io index
[00:01:35] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:35] Build completed unsuccessfully in 0:00:00
[00:01:35] make: *** [prepare] Error 1
[00:01:35] Makefile:71: recipe for target 'prepare' failed
[00:01:37] Command failed. Attempt 3/5:
[00:01:37] Command failed. Attempt 3/5:
[00:01:37]     Updating crates.io index
[00:01:38] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:38] Build completed unsuccessfully in 0:00:00
[00:01:38] make: *** [prepare] Error 1
[00:01:38] Makefile:71: recipe for target 'prepare' failed
[00:01:41] Command failed. Attempt 4/5:
[00:01:41] Command failed. Attempt 4/5:
[00:01:41]     Updating crates.io index
[00:01:41] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:41] Build completed unsuccessfully in 0:00:00
[00:01:42] Makefile:71: recipe for target 'prepare' failed
[00:01:42] make: *** [prepare] Error 1
[00:01:46] Command failed. Attempt 5/5:
[00:01:46] Command failed. Attempt 5/5:
[00:01:46]     Updating crates.io index
[00:01:46] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:46] Build completed unsuccessfully in 0:00:00
[00:01:46] Makefile:71: recipe for target 'prepare' failed
[00:01:46] make: *** [prepare] Error 1
[00:01:46] The command has failed after 5 attempts.
---
travis_time:end:044804f4:start=1548272678126877160,finish=1548272678132976402,duration=6099242
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:11492e00
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:051e7387
travis_time:start:051e7387
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:06b1d8ce
$ dmesg | grep -i kill
