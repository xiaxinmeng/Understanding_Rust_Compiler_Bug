plain
travis_time:end:0971eb08:start=1547235021307466662,finish=1547235022270282226,duration=962815564
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
#####################################################                     74.5%
######################################################################## 100.0%
[00:01:48] extracting /checkout/obj/build/cache/2019-01-04/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:48]     Updating crates.io index
[00:01:58] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:58] Build completed unsuccessfully in 0:00:29
[00:01:58] make: *** [prepare] Error 1
[00:01:58] Makefile:71: recipe for target 'prepare' failed
[00:01:59] Command failed. Attempt 2/5:
[00:01:59] Command failed. Attempt 2/5:
[00:02:00] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:00] Build completed unsuccessfully in 0:00:00
[00:02:00] make: *** [prepare] Error 1
[00:02:00] Makefile:71: recipe for target 'prepare' failed
[00:02:02] Command failed. Attempt 3/5:
[00:02:02] Command failed. Attempt 3/5:
[00:02:02] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:02] Build completed unsuccessfully in 0:00:00
[00:02:02] Makefile:71: recipe for target 'prepare' failed
[00:02:02] make: *** [prepare] Error 1
[00:02:05] Command failed. Attempt 4/5:
[00:02:05] Command failed. Attempt 4/5:
[00:02:05] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:05] Build completed unsuccessfully in 0:00:00
[00:02:05] Makefile:71: recipe for target 'prepare' failed
[00:02:05] make: *** [prepare] Error 1
[00:02:09] Command failed. Attempt 5/5:
[00:02:09] Command failed. Attempt 5/5:
[00:02:10] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:02:10] Build completed unsuccessfully in 0:00:00
[00:02:10] Makefile:71: recipe for target 'prepare' failed
[00:02:10] The command has failed after 5 attempts.
[00:02:10] make: *** [prepare] Error 1
---
travis_time:end:0d462fec:start=1547235165061478606,finish=1547235165067477498,duration=5998892
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1844a2e0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2609a383
travis_time:start:2609a383
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1b2f37ea
$ dmesg | grep -i kill
